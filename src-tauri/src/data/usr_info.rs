pub struct UserInfo{
  pub username: String,
  pub relation: UserRelation,
}

#[derive(Clone, Copy)]
pub enum UserRelation{
  Friend,
  Known,
}