use serde::Serialize;

use crate::data::IdentifiedUserInfo;

#[derive(Serialize, Clone)]
pub struct Message<'a> {
  /// chat message that is being sent
  pub msg: &'a str,
  /// complete identified user info to allow frontend to determine the user that sent the message
  pub sender: &'a IdentifiedUserInfo,
}
