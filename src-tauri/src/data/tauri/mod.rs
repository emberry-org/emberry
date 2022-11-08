use std::borrow::Cow;
use std::io::ErrorKind;

use tauri::Window;

use super::sqlite::user_batch::get_limit_offset;
use super::{config, IdentifiedUserInfo, UserIdentifier, UserInfo};

use super::sqlite::{exec, user::*};

#[tauri::command]
pub fn get_usr_info(bs58cert: String) -> Result<UserInfo, tauri::Error> {
  let user = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };

  exec(get, &user).map_err(tauri::Error::Io)
}

#[tauri::command]
pub fn get_usrs<'a>(
  limit: i64,
  offset: usize,
) -> Result<Vec<IdentifiedUserInfo<'a>>, tauri::Error> {
  exec(get_limit_offset, (limit, offset)).map_err(tauri::Error::Io)
}

#[tauri::command]
pub fn get_local<'a>() -> Option<IdentifiedUserInfo<'a>> {
  let lock = config::IDI.read().unwrap();
  lock.clone()
}

#[tauri::command]
pub fn update_username(window: Window, name: String) {
  let frontend_event = |info: &IdentifiedUserInfo| {
    let event = format!("usr_name_{}", info.identifier.bs58);
    if let Err(err) = window.emit(&event, &info.info.username) {
      log::error!("Failed to emit event: '{}'", err);
    }
  };
  let mut lock = config::IDI.write().unwrap();
  let option = lock.as_mut();
  if let Some(mut id_info) = option {
    if name != id_info.info.username {
      id_info.info.username = name;
      match exec(upsert, (id_info, frontend_event)) {
        Ok(()) => (),
        Err(err) => log::warn!("Could not update local username: '{}'", err),
      }
    }
  }
}
