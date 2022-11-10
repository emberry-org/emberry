use std::borrow::Cow;

use rusqlite::{params, Connection};

use crate::data::{IdentifiedUserInfo, UserIdentifier, UserInfo, UserRelation};

/// Tries to get `limit` amount of [IdentifiedUserInfo] from the DB</br>
/// `offset` can be used to offset the start position of the query
///
/// There will be less or equal to `limit` elements in the result [Vec<IdentifiedUserInfo>]
///
/// # Errors
/// This function will return:</br>
/// The first error returned by executing the underlying SQLite query on `db`
pub fn get_limit_offset<'a>(
  db: &mut Connection,
  range: (i64, usize),
) -> Result<Vec<IdentifiedUserInfo<'a>>, rusqlite::Error> {
  let (limit, offset) = range;
  let mut statement = db.prepare(
    "SELECT tls_cert, username, relation FROM users WHERE relation < 255 LIMIT (?1) OFFSET (?2)",
  )?;
  let rows = statement.query_map(params![limit, offset], |row| {
    let cert: String = row.get(0)?;
    let username: String = row.get(1)?;
    let relation = UserRelation::from(row.get::<usize, u8>(2)?);
    let ident_info = IdentifiedUserInfo {
      identifier: UserIdentifier {
        bs58: Cow::Owned(cert),
      },
      info: UserInfo { username, relation },
    };
    Ok(ident_info)
  })?;

  let mut all = Vec::new();

  for row in rows {
    all.push(row?);
  }

  Ok(all)
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use super::*;
  use crate::data::sqlite::{schema, user::upsert};
  use rusqlite::Connection;

  fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
  }

  fn sample_users() -> Vec<IdentifiedUserInfo<'static>> {
    let mut all = vec![];
    for i in 0..10 {
      all.push(IdentifiedUserInfo {
        identifier: UserIdentifier {
          bs58: Cow::Owned(format!("user{}", i)),
        },
        info: UserInfo {
          username: format!("generic_username{}", i),
          relation: UserRelation::Known,
        },
      });
    }
    all[3].info.relation = UserRelation::Friend;
    all
  }

  fn create_sample_users(db: &mut Connection) -> Result<(), rusqlite::Error> {
    let all = sample_users();
    for user in all {
      upsert(db, (&user, |_| ()))?;
    }
    Ok(())
  }

  fn get_limit<'a>(
    db: &mut Connection,
    limit: i64,
  ) -> Result<Vec<IdentifiedUserInfo<'a>>, rusqlite::Error> {
    get_limit_offset(db, (limit, 0))
  }

  /// Tests if get_limit actually return at most `limit` entries
  #[test]
  fn get_exhausted_limit() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit(&mut db, 5);

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[0..5],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit actually return at most `limit` entries and if `offset` works
  #[test]
  fn get_exhausted_limit_offset() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit_offset(&mut db, (5, 3));

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[3..8],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit return less then `limit` if there are no more in db
  #[test]
  fn get_overshoot_limit() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit(&mut db, 100);

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[..],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit return less then `limit` if there are no more in db
  /// and if offset works with that
  #[test]
  fn get_overshoot_limit_offset() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit_offset(&mut db, (100, 3));

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[3..],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit return all if `limit = -1`
  /// and if offset works with that
  #[test]
  fn get_all_offset() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit_offset(&mut db, (-1, 3));

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[3..],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit return all if `limit = -1`
  #[test]
  fn get_all() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    let result = create_sample_users(&mut db);
    if let Err(err) = result {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit_offset(&mut db, (-1, 0));

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[..],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }

  /// Tests if get_limit return all if `limit = -1`
  #[test]
  fn get_all_exclude_local() {
    init();
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    // local user
    let user = IdentifiedUserInfo {
      identifier: UserIdentifier {
        bs58: Cow::Owned("user_local".into()),
      },
      info: UserInfo {
        username: "local_user_username".into(),
        relation: UserRelation::Local,
      },
    };

    if let Err(err) = upsert(&mut db, (&user, |_| ())) {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }
    if let Err(err) = create_sample_users(&mut db) {
      panic!("error executing creating 'upsert' command: '{}'", err);
    }

    let result = get_limit_offset(&mut db, (-1, 0));

    match result {
      Err(err) => {
        panic!("error executing 'get_limit' command: '{}'", err);
      }
      Ok(result) => {
        let exprected = sample_users();
        assert_eq!(
          &result[..],
          &exprected[..],
          "\nget returned 'left' but 'right' was expected"
        );
      }
    }
  }
}
