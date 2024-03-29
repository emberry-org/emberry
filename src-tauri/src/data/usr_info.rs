use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UserInfo {
  pub username: String,
  pub relation: UserRelation,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum UserRelation {
  Undefined,
  Known = 1,
  Friend = 2,
  Stranger = 3,
  Local = 255,
}

impl From<u8> for UserRelation {
  fn from(value: u8) -> Self {
    match value {
      1 => Self::Known,
      2 => Self::Friend,
      3 => Self::Stranger,
      255 => Self::Local,
      _ => Self::Undefined,
    }
  }
}
