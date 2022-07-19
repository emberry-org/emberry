use std::io;

use rustls::Certificate;

// Load public certificate from file.
pub fn craft() -> rustls::Certificate {
  let mut reader = io::BufReader::new(dotenv!("CERT").as_bytes());

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

#[cfg(test)]
mod tests {

  #[test]
  fn server_cert_creation() {
    super::craft();
  }
}
