use std::io::{Error, ErrorKind};

use log::error;
use smoke::{messages::EmbMessage, User};

use crate::network::ctrl_chnl::RhizomeConnection;
use crate::network::Networking;

use super::state;

// todo : would be nice if `request_room` called a tauri event if the user was not found. (containing the user pubkey)
#[tauri::command(async)]
pub async fn request_room(
  bs58cert: String,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  // This is a temporary solution
  let usr = User {
    cert_data: match bs58::decode(&bs58cert).into_vec() {
      Ok(cert) => cert,
      Err(err) => {
        error!("cannot parse base58 sting: '{}'. Error: {}", bs58cert, err);
        return Err(tauri::Error::Io(Error::new(
          ErrorKind::InvalidData,
          "bs58 parsing error",
        )));
      }
    },
  };

  // try to add to pending list
  let msg = match net.pending.lock().unwrap().entry(usr.clone()) {
    std::collections::hash_map::Entry::Occupied(mut e) => {
      e.insert(crate::network::RRState::Agreement);
      EmbMessage::Accept(true)
    }
    std::collections::hash_map::Entry::Vacant(e) => {
      e.insert(crate::network::RRState::Pending);
      EmbMessage::Room(usr)
    }
  };

  state::send(&rc, msg).await?;
  Ok(())
}
