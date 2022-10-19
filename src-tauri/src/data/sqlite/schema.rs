use rusqlite::Connection;

pub fn validate(db: &mut Connection) {
  validate_user_table(db);
}

fn validate_user_table(db: &mut Connection) {
  db.execute(
    r#"CREATE TABLE IF NOT EXISTS "users" (
"id"	INTEGER NOT NULL UNIQUE,
"username"	TEXT,
"tls_cert"	BLOB NOT NULL,
"relation"	INTEGER,
PRIMARY KEY("id")
);"#,
    [],
  )
  .unwrap();
}

#[cfg(test)]
mod tests {
  use rusqlite::Connection;

  #[test]
  fn validate() {
    let mut db = Connection::open_in_memory().unwrap();
    super::validate(&mut db); //run once to test creation
    super::validate(&mut db); //run twice to test validation of existing schema
  }
}
