pub mod actions;

use lazy_static::lazy_static;
use log::warn;
use rusqlite::Connection;
use std::sync::Mutex;

use crate::data::path::DATA;

lazy_static! {
  static ref DATABASE: Mutex<Connection> = validate();
}

fn generate() -> Connection {
  let db = Connection::open(DATA.clone());

  match db {
    Err(err) => {
      warn!("Unable to open database: {}", err);
      warn!("Using in memory database");
      // using unwrap here is safe as sqlite does not return an error
      // when creating in memory database
      Connection::open_in_memory().expect("In memory database creation failed")
    }
    Ok(db) => db,
  }
}

fn validate() -> Mutex<Connection> {
  todo!("check version and if needed update/create tables");
  Mutex::new(generate())
}
