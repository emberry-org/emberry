use rusqlite::Connection;

pub fn validate(db: &mut Connection) {
  validate_user_table(db);
}

fn validate_user_table(db: &mut Connection) {
  db.execute(
    r#"CREATE TABLE IF NOT EXISTS "users" (
"tls_cert" TEXT NOT NULL UNIQUE,
"username" TEXT NOT NULL,
"relation" INTEGER NOT NULL,
PRIMARY KEY("tls_cert")
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
