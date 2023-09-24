use std::io::{Error, ErrorKind};

use smoke::messages::RoomId;
use smoke::User;

use tracing::error;
use tracing::trace;
use tracing::trace_span;
use tracing::Instrument;

use crate::data::UserIdentifier;
use crate::network::peer_tunnel::TunnelBore;
use crate::network::Networking;
use crate::network::RRState;
use crate::network::UserIdentification;

#[derive(Clone, serde::Serialize)]
struct NewRoomPayload {
  room_id: String,
  peer_id: String,
}

pub async fn try_holepunch(
  window: tauri::Window,
  net_state: tauri::State<'_, Networking>,
  room_id: Option<RoomId>,
  usr: &User,
  identification: &UserIdentification,
) -> tauri::Result<()> {
  if let Some(room_id) = room_id {
    let span = match net_state.pending.lock().unwrap().get(usr) {
      Some(RRState::Accepted) => {
        let local = User {
          cert_data: identification.certificate.0.clone(),
        };
        trace_span!("room_req", source = ?usr, target = ?local)
      }
      Some(RRState::Requested(_instant)) => {
        let local = User {
          cert_data: identification.certificate.0.clone(),
        };
        trace_span!("room_req", source = ?local, target = ?usr)
      }
      None => {
        error!("rhizome sent accepted room for a non existing room");
        return Err(tauri::Error::Io(Error::new(
          ErrorKind::Other,
          "Rhizome just sent a malicious room opening packet (this should not happen)",
        )));
      }
    };

    let tunnel = TunnelBore {
      window: window.clone(),
      room_id: room_id.clone(),
      peer: usr,
      identification,
    }
    .drill()
    .instrument(span)
    .await?;

    net_state
      .chats
      .lock()
      .unwrap()
      .insert(room_id.clone(), tunnel);

    window
      .emit(
        "new-room",
        NewRoomPayload {
          room_id: bs58::encode(&room_id.0).into_string(),
          peer_id: UserIdentifier::from(usr).bs58,
        },
      )
      .expect("Failed to emit WantsRoom event");
  } else {
    // rejection
    let mut guard = net_state.pending.lock().unwrap();
    if let Some(kv) = guard.get_key_value(usr) {
      match kv.1 {
        RRState::Accepted => {
          let local = User {
            cert_data: identification.certificate.0.clone(),
          };
          let span = trace_span!("room_req", source = ?usr, target = ?local);
          let _guard = span.enter();
          trace!("colling requests dropping");
          return Ok(());
        }
        RRState::Requested(_instant) => {
          let local = User {
            cert_data: identification.certificate.0.clone(),
          };
          let span = trace_span!("room_req", source = ?local, target = ?usr);
          let _guard = span.enter();
          trace!("room request rejected");
          guard.remove(usr); // This case is a normal rejection
        }
      }
    }
  }

  Ok(())
}
