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
use tracing::warn;

#[tauri::command(async)]
pub async fn accept_room(
  bs58cert: String,
  accepted: bool,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()> {
  let ident = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };
  let usr: User = (&ident).try_into()?;
  let err = || {
    Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "It should not be possible to interact with the room request popup if the request is already pending"
      )))
  };

  {
    let mut guard = net.pending.lock().unwrap();
    match guard.entry(usr) {
      std::collections::hash_map::Entry::Occupied(mut entry) => match entry.get() {
        RRState::Pending(timer) => {
          if timer.elapsed() > smoke::ROOM_REQ_TIMEOUT {
            entry.insert(RRState::Agreement);
          } else {
            warn!("tried accepting room that was already in agreement");
            return err();
          }
        }
        RRState::Agreement => return err(),
      },
      std::collections::hash_map::Entry::Vacant(entry) => {
        entry.insert(RRState::Agreement);
      }
    }
  }

  let msg = EmbMessage::Accept(accepted);
  state::send(&rc, msg).await?;
  Ok(())
}
