use super::room_creation::try_holepunch;
use std::{
  io::{self, ErrorKind},
  sync::atomic::Ordering,
};

use crate::{
  data::{
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
use rustls::Certificate;
use serde_json::json;
use smoke::messages::EmbMessage;
use smoke::messages::RhizMessage::{self, *};
use tauri::{api::notification::Notification, AppHandle, Window};
use tokio::{io::BufReader, net::TcpStream};
use tokio::{select, sync::mpsc::Receiver};
use tokio_rustls::client::TlsStream;

use super::Networking;

pub struct ControlChannel<'a> {
  pub window: &'a Window,
  pub app: &'a AppHandle,
  pub rx: Receiver<EmberryMessage>,
  pub tls: BufReader<TlsStream<TcpStream>>,
  pub net: tauri::State<'a, Networking>,
  pub rc: &'a tauri::State<'a, RhizomeConnection>,
  pub identity: Certificate,
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
        msg = RhizMessage::recv_with(&mut self.tls, &mut buf) => self.handle_rhiz_msg(msg).await?
      }
    }
  }

  async fn handle_rhiz_msg(
    &mut self,
    msg: Result<RhizMessage, io::Error>,
  ) -> tauri::Result<()> {
    trace!("ctrl recv: {:?}", msg);
    match msg? {
      Shutdown() => return Ok(()),
      HasRoute(usr) => {
        let pending = self.net.pending.lock().unwrap().contains_key(&usr);
        self.window
          .emit(
            "has-route",
            json!({ "pending": pending, "usr": UserIdentifier::from(&usr).bs58, }),
          )
          .expect("Failed to emit HasRoute")
      }
      NoRoute(usr) => {
        // might want to remove the ".remove(&usr)" when trying to auto reconnect...
        let pending = self.net.pending.lock().unwrap().remove(&usr);
        self.window
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
          let mut guard = self.net.pending.lock().unwrap();
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
                  self.window
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

            self.window
              .emit("wants-room", &ident_info)
              .expect("Failed to emit WantsRoom event");

            /* Create a new notification for the message */
            if !crate::FOCUS.load(Ordering::SeqCst) {
              Notification::new(self.app.config().tauri.bundle.identifier.clone())
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
          let priority = self.identity.0 < usr.cert_data;
          let msg = EmbMessage::Accept(priority);
          state::send(self.rc, msg).await?;
        }
      }
      AcceptedRoom(id, usr) => {
        let priority = self.identity.0 < usr.cert_data;
        try_holepunch(self.window.clone(), self.app, self.net.clone(), id, usr, priority).await?
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

}
