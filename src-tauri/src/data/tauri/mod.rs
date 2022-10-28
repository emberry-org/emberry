use std::borrow::Cow;

use smoke::User;

use super::{UserIdentifier, UserInfo};

use super::sqlite::{exec, user::*};

#[tauri::command]
pub fn get_usr_info(bs58cert: String) -> Result<UserInfo, tauri::Error> {
  let user: User = match (UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  }
  .try_into())
  {
    Ok(usr) => usr,
    Err(e) => {
      log::warn!("'{}', returning alleged bs58cert as username", e);
      return Ok(UserInfo {
        username: bs58cert,
        relation: crate::data::UserRelation::Undefined,
      });
    }
  };

  exec(get, &user).map_err(tauri::Error::Io)
}
