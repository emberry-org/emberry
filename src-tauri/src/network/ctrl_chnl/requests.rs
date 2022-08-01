use smoke::{messages::EmbMessage, User};

use crate::network::ctrl_chnl::RhizomeConnection;
use crate::network::Networking;

use super::state;

#[tauri::command(async)]
pub async fn request_room(
  usr: User,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  // try to add to pending list
  {
    let mut guard = net.pending.lock().unwrap();
    if guard.contains_key(&usr) {
      // return if the request is already pending
      todo!("send a room accept when there remote is waiting for approval or block the room request ui until room accept ui has been closed");
      // DISCUSS WITH MAX
      return Ok(());
    }

    guard.insert(usr, crate::network::RRState::Pending);
  }

  let msg = EmbMessage::Room(usr);

  state::send(&rc, msg).await?;
  Ok(())
}
