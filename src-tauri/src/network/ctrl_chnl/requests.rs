use smoke::{messages::EmbMessage, User};

use crate::network::ctrl_chnl::RhizomeConnection;
use crate::network::Networking;

use super::state;

// todo : would be nice if `request_room` returned an error if the user was not found.
#[tauri::command(async)]
pub async fn request_room(
  usr: User,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  // try to add to pending list
  let msg = match net.pending.lock().unwrap().entry(usr) {
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
