use std::io::{Error, ErrorKind};

use super::EmberryMessage::Direct;
use super::RhizomeConnection;
use smoke::messages::EmbMessage;
use tauri::Result;

#[tauri::command(async)]
pub async fn send_room_affirm(
  state: tauri::State<'_, RhizomeConnection>,
  accepted: bool,
) -> Result<()> {
  let msg = EmbMessage::Accept(accepted);
  // create variable outside of inner scope
  // use inner scope to drop mutex guard before sending the message
  let guard = state.read().await;

  let tx = match &*guard {
    Some(rc) => &rc.channel,
    None => {
      return Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "No connection to rhizome",
      )))
    }
  };

  tx.send(Direct(msg)).await.map_err(|_| {
    tauri::Error::Io(Error::new(
      ErrorKind::ConnectionReset,
      "Rhizome connection closed",
    ))
  })
}
