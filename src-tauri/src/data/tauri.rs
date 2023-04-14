use std::borrow::Cow;

use tauri::Window;
use tracing::{error, warn};

use super::sqlite::user_batch::get_limit_offset;
use super::{cert_gen, config, IdentifiedUserInfo, UserIdentifier, UserInfo};

use super::sqlite::{exec, try_exec, user::*};

#[tauri::command]
pub fn get_usr_info(bs58cert: String) -> UserInfo {
  let user = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };

  exec(get, &user)
}

#[tauri::command]
pub fn get_usrs<'a>(
  limit: i64,
  offset: usize,
) -> Result<Vec<IdentifiedUserInfo<'a>>, tauri::Error> {
  try_exec(get_limit_offset, (limit, offset)).map_err(tauri::Error::Io)
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
      error!("Failed to emit event: '{}'", err);
    }
  };
  let mut lock = config::IDI.write().unwrap();
  let option = lock.as_mut();
  if let Some(mut id_info) = option {
    if name != id_info.info.username {
      id_info.info.username = name;
      match try_exec(upsert, (id_info, frontend_event)) {
        Ok(()) => (),
        Err(err) => warn!("Could not update local username: '{}'", err),
      }
    }
  }
}

#[tauri::command]
pub fn generate_user_certificate() {
  cert_gen::generate_cert(&config::PEM.filepath).unwrap()
}
