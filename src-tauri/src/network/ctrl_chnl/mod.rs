mod certs;
mod messages;
pub mod requests;
pub mod responses;
mod state;

use std::{
  io::{self, Error, ErrorKind},
  sync::Arc,
};

use crate::network::hole_punch;

pub use self::state::RwOption;
use log::trace;
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
use tauri::Window;
use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpStream,
};
use tokio::{
  select,
  sync::mpsc::{self, Receiver},
};
use tokio_rustls::{client::TlsStream, TlsConnector};

use super::{Networking, RRState, ENV};

#[tauri::command(async)]
pub async fn connect(
  window: tauri::Window,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  if rc.read().await.is_some() {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Already connected to the server",
    )));
  }

  let mut root_store = RootCertStore::empty();

  let cert = certs::craft();
  root_store.add(&cert).unwrap(); // unwrap is safe here as invalid certificates wont be returned from load

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
  if plaintext != "rhizome v0.2.0\n" {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Server did greet with rhizome signature",
    )));
  }

  tls.write_all(dotenv!("PUBLIC_KEY").as_bytes()).await?;

  let (tx, rx) = mpsc::channel::<EmberryMessage>(25);

  let conn = State { channel: tx };
  rc.write().await.replace(conn);

  let res = run_channel_result(&window, rx, tls, net, &rc).await;

  *rc.write().await = None;

  res
}

async fn run_channel_result<'a>(
  window: &tauri::Window,
  mut rx: Receiver<EmberryMessage>,
  mut tls: BufReader<TlsStream<TcpStream>>,
  net: tauri::State<'_, Networking>,
  rc: &tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let mut buf = vec![];
  loop {
    select! {
      Some(msg) = rx.recv() => {
        match msg {
            EmberryMessage::Direct(msg) => msg.send_with(&mut tls).await?,
            EmberryMessage::Close() => return Ok(()),
        }
      }
      msg = RhizMessage::recv_with(&mut tls, &mut buf) => handle_rhiz_msg(msg, window, &net, rc).await?
    }
  }
}

async fn handle_rhiz_msg(
  msg: Result<RhizMessage, io::Error>,
  window: &Window,
  net: &tauri::State<'_, Networking>,
  rc: &tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  trace!("ctrl recv: {:?}", msg);
  match msg? {
    Shutdown() => return Ok(()),
    HasRoute(usr) => {
      let pending = net.pending.lock().unwrap().contains_key(&usr);
      window
        .emit("has-route", json!({ "pending": pending, "usr": usr, }))
        .expect("Failed to emit HasRoute")
    }
    NoRoute(usr) => {
      // might want to remove the ".remove(&usr)" when trying to auto reconnect...
      let pending = net.pending.lock().unwrap().remove(&usr);
      window
        .emit(
          "no-route",
          json!({ "pending": pending.is_some(), "usr": usr, }),
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
          window
            .emit("wants-room", usr)
            .expect("Failed to emit WantsRoom event");
        } else {
          // Here we get a WantsRoom while we already want a room with them (they were unaware when they made their request)
          // In this situation the user with the higher value as pub key rejects the request
          // the client with the lower value pub key auto accepts
          // this is done to remove the dublicate request
          guard.insert(usr, super::RRState::Agreement);
        }
      }

      if !none {
        // this is the same case where guard.insert(Agreement) happens just outside scope because we want to drop guard before await
        let msg = EmbMessage::Accept(ENV.public_key.as_bytes() < usr.key.as_slice());
        state::send(rc, msg).await?;
      }
    }
    AcceptedRoom(id, usr) => try_holepunch(window.clone(), net.clone(), id, usr).await?,
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
  net_state: tauri::State<'_, Networking>,
  room_id: Option<RoomId>,
  usr: User,
) -> tauri::Result<()> {
  if let Some(room_id) = room_id {
    if net_state.pending.lock().unwrap().remove(&usr).is_some() {
      // only hole punch if there is a connection pending
      hole_punch(window, net_state, room_id).await?;
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
