use std::io::{Error, ErrorKind};

use smoke::{messages::EmbMessage, User};

use crate::network::ctrl_chnl::{EmberryMessage, RhizomeConnection};
use crate::network::Networking;

use super::state;

#[tauri::command(async)]
pub async fn request_room(
  usr: User,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  // try to add to pending list
  if !net.pending.lock().unwrap().insert(usr) {
    // return if the request is already pending
    return Ok(());
  }

  let msg = EmbMessage::Room(usr);

  state::send(&rc, msg).await?;
  Ok(())
}
