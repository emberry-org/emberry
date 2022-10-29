use log::warn;
use rusqlite::{params, Connection};

use crate::data::{UserIdentifier, UserInfo, UserRelation};

pub fn get(db: &mut Connection, data: &UserIdentifier) -> Result<UserInfo, rusqlite::Error> {
  let mut statement =
    db.prepare("SELECT id, username, relation FROM users WHERE tls_cert = (?1)")?;
  let mut rows = statement.query_map([&data.bs58], |row| {
    let id: u64 = row.get(0)?;
    let username: String = row.get(1)?;
    let relation = UserRelation::from(row.get::<usize, u8>(2)?);
    Ok((id, username, relation))
  })?;

  if let Some(row) = rows.next() {
    let (_id, name, relation) = row?;
    if rows.next().is_some() {
      warn!(
        "more then one database entry for certifificate: '{}'",
        &data.bs58
      );
    }
    Ok(UserInfo {
      username: name,
      relation,
    })
  } else {
    Ok(UserInfo {
      username: data.bs58.to_string(),
      relation: UserRelation::Stranger,
    })
  }
}

pub fn upsert(
  db: &mut Connection,
  certificate: String,
  info: UserInfo,
) -> Result<(), rusqlite::Error> {
  let _ = db.execute(
    r#"INSERT INTO users (username, tls_cert, relation) VALUES (?1, ?2, ?3)
ON CONFLICT (user_id) DO UPDATE 
SET name = excluded.name, deck = excluded.deck, currency = excluded.currency ,elo = excluded.elo"#,
    params![info.username, info.relation as u8, certificate],
  )?;
  Ok(())
}
