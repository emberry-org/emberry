mod certs;
mod messages;
pub mod requests;
pub mod responses;
mod room_creation;
mod state;

use room_creation::hole_punch;
use std::{
  io::{self, Error, ErrorKind},
  sync::{atomic::Ordering, Arc},
  time::Instant,
};

use crate::data::{
  config,
  sqlite::{
    exec, try_exec,
    user::{try_get, upsert},
  },
  IdentifiedUserInfo, UserIdentifier, UserInfo,
};

pub use self::state::RwOption;
use log::{error, trace};
pub use messages::EmberryMessage;
use rustls::{ClientConfig, RootCertStore, ServerName};
use serde_json::json;
use smoke::messages::{EmbMessage, RoomId};
use smoke::{
  messages::RhizMessage::{self, *},
  User,
};
pub use state::RhizomeConnection;
pub use state::State;
use tauri::{api::notification::Notification, Window};
use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpStream,
};
use tokio::{
  select,
  sync::mpsc::{self, Receiver},
};
use tokio_rustls::{client::TlsStream, TlsConnector};

use super::{Networking, RRState};

#[tauri::command(async)]
pub async fn connect(
  window: tauri::Window,
  app_handle: tauri::AppHandle,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let start = Instant::now();
  if rc.read().await.is_some() {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Already connected to the server",
    )));
  }

  let server_cert = certs::craft();
  let (client_cert, _) = match config::PEM_DATA.as_ref() {
    Some(data) => data,
    None => {
      return Err(tauri::Error::Io(io::Error::new(
        io::ErrorKind::Unsupported,
        "Identity needed to connect with rhizome",
      )))
    }
  };

  let mut root_store = RootCertStore::empty();
  root_store
    .add(&server_cert)
    .map_err(|err| tauri::Error::Io(io::Error::new(ErrorKind::InvalidData, err)))?;

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();

  let server_name = get_server_name();
  let conn = TlsConnector::from(Arc::new(config));
  let sock = TcpStream::connect(dotenv!("CONTROL_ADDRESS")).await?;
  let mut tls = BufReader::new(conn.connect(server_name, sock).await?);

  let mut plaintext = String::new();
  tls.read_line(&mut plaintext).await?;
  if plaintext != "rhizome v0.3.0\n" {
    error!("invalid rhizome version");
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Server did greet with rhizome signature",
    )));
  }
  window
    .emit("rz-con", start.elapsed().as_millis() as u64)
    .expect("Failed to emit event");

  let cobs_cert = match postcard::to_vec_cobs::<Vec<u8>, 1024>(&client_cert.0) {
    Ok(cobs) => cobs,
    Err(err) => {
      error!("Error serializing USER_CERT in 1024 bytes: {}", err);
      return Err(tauri::Error::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "cannot serialize USER_CERT",
      )));
    }
  };
  tls.write_all(&cobs_cert).await?;

  let (tx, rx) = mpsc::channel::<EmberryMessage>(25);

  let conn = State { channel: tx };
  rc.write().await.replace(conn);

  let res = run_channel_result(&window, &app_handle, rx, tls, net, &rc).await;

  *rc.write().await = None;

  window
    .emit("rz-dc", start.elapsed().as_millis() as u64)
    .expect("Failed to emit event");

  res
}

async fn run_channel_result<'a>(
  window: &tauri::Window,
  app_handle: &tauri::AppHandle,
  mut rx: Receiver<EmberryMessage>,
  mut tls: BufReader<TlsStream<TcpStream>>,
  net: tauri::State<'_, Networking>,
  rc: &tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let mut buf = vec![];
  loop {
    select! {
      Some(msg) = rx.recv() => {
        trace!("crtl send: {:?}", &msg);
        match msg {
            EmberryMessage::Direct(msg) => msg.send_with(&mut tls).await?,
            EmberryMessage::Close() => return Ok(()),
        }
      }
      msg = RhizMessage::recv_with(&mut tls, &mut buf) => handle_rhiz_msg(msg, window, app_handle, &net, rc).await?
    }
  }
}

async fn handle_rhiz_msg(
  msg: Result<RhizMessage, io::Error>,
  window: &Window,
  app_handle: &tauri::AppHandle,
  net: &tauri::State<'_, Networking>,
  rc: &tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  trace!("ctrl recv: {:?}", msg);
  match msg? {
    Shutdown() => return Ok(()),
    HasRoute(usr) => {
      let pending = net.pending.lock().unwrap().contains_key(&usr);
      window
        .emit(
          "has-route",
          json!({ "pending": pending, "usr": UserIdentifier::from(&usr).bs58, }),
        )
        .expect("Failed to emit HasRoute")
    }
    NoRoute(usr) => {
      // might want to remove the ".remove(&usr)" when trying to auto reconnect...
      let pending = net.pending.lock().unwrap().remove(&usr);
      window
        .emit(
          "no-route",
          json!({ "pending": pending.is_some(), "usr": UserIdentifier::from(&usr).bs58, }),
        )
        .expect("Failed to emit NoRoute")
    }
    WantsRoom(usr) => {
      // only option here is None or RRState::RemoteUnaware
      let none;
      {
        let mut guard = net.pending.lock().unwrap();
        none = guard.get(&usr).is_none();
        if none {
          let ident = UserIdentifier::from(&usr);
          let info = exec(try_get, &ident);
          let ident_info = match info {
            Ok(info) => IdentifiedUserInfo {
              identifier: ident,
              info,
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => {
              let ident_info = IdentifiedUserInfo {
                info: UserInfo {
                  username: ident.bs58.to_string(),
                  relation: crate::data::UserRelation::Stranger,
                },
                identifier: ident.as_ref(),
              };
              let new_user_event = |ident_info: &IdentifiedUserInfo| {
                window
                  .emit("new-user", &ident_info.info.username)
                  .expect("Failed to emit new-user event")
              };
              try_exec(upsert, (&ident_info, new_user_event))?;

              ident_info
            }
            Err(err) => {
              log::error!("SQLite access error : '{}'", err);
              return Err(tauri::Error::Io(io::Error::new(
                ErrorKind::Other,
                "SQLite error",
              )));
            }
          };

          window
            .emit("wants-room", &ident_info)
            .expect("Failed to emit WantsRoom event");

          /* Create a new notification for the message */
          if !crate::FOCUS.load(Ordering::SeqCst) {
            Notification::new(&app_handle.config().tauri.bundle.identifier)
              .title(format!("{} wants to connect to you", ident_info.info.username))
              .show()
              .expect("Failed to send desktop notification");
          }
        } else {
          // Here we get a WantsRoom while we already want a room with them (they were unaware when they made their request)
          // In this situation the user with the higher value as pub key rejects the request
          // the client with the lower value pub key auto accepts
          // this is done to remove the dublicate request
          guard.insert(usr.clone(), super::RRState::Agreement);
        }
      }

      if !none {
        // this is the same case where guard.insert(Agreement) happens just outside scope because we want to drop guard before await
        //                    we can unsafe unwrap here because we know that PEM_DATA is not None because the receive loop
        //                    only starts if PEM_DATA is Some()
        let priority = unsafe { &config::PEM_DATA.as_ref().unwrap_unchecked().0.0 } < &usr.cert_data;
        let msg = EmbMessage::Accept(
          priority,
        );
        state::send(rc, msg).await?;
      }
    }
    AcceptedRoom(id, usr) => {
      //                    we can unsafe unwrap here because we know that PEM_DATA is not None because the receive loop
      //                    only starts if PEM_DATA is Some()
      let priority = unsafe { &config::PEM_DATA.as_ref().unwrap_unchecked().0.0 } < &usr.cert_data;
      try_holepunch(window.clone(), app_handle, net.clone(), id, usr, priority).await?
    }
    ServerError(err) => {
      return Err(tauri::Error::Io(io::Error::new(
        ErrorKind::Other,
        format!("Rhizome internal error: {}", err),
      )))
    }
  };

  Ok(())
}

async fn try_holepunch(
  window: tauri::Window,
  app_handle: &tauri::AppHandle,
  net_state: tauri::State<'_, Networking>,
  room_id: Option<RoomId>,
  usr: User,
  priority: bool,
) -> tauri::Result<()> {
  if let Some(room_id) = room_id {
    if net_state.pending.lock().unwrap().remove(&usr).is_some() {
      // only hole punch if there is a connection pending
      hole_punch(window, app_handle, net_state, room_id, usr, priority).await?;
    } else {
      // This is rather weak protection as a compromized rhizome server could still just send a different room id with a valid user
      // Room id procedure is subject to change in the future. (plan is to use cryptographic signatures to mitigated unwanted ip leak)
      return Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "Rhizome just sent a malicious room opening packet (this should not happen)",
      )));
    }
  } else {
    let mut guard = net_state.pending.lock().unwrap();
    if let Some(kv) = guard.get_key_value(&usr) {
      match kv.1 {
        RRState::Agreement => return Ok(()), // We got the edgecase of colliding requests, throw away this one
        RRState::Pending => {
          guard.remove(&usr); // This case is a normal rejection
        }
      }
    }
  }

  Ok(())
}

#[inline]
fn get_server_name() -> ServerName {
  // unwrap is ok here as this is tested with tests on build time
  dotenv!("SERVER_DOMAIN").try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::get_server_name;

  #[test]
  fn server_name_creation() {
    get_server_name();
  }
}
