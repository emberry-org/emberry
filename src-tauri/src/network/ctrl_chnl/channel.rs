use super::room_creation::try_holepunch;
use std::io::{self, ErrorKind};

use crate::{
  data::{fetch_userinfo, UserIdentifier},
  frontend::{notification, os_notify},
  network::{ctrl_chnl::state, RRState, UserIdentification},
};

pub use super::messages::EmberryMessage;
pub use super::state::RwOption;
pub use super::state::{RhizomeConnection, State};
use serde_json::json;
use smoke::messages::EmbMessage;
use smoke::messages::RhizMessage::{self, *};
use tauri::{AppHandle, Window};
use tokio::{io::BufReader, net::TcpStream};
use tokio::{select, sync::mpsc::Receiver};
use tokio_rustls::client::TlsStream;
use tracing::{error, trace};

use super::Networking;

pub struct ControlChannel<'a> {
  pub window: &'a Window,
  pub app: &'a AppHandle,
  pub rx: Receiver<EmberryMessage>,
  pub tls: BufReader<TlsStream<TcpStream>>,
  pub net: tauri::State<'a, Networking>,
  pub rc: &'a tauri::State<'a, RhizomeConnection>,
  pub identification: UserIdentification,
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

  async fn handle_rhiz_msg(&mut self, msg: Result<RhizMessage, io::Error>) -> tauri::Result<()> {
    trace!("ctrl recv: {:?}", msg);
    match msg? {
      Shutdown() => return Ok(()),
      HasRoute(usr) => {
        let pending = self.net.pending.lock().unwrap().contains_key(&usr);
        self
          .window
          .emit(
            "has-route",
            json!({ "pending": pending, "usr": UserIdentifier::from(&usr).bs58, }),
          )
          .expect("Failed to emit HasRoute")
      }
      NoRoute(usr) => {
        // might want to remove the ".remove(&usr)" when trying to auto reconnect...
        let pending = self.net.pending.lock().unwrap().remove(&usr);
        self
          .window
          .emit(
            "no-route",
            json!({ "pending": pending.is_some(), "usr": UserIdentifier::from(&usr).bs58, }),
          )
          .expect("Failed to emit NoRoute")
      }
      WantsRoom(usr) => {
        // only option here is None or RRState::Pending
        let message = {
          let mut guard = self.net.pending.lock().unwrap();
          let handle_request = || -> tauri::Result<()> {
            let ident_info = fetch_userinfo(UserIdentifier::from(&usr), self.window)?;

            self
              .window
              .emit("wants-room", &ident_info)
              .expect("Failed to emit WantsRoom event");

            os_notify(notification().title(format!(
              "{} wants to connect to you",
              ident_info.info.username
            )));

            Ok(())
          };

          match guard.get(&usr) {
            None => {
              return handle_request();
            }
            Some(RRState::Pending(instant)) => {
              if instant.elapsed() > smoke::ROOM_REQ_TIMEOUT {
                guard.remove(&usr);
                trace!("previous pending room request timed out, discarding");
                return handle_request();
              } else {
                // Here we get a WantsRoom while we already requested a room with the same peer
                // (they were unaware when they made their request).
                //
                // In this situation the user with the higher value as pub key rejects the request
                // the client with the lower value pub key auto accepts
                // this is done to remove the dublicate request
                let priority = self.identification.certificate.0 < usr.cert_data;
                EmbMessage::Accept(priority)
              }
            }
            Some(RRState::Agreement) => {
              error!("received room request while we are already forming one with this peer");
              return Ok(());
            }
          }
        };
        state::send(self.rc, message).await?;
      }
      AcceptedRoom(id, usr) => {
        let priority = self.identification.certificate.0 < usr.cert_data;
        if let Err(err) = try_holepunch(
          self.window.clone(),
          self.net.clone(),
          id,
          &usr,
          &self.identification,
          priority,
        )
        .await
        {
          self
            .window
            .emit(
              "error",
              format!("Connecting to {usr:?} failed! ERROR: '{}'", err),
            )
            .expect("Failed to emit error event");
        }
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
