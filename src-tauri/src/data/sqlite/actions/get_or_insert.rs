use rusqlite::Connection;
use smoke::{PubKey, User};

use crate::{exec, data::UserInfo};

fn _get_or_insert(db: &mut Connection, data: (&User, UserInfo)) -> Result<User, rusqlite::Error> {
  let transaction = db.transaction()?;
  let mut statement = transaction
    .prepare("SELECT id, username, relation FROM users WHERE tls_cert = (?1)")
    .unwrap();
  let rows = statement.query_map([data.0.key], |row| Ok(()));
  Err(rusqlite::Error::InvalidQuery)
}

#[tauri::command]
pub fn get_or_insert(user: &User, info: UserInfo) -> Result<User, rusqlite::Error> {
  exec!(_get_or_insert, (user, info))
}
