use std::borrow::Cow;

use smoke::{messages::EmbMessage, User};
use tauri::Window;

use crate::data::sqlite::user::{try_get, upsert};
use crate::data::sqlite::{exec, try_exec};
use crate::data::{config, IdentifiedUserInfo, UserIdentifier, UserInfo};
use crate::network::ctrl_chnl::RhizomeConnection;
use crate::network::Networking;

use super::state;

// todo : would be nice if `request_room` called a tauri event if the user was not found. (containing the user pubkey)
#[tauri::command(async)]
pub async fn request_room(
  window: Window,
  bs58cert: String,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let ident = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };
  let usr: User = (&ident).try_into()?;

  if let Some((cert, _)) = config::PEM_DATA.as_ref() {
    if cert.0 == usr.cert_data {
      log::warn!("Cannot request a room with yourself");
      return Ok(());
    }
  } else {
    log::warn!("Cannot request a room without being authenticated to the server");
    return Ok(());
  }

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

  // When we send a room request check if the user is in the database
  // if thats not the case add it and tell the frontend about it
  if let EmbMessage::Room(_) = msg {
    let info = exec(try_get, &ident);
    if let Err(rusqlite::Error::QueryReturnedNoRows) = info {
      let ident_info = IdentifiedUserInfo {
        info: UserInfo {
          username: ident.bs58.to_string(),
          relation: crate::data::UserRelation::Stranger,
        },
        identifier: ident.as_ref(),
      };
      let new_user_event = |ident_info: &IdentifiedUserInfo| {
        window
          .emit("new-user", &ident_info.info.username)
          .expect("Failed to emit new-user event")
      };
      try_exec(upsert, (&ident_info, new_user_event))?;
    };
  }

  state::send(&rc, msg).await?;
  Ok(())
}
