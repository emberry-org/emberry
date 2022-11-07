use std::sync::RwLock;

use crate::data::{IdentifiedUserInfo, UserIdentifier};

use super::{
  path::CONFIG,
  sqlite::{exec, user::get},
  PemfileReader, UserInfo, UserRelation,
};
use lazy_static::lazy_static;

lazy_static! {
  /// PemfileReader to the .pem of the current user
  pub static ref PEM: PemfileReader = pem_reader();
  /// IdentifiedUserInfo of the current user;
  /// None if [PEM] has no valid cert
  pub static ref IDI: RwLock<Option<IdentifiedUserInfo<'static>>> = RwLock::new(maybe_info());
}

fn maybe_info<'a>() -> Option<IdentifiedUserInfo<'a>> {
  let id = maybe_identifier()?;
  let info = match exec(get, &id) {
    Ok(info) => info,
    Err(_err) => UserInfo {
      relation: UserRelation::Local,
      username: id.bs58.to_string(),
    },
  };

  Some(IdentifiedUserInfo {
    identifier: id,
    info,
  })
}

fn maybe_identifier<'a>() -> Option<UserIdentifier<'a>> {
  if !PEM.filepath.exists() {
    log::warn!("User identity PEM file does not exist: '{}'", PEM.filepath.to_string_lossy());
    return None;
  }
  match (&*PEM).try_into() {
    Ok(id) => Some(id),
    Err(err) => {
      log::error!("Unable to obtain local user id: '{}'", err);
      None
    }
  }
}

fn pem_reader() -> PemfileReader {
  let mut filepath = CONFIG.clone();
  filepath.push("identity.pem");
  PemfileReader { filepath }
}
