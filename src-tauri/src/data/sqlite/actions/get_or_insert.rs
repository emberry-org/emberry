use rusqlite::Connection;
use smoke::{PubKey, User};

use crate::exec;

fn _get_or_insert(db: &mut Connection, user: &PubKey) -> Result<User, rusqlite::Error> {
  
  let transaction = db.transaction()?;
  let mut statement = transaction
            .prepare(
                "SELECT DISTINCT user_id, card_name, skin_name FROM card_data WHERE user_id = (?1)",
            )
            .unwrap();
  Err(rusqlite::Error::InvalidQuery)
}

#[tauri::command]
pub fn get_or_insert(user: &PubKey) -> Result<User, rusqlite::Error> {
  exec!(_get_or_insert, user)
}
