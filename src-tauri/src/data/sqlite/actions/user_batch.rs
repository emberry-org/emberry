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
pub fn get_limit_offset(
  db: &mut Connection,
  range: (i64, usize),
) -> Result<Vec<IdentifiedUserInfo>, rusqlite::Error> {
  let (limit, offset) = range;
  let mut statement = db.prepare(
    "SELECT tls_cert, username, relation FROM users WHERE relation < 255 LIMIT (?1) OFFSET (?2)",
  )?;
  let rows = statement.query_map(params![limit, offset], |row| {
    let cert: String = row.get(0)?;
    let username: String = row.get(1)?;
    let relation = UserRelation::from(row.get::<usize, u8>(2)?);
    let ident_info = IdentifiedUserInfo {
      identifier: UserIdentifier { bs58: cert },
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

  use super::*;
  use crate::data::sqlite::{schema, user::upsert};
  use rusqlite::Connection;

  fn sample_users() -> Vec<IdentifiedUserInfo> {
    let mut all = vec![];
    for i in 0..10 {
      all.push(IdentifiedUserInfo {
        identifier: UserIdentifier {
          bs58: format!("user{}", i),
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

  fn get_limit(
    db: &mut Connection,
    limit: i64,
  ) -> Result<Vec<IdentifiedUserInfo>, rusqlite::Error> {
    get_limit_offset(db, (limit, 0))
  }

  /// Tests if get_limit actually return at most `limit` entries
  #[test_log::test]
  fn get_exhausted_limit() {
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
  #[test_log::test]
  fn get_exhausted_limit_offset() {
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
  #[test_log::test]
  fn get_overshoot_limit() {
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
  #[test_log::test]
  fn get_overshoot_limit_offset() {
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
  #[test_log::test]
  fn get_all_offset() {
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
  #[test_log::test]
  fn get_all() {
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
  #[test_log::test]
  fn get_all_exclude_local() {
    let mut db = Connection::open_in_memory().unwrap();
    schema::validate(&mut db);

    // local user
    let user = IdentifiedUserInfo {
      identifier: UserIdentifier {
        bs58: "user_local".into(),
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
