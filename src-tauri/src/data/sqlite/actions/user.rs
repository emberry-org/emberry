use log::warn;
use rusqlite::{params, Connection};

use crate::data::{IdentifiedUserInfo, UserIdentifier, UserInfo, UserRelation};

pub fn get(db: &mut Connection, data: &UserIdentifier) -> Result<UserInfo, rusqlite::Error> {
  let mut statement =
    db.prepare("SELECT username, relation FROM users WHERE tls_cert = (?1)")?;
  let mut rows = statement.query_map([&data.bs58], |row| {
    let username: String = row.get(0)?;
    let relation = UserRelation::from(row.get::<usize, u8>(1)?);
    Ok((username, relation))
  })?;

  if let Some(row) = rows.next() {
    let (name, relation) = row?;
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
    log::info!("no database entry for '{}'", &data.bs58);
    Ok(UserInfo {
      username: data.bs58.to_string(),
      relation: UserRelation::Stranger,
    })
  }
}

pub fn upsert(db: &mut Connection, ident_info: &IdentifiedUserInfo) -> Result<(), rusqlite::Error> {
  log::trace!("upserting entry for: '{}'", ident_info.identifier.bs58);
  let _ = db.execute(
    r#"INSERT INTO users (tls_cert, username, relation) VALUES (?1, ?2, ?3)
ON CONFLICT (tls_cert) DO UPDATE
SET username = excluded.username, relation = excluded.relation"#,
    params![
      ident_info.identifier.bs58,
      ident_info.info.username,
      ident_info.info.relation as u8,
    ],
  )?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use super::*;
  use crate::data::sqlite::schema;
  use rusqlite::Connection;

  fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
  }

  fn sample_user_ident() -> UserIdentifier<'static> {
    UserIdentifier {
      bs58: Cow::Owned("bs58 certificate string".to_string()),
    }
  }

  fn sample_user_info() -> UserInfo {
    UserInfo {
      relation: UserRelation::Known,
      username: "special username".into(),
    }
  }

  fn sample_user_info_updated() -> UserInfo {
    UserInfo {
      relation: UserRelation::Friend,
      username: "updated username".into(),
    }
  }

  fn create_sample_user(db: &mut Connection) -> Result<(), rusqlite::Error> {
    let identifier = sample_user_ident();
    let info = sample_user_info();
    let ident_info = IdentifiedUserInfo { identifier, info };

    upsert(db, &ident_info)
  }

  fn update_sample_user(db: &mut Connection) -> Result<(), rusqlite::Error> {
    let identifier = sample_user_ident();
    let info = sample_user_info_updated();
    let ident_info = IdentifiedUserInfo { identifier, info };

    upsert(db, &ident_info)
  }

  fn get_sample_user(db: &mut Connection) -> Result<UserInfo, rusqlite::Error> {
    let identifier = sample_user_ident();

    get(db, &identifier)
  }

  /// Tests if get returns sensible data for a non match
  #[test]
  fn get_non_existing_user() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let ident = sample_user_ident();

    let result = get_sample_user(&mut db);

    match result {
      Err(err) => {
        panic!("error executing 'get' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = UserInfo {
          relation: UserRelation::Stranger,
          username: ident.bs58.into_owned(),
        };
        assert_eq!(
          result, exprected,
          "\nget returned 'left' but 'right' was expected as there is no entry for that id"
        );
      }
    }
  }

  /// Tests if the upsert command can insert a user without errors
  #[test]
  fn insert_user() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    if let Err(err) = create_sample_user(&mut db) {
      panic!("error executing 'upsert' command: '{}'", err);
    }
  }

  /// Tests if the upsert command can run without errors twice on the same id
  #[test]
  fn insert_update_user() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = update_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing updating 'upsert' command: '{}'", err);
    }
  }

  /// Tests if a get after an upsert returns the correct data
  #[test]
  fn get_inserted_user() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_sample_user(&mut db);

    match result {
      Err(err) => {
        panic!("error executing 'get' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_user_info();
        assert_eq!(
          result, exprected,
          "\nget returned 'left' but 'right' was expected as it has previously been inserterd"
        );
      }
    }
  }

  /// Tests if a get after an updating upsert returns the correct data
  #[test]
  fn get_updated_user() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }
    let result = update_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing updating 'upsert' command: '{}'", err);
    }

    let result = get_sample_user(&mut db);

    match result {
      Err(err) => {
        panic!("error executing 'get' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_user_info_updated();
        assert_eq!(result, exprected, "\nget returned 'left' but 'right' was expected as it has previously been inserterd/updated");
      }
    }
  }
}
