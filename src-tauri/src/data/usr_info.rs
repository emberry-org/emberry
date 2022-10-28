use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UserInfo {
  pub username: String,
  pub relation: UserRelation,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum UserRelation {
  Known = 0,
  Friend = 1,
  Stranger = 2,
  Undefined,
}

impl From<u8> for UserRelation {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::Known,
      1 => Self::Friend,
      2 => Self::Stranger,
      _ => Self::Undefined,
    }
  }
}
