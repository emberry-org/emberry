use log::warn;
use rusqlite::{params, Connection};

use crate::data::{UserInfo, UserRelation, UserIdentifier};

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

fn _update(
  db: &mut Connection,
  certificate: String,
  info: UserInfo,
) -> Result<(), rusqlite::Error> {
  let _ = db.execute(
    "UPDATE users SET username = (?1), relation = (?2) WHERE tls_cert = (?3)",
    params![info.username, info.relation as u8, certificate],
  )?;
  Ok(())
}

fn _insert(
  db: &mut Connection,
  certificate: String,
  info: UserInfo,
) -> Result<(), rusqlite::Error> {
  let _ = db.execute(
    "INSERT INTO users (username, relation, tls_cert) VALUES (?1, ?2, ?3)",
    params![info.username, info.relation as u8, certificate],
  )?;
  Ok(())
}
