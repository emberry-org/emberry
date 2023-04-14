use std::sync::RwLock;

use crate::data::{IdentifiedUserInfo, UserIdentifier};

use super::{
  path::DATA,
  sqlite::{try_exec, user::try_get},
  PemfileReader, UserInfo, UserRelation,
};
use once_cell::sync::Lazy;
use tokio_rustls::rustls::{Certificate, PrivateKey};
use tracing::{error, warn};

/// PemfileReader to the .pem of the current user
pub static PEM: Lazy<PemfileReader> = Lazy::new(pem_reader);
/// IdentifiedUserInfo of the current user;
/// None if [PEM] has no valid cert
pub static IDI: Lazy<RwLock<Option<IdentifiedUserInfo<'static>>>> =
  Lazy::new(|| RwLock::new(maybe_info()));

#[deprecated]
/// cert and key of the local user from .pem file
pub static PEM_DATA: Lazy<Option<(Certificate, PrivateKey)>> = Lazy::new(maybe_pem_data);

fn maybe_pem_data() -> Option<(Certificate, PrivateKey)> {
  match PEM.parse() {
    Ok(data) => Some(data),
    Err(err) => {
      warn!(
        "Failed to parse Certificate and PrivateKey from '{}', Err: '{}'",
        PEM.filepath.to_string_lossy(),
        err
      );
      None
    }
  }
}

fn maybe_info<'a>() -> Option<IdentifiedUserInfo<'a>> {
  let id = maybe_identifier()?;
  let info = match try_exec(try_get, &id) {
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
    warn!(
      "User identity PEM file does not exist: '{}'",
      PEM.filepath.to_string_lossy()
    );
    return None;
  }
  match (&*PEM).try_into() {
    Ok(id) => Some(id),
    Err(err) => {
      error!("Unable to obtain local user id: '{}'", err);
      None
    }
  }
}

fn pem_reader() -> PemfileReader {
  let mut filepath = DATA.clone();
  filepath.push("identity.pem");
  PemfileReader { filepath }
}
