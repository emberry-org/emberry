use crate::data::UserIdentifier;

use super::{path::CONFIG, PemfileReader};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref PEM: PemfileReader = pem_reader();
  pub static ref ID: Option<UserIdentifier<'static>> = {
    if !PEM.filepath.exists() {
      return None;
    }
    match (&*PEM).try_into() {
      Ok(id) => Some(id),
      Err(err) => {
        log::error!("Unable to obtain local user id: '{}'", err);
        None
      }
    }
  };
}

fn pem_reader() -> PemfileReader {
  let mut filepath = CONFIG.clone();
  filepath.push("identity.pem");
  PemfileReader { filepath }
}
