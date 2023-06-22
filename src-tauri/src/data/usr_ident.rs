use serde::{Deserialize, Serialize};
use smoke::User;
use tracing::error;

use super::UserInfo;

/// Wraps UserInfo with an Identifier
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IdentifiedUserInfo {
  pub identifier: UserIdentifier,
  pub info: UserInfo,
}

/// Client side local uniqe user identifier
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct UserIdentifier {
  pub bs58: String,
}

impl From<&User> for UserIdentifier {
  fn from(usr: &User) -> Self {
    UserIdentifier {
      bs58: bs58::encode(&usr.cert_data).into_string(),
    }
  }
}

impl TryInto<User> for &UserIdentifier {
  type Error = std::io::Error;
  fn try_into(self) -> Result<User, Self::Error> {
    let usr = User {
      cert_data: match bs58::decode(self.bs58.as_bytes()).into_vec() {
        Ok(cert) => cert,
        Err(err) => {
          error!("cannot parse base58 sting: '{}'. Error: {}", self.bs58, err);
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
