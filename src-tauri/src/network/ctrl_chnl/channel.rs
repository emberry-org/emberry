use super::room_creation::hole_punch;
use std::{
  io::{self, Error, ErrorKind},
  sync::atomic::Ordering,
};

use crate::{
  data::{
    config,
    sqlite::{
      exec, try_exec,
      user::{try_get, upsert},
    },
    IdentifiedUserInfo, UserIdentifier, UserInfo,
  },
  network::ctrl_chnl::state,
};

pub use super::messages::EmberryMessage;
pub use super::state::RwOption;
pub use super::state::{RhizomeConnection, State};
use log::trace;
use serde_json::json;
use smoke::messages::{EmbMessage, RoomId};
use smoke::{
  messages::RhizMessage::{self, *},
  User,
};
use tauri::{api::notification::Notification, AppHandle, Window};
use tokio::{
  io::BufReader,
  net::TcpStream,
};
use tokio::{
  select,
  sync::mpsc::Receiver,
};
use tokio_rustls::{client::TlsStream};

use super::{Networking, RRState};

pub struct ControlChannel<'a> {
  pub window: &'a Window,
  pub app: &'a AppHandle,
  pub rx: Receiver<EmberryMessage>,
  pub tls: BufReader<TlsStream<TcpStream>>,
  pub net: tauri::State<'a, Networking>,
  pub rc: &'a tauri::State<'a, RhizomeConnection>,
}

impl<'a> ControlChannel<'a> {
  pub async fn spin(mut self) -> tauri::Result<()> {
    let mut buf = vec![];
    loop {
      select! {
        Some(msg) = self.rx.recv() => {
          trace!("crtl send: {:?}", &msg);
          match msg {
              EmberryMessage::Direct(msg) => msg.send_with(&mut self.tls).await?,
              EmberryMessage::Close() => return Ok(()),
          }
        }
        msg = RhizMessage::recv_with(&mut self.tls, &mut buf) => Self::handle_rhiz_msg(msg, self.window, self.app, &self.net, self.rc).await?
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
                .title(format!(
                  "{} wants to connect to you",
                  ident_info.info.username
                ))
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
          let priority =
            unsafe { &config::PEM_DATA.as_ref().unwrap_unchecked().0 .0 } < &usr.cert_data;
          let msg = EmbMessage::Accept(priority);
          state::send(rc, msg).await?;
        }
      }
      AcceptedRoom(id, usr) => {
        //                    we can unsafe unwrap here because we know that PEM_DATA is not None because the receive loop
        //                    only starts if PEM_DATA is Some()
        let priority =
          unsafe { &config::PEM_DATA.as_ref().unwrap_unchecked().0 .0 } < &usr.cert_data;
        Self::try_holepunch(window.clone(), app_handle, net.clone(), id, usr, priority).await?
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
}
