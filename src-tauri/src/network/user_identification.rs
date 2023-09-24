use rustls::{Certificate, PrivateKey};

pub struct UserIdentification {
  pub certificate: Certificate,
  pub private_key: PrivateKey,
}
