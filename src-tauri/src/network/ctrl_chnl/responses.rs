use crate::{
  data::UserIdentifier,
  network::{Networking, RRState},
};

use super::{state, RhizomeConnection};
use smoke::messages::EmbMessage;
use smoke::User;
use std::{
  borrow::Cow,
  io::{Error, ErrorKind},
};
use tauri::Result;

#[tauri::command(async)]
pub async fn accept_room(
  bs58cert: String,
  accepted: bool,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()> {
  {
    let ident = UserIdentifier {
      bs58: Cow::Borrowed(&bs58cert),
    };
    let usr: User = (&ident).try_into()?;

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
