mod actions;
mod schema;

use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;
use tracing::{info, warn};

use crate::data::path::DATA;

pub use actions::*;

pub static DATABASE: Lazy<Mutex<Connection>> = Lazy::new(generate);

fn generate() -> Mutex<Connection> {
  let mut path = DATA.clone();
  path.push("warehouse.db3");
  let db = Connection::open(&path);

  match db {
    Err(err) => {
      warn!("Unable to open database file: {}", err);
      warn!("Using in memory database");
      // using unwrap here is safe as sqlite does not return an error
      // when creating in memory database
      let mut db = Connection::open_in_memory().expect("In memory database creation failed");
      schema::validate(&mut db);
      Mutex::new(db)
    }
    Ok(mut db) => {
      info!("[created/write_open] file: {path:?}");
      schema::validate(&mut db);
      Mutex::new(db)
    }
  }
}
