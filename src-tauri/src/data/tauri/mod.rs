use std::borrow::Cow;

use super::sqlite::user_batch::get_limit_offset;
use super::{UserIdentifier, UserInfo, IdentifiedUserInfo};

use super::sqlite::{exec, user::*};

#[tauri::command]
pub fn get_usr_info(bs58cert: String) -> Result<UserInfo, tauri::Error> {
  let user = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };

  exec(get, &user).map_err(tauri::Error::Io)
}

#[tauri::command]
pub fn get_usrs<'a>(limit: usize, offset: usize) -> Result<Vec<IdentifiedUserInfo<'a>>, tauri::Error> {
  exec(get_limit_offset, (limit, offset)).map_err(tauri::Error::Io)
}
