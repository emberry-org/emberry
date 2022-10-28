mod actions;
mod schema;

use lazy_static::lazy_static;
use log::warn;
use rusqlite::Connection;
use std::sync::Mutex;

use crate::data::path::DATA;

pub use actions::*;

lazy_static! {
  pub static ref DATABASE: Mutex<Connection> = generate();
}

fn generate() -> Mutex<Connection> {
  let db = Connection::open(DATA.clone());

  match db {
    Err(err) => {
      warn!("Unable to open database: {}", err);
      warn!("Using in memory database");
      // using unwrap here is safe as sqlite does not return an error
      // when creating in memory database
      let mut db = Connection::open_in_memory().expect("In memory database creation failed");
      schema::validate(&mut db);
      Mutex::new(db)
    }
    Ok(mut db) => {
      schema::validate(&mut db);
      Mutex::new(db)
    }
  }
}
