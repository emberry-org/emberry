use std::io::{self, ErrorKind};

use tauri::Window;

use super::{
  sqlite::{
    exec, try_exec,
    user::{try_get, upsert},
  },
  IdentifiedUserInfo, UserIdentifier, UserInfo,
};

/// Fetches [IdentifiedUserInfo] about a [User] from the database and informs the frontend about database changes
pub fn fetch_userinfo<'a>(
  ident: UserIdentifier<'a>,
  window: &Window,
) -> tauri::Result<IdentifiedUserInfo<'a>> {
  let info = exec(try_get, &ident);
  let ident_info = match info {
    Ok(info) => IdentifiedUserInfo {
      identifier: ident,
      info,
    },
    Err(rusqlite::Error::QueryReturnedNoRows) => {
      let ident_info = IdentifiedUserInfo {
        info: UserInfo {
          username: ident.bs58.to_string(),
          relation: crate::data::UserRelation::Stranger,
        },
        identifier: ident,
      };
      let new_user_event = |ident_info: &IdentifiedUserInfo| {
        window
          .emit("new-user", &ident_info.info.username)
          .expect("Failed to emit new-user event")
      };
      try_exec(upsert, (&ident_info, new_user_event))?;

      ident_info
    }
    Err(err) => {
      log::error!("SQLite access error : '{}'", err);
      return Err(tauri::Error::Io(io::Error::new(
        ErrorKind::Other,
        "SQLite error",
      )));
    }
  };

  Ok(ident_info)
}
