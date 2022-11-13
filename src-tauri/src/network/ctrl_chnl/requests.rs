use std::borrow::Cow;

use smoke::{messages::EmbMessage, User};

use crate::data::UserIdentifier;
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
  let usr: User = UserIdentifier {
    bs58: Cow::Owned(bs58cert),
  }
  .try_into()?;

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
