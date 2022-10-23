use crate::network::{Networking, RRState};

use super::{state, RhizomeConnection};
use log::error;
use smoke::messages::EmbMessage;
use smoke::User;
use std::io::{Error, ErrorKind};
use tauri::Result;

#[tauri::command(async)]
pub async fn accept_room(
  bs58_cert: String,
  accepted: bool,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()> {
  {
    // This is a temporary solution
    let usr = User {
      cert_data: match bs58::decode(&bs58_cert).into_vec() {
        Ok(cert) => cert,
        Err(err) => {
          error!("cannot parse base58 sting: '{}'. Error: {}", bs58_cert, err);
          return Err(tauri::Error::Io(Error::new(
            ErrorKind::InvalidData,
            "bs58 parsing error",
          )));
        }
      },
    };

    let mut guard = net.pending.lock().unwrap();
    let some = if accepted {
      guard.insert(usr, RRState::Agreement).is_some()
    } else {
      guard.contains_key(&usr)
    };
    if some {
      return Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "It should not be possible to interact with the room request popup if the request is already pending"
      )));
    }
  }

  let msg = EmbMessage::Accept(accepted);
  state::send(&rc, msg).await?;
  Ok(())
}
