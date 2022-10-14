mod get_or_insert;

pub use get_or_insert::*;

/// Uses the crate local mutex sqlite connection to run the supplied action
/// The supplied action MUST never panic
///
/// # Errors
/// This function will return:</br>
/// The first error returned by the supplied action
///
/// # Panics
/// If said mutex is poisoned
#[macro_export]
macro_rules! exec {
  ($action:ident, $input:expr) => {{
    let db = &*$crate::data::sqlite::DATABASE;
    let mut db = db.lock().unwrap();
    $action(&mut db, $input)
  }};
}
