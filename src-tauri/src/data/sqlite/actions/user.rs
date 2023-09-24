use rusqlite::{params, Connection, Error::QueryReturnedNoRows};
use tracing::{debug, trace, warn};

use crate::data::{IdentifiedUserInfo, UserIdentifier, UserInfo, UserRelation};

/// Tries to get the user info entry from the given db
///
/// # Errors
/// This function will return:</br>
/// The first error returned by executing the underlying SQLite query on `db`</br>
/// [QueryReturnedNoRows] error if the entry is not present in the 'db'
pub fn try_get(db: &mut Connection, data: &UserIdentifier) -> Result<UserInfo, rusqlite::Error> {
  let mut statement = db.prepare("SELECT username, relation FROM users WHERE tls_cert = (?1)")?;
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
    debug!("no database entry for '{}'", &data.bs58);
    Err(QueryReturnedNoRows)
  }
}

/// Tries to get the user info entry from the given db
///
/// If there is no entry or db error,
/// [UserInfo] containing the bs58 encoded certificate as username
/// and [UserRelation::Stranger]
pub fn get(db: &mut Connection, data: &UserIdentifier) -> UserInfo {
  match try_get(db, data) {
    Ok(data) => data,
    Err(err) => {
      debug!("no database entry for '{}', SQL err: '{}'", &data.bs58, err);
      UserInfo {
        username: data.bs58.to_string(),
        relation: UserRelation::Stranger,
      }
    }
  }
}

/// Tries to upsert (insert or update) the user info entry into given database
/// `callback` is executed before the database access
///
/// # Errors
/// This function will return:</br>
/// The first error returned by executing the underlying SQLite query on `db`
pub fn upsert<'a, C>(
  db: &mut Connection,
  input: (&'a IdentifiedUserInfo, C),
) -> Result<(), rusqlite::Error>
where
  C: FnOnce(&'a IdentifiedUserInfo),
{
  let (ident_info, callback) = input;
  trace!("upserting entry for: '{}'", ident_info.identifier.bs58);

  callback(ident_info);

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
  use super::*;
  use crate::data::sqlite::schema;
  use rusqlite::Connection;

  fn sample_user_ident() -> UserIdentifier {
    UserIdentifier {
      bs58: "bs58 certificate string".to_string(),
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

    upsert(db, (&ident_info, |_| ()))
  }

  fn update_sample_user(db: &mut Connection) -> Result<(), rusqlite::Error> {
    let identifier = sample_user_ident();
    let info = sample_user_info_updated();
    let ident_info = IdentifiedUserInfo { identifier, info };

    upsert(db, (&ident_info, |_| ()))
  }

  fn get_sample_user(db: &mut Connection) -> UserInfo {
    let identifier = sample_user_ident();

    get(db, &identifier)
  }

  /// Tests if get returns sensible data for a non match
  #[test_log::test]
  fn get_non_existing_user() {
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let ident = sample_user_ident();

    let result = get_sample_user(&mut db);

    let exprected = UserInfo {
      relation: UserRelation::Stranger,
      username: ident.bs58,
    };
    assert_eq!(
      result, exprected,
      "\nget returned 'left' but 'right' was expected as there is no entry for that id"
    );
  }

  /// Tests if the upsert command can insert a user without errors
  #[test_log::test]
  fn insert_user() {
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    if let Err(err) = create_sample_user(&mut db) {
      panic!("error executing 'upsert' command: '{}'", err);
    }
  }

  /// Tests if the upsert command can run without errors twice on the same id
  #[test_log::test]
  fn insert_update_user() {
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
  #[test_log::test]
  fn get_inserted_user() {
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_user(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_sample_user(&mut db);

    let exprected = sample_user_info();
    assert_eq!(
      result, exprected,
      "\nget returned 'left' but 'right' was expected as it has previously been inserterd"
    );
  }

  /// Tests if a get after an updating upsert returns the correct data
  #[test_log::test]
  fn get_updated_user() {
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

    let exprected = sample_user_info_updated();
    assert_eq!(
      result, exprected,
      "\nget returned 'left' but 'right' was expected as it has previously been inserterd/updated"
    );
  }
}
