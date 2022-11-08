use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use smoke::User;

use super::UserInfo;

/// Wraps UserInfo with an Identifier
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IdentifiedUserInfo<'a> {
  pub identifier: UserIdentifier<'a>,
  pub info: UserInfo,
}

/// Client side local uniqe user identifier
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct UserIdentifier<'a> {
  pub bs58: Cow<'a, String>,
}

impl<'a> UserIdentifier<'a> {
  /// Creates a new UserIdentifier that uses a [Cow::Borrowed] from
  /// self
  pub fn as_ref(&'a self) -> UserIdentifier<'a> {
    UserIdentifier {
      bs58: Cow::Borrowed(&self.bs58),
    }
  }
}

impl<'a> From<&User> for UserIdentifier<'a> {
  fn from(usr: &User) -> Self {
    UserIdentifier {
      bs58: Cow::Owned(bs58::encode(&usr.cert_data).into_string()),
    }
  }
}

impl TryInto<User> for UserIdentifier<'_> {
  type Error = std::io::Error;
  fn try_into(self) -> Result<User, Self::Error> {
    let usr = User {
      cert_data: match bs58::decode(self.bs58.as_bytes()).into_vec() {
        Ok(cert) => cert,
        Err(err) => {
          log::error!("cannot parse base58 sting: '{}'. Error: {}", self.bs58, err);
          return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bs58 parsing error",
          ));
        }
      },
    };
    Ok(usr)
  }
}
