use crate::network::Networking;

use super::{state, RhizomeConnection};
use smoke::messages::EmbMessage;
use smoke::User;
use tauri::Result;

#[tauri::command(async)]
pub async fn accept_room(
  usr: User,
  accepted: bool,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()> {
  if accepted {
    if !net.pending.lock().unwrap().insert(usr) {
      // return if the request is already pending
      return Ok(());
    }
    let msg = EmbMessage::Accept(true);
    state::send(&rc, msg).await?;
  } else {
    let msg = EmbMessage::Accept(false);
    state::send(&rc, msg).await?;
    if !net.pending.lock().unwrap().remove(&usr) {
      // return if the request is already pending
      todo!("investigate how to handle this: denied request but there is already pending");
      // is this even possible?
    }
  }

  Ok(())
}
