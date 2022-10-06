use std::io;

use rustls::Certificate;

// Load public certificate from file.
pub fn craft_peer_cert() -> rustls::Certificate {
  let mut reader = io::BufReader::new(dotenv!("PEER_CERT").as_bytes());

  // Load and return a single private key.
  // use unwrap as we test this during compile time using tests and its is uneffected by runtime
  if let rustls_pemfile::Item::X509Certificate(key) =
    rustls_pemfile::read_one(&mut reader).unwrap().unwrap()
  {
    Certificate(key)
  } else {
    panic!("no parsable certificate in dotenv");
  }
}

// Load public certificate from file.
pub fn craft_cert() -> rustls::Certificate {
  let mut reader = io::BufReader::new(dotenv!("USER_CERT").as_bytes());

  // Load and return a single private key.
  // use unwrap as we test this during compile time using tests and its is uneffected by runtime
  if let rustls_pemfile::Item::X509Certificate(key) =
    rustls_pemfile::read_one(&mut reader).unwrap().unwrap()
  {
    Certificate(key)
  } else {
    panic!("no parsable certificate in dotenv");
  }
}

// Load public certificate from file.
pub fn craft_key() -> rustls::PrivateKey {
  let mut reader = io::BufReader::new(dotenv!("USER_KEY").as_bytes());

  // Load and return a single private key.
  // use unwrap as we test this during compile time using tests and its is uneffected by runtime
  if let rustls_pemfile::Item::PKCS8Key(key) =
    rustls_pemfile::read_one(&mut reader).unwrap().unwrap()
  {
    rustls::PrivateKey(key)
  } else {
    panic!("no parsable certificate in dotenv");
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn user_key_creation() {
    super::craft_key();
  }


  #[test]
  fn user_cert_creation() {
    super::craft_cert();
  }

  #[test]
  fn peer_cert_creation() {
    super::craft_peer_cert();
  }
}
