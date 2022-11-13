use std::io;
use std::io::ErrorKind;

use rusqlite::Connection;

use super::DATABASE;

pub mod user;
pub mod user_batch;

/// Uses the crate local mutex sqlite connection to run the supplied action
/// The supplied action MUST never panic
///
/// # Errors
/// This function logs the first error returned by the supplied action
/// and returns an [io::Error] with [ErrorKind::Other] and no further information
/// then "SQLite error"
///
/// # Panics
/// If said mutex is poisoned
#[inline(always)]
pub fn try_exec<F, T, O>(action: F, input: T) -> Result<O, std::io::Error>
where
  F: FnOnce(&mut Connection, T) -> Result<O, rusqlite::Error>,
{
  let db = &*DATABASE;
  let mut db = db.lock().unwrap();
  action(&mut db, input).map_err(|err| {
    log::error!("SQLite access error: '{}'", err);
    io::Error::new(ErrorKind::Other, "SQLite error")
  })
}

/// Uses the crate local mutex sqlite connection to run the supplied action
/// The supplied action MUST never panic
///
/// # Panics
/// If said mutex is poisoned
#[inline(always)]
pub fn exec<F, T, O>(action: F, input: T) -> O
where
  F: FnOnce(&mut Connection, T) -> O,
{
  let db = &*DATABASE;
  let mut db = db.lock().unwrap();
  action(&mut db, input)
}
