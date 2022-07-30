use std::io::{Error, ErrorKind};

use smoke::{messages::EmbMessage, User};

use crate::network::ctrl_chnl::{EmberryMessage, RhizomeConnection};
use crate::network::Networking;

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

  let msg = EmberryMessage::Direct(EmbMessage::Room(usr));

  // obtain the sender to the ctrl channel
  let guard = rc.read().await;
  let tx = match &*guard {
    Some(rc) => &rc.channel,
    None => {
      return Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "No connection to rhizome",
      )))
    }
  };

  // send to ctrl channel
  tx.send(msg).await.map_err(|_| {
    tauri::Error::Io(Error::new(
      ErrorKind::ConnectionReset,
      "Rhizome connection closed",
    ))
  })
}
