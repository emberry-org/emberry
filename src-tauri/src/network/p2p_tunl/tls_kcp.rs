use std::{io, sync::Arc};

use super::resolver::ClientCertResolver;
use rustls::{server::AllowAnyAuthenticatedClient, Certificate, ClientConfig, RootCertStore};
use tokio_kcp::KcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector, TlsStream};

pub async fn wrap_client(stream: KcpStream) -> TlsStream<KcpStream> {
  let mut root_store = RootCertStore::empty();

  let cert = craft_peer_cert();

  root_store.add(&cert).unwrap();

  let cert = craft_cert();
  let key = craft_key();
  let cac_resolver = Arc::new(ClientCertResolver::new(cert, key));

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_client_cert_resolver(cac_resolver);

  let server_name = dotenv!("PEER_NAME").try_into().unwrap();
  let conn = TlsConnector::from(Arc::new(config));

  TlsStream::Client(conn.connect(server_name, stream).await.unwrap())
}

pub async fn wrap_server(stream: KcpStream) -> TlsStream<KcpStream> {
  let mut client_cert_store = RootCertStore::empty();
  client_cert_store.add(&craft_peer_cert()).unwrap();

  let client_cert_verifier = AllowAnyAuthenticatedClient::new(client_cert_store);

  // Build TLS configuration.
  let tls_cfg = {
    // Load public certificate.
    let certs = vec![craft_cert()];
    // Load private key.
    let key = craft_key();
    // Do not use client certificate authentication.
    let cfg = rustls::ServerConfig::builder()
      .with_safe_defaults()
      .with_client_cert_verifier(client_cert_verifier)
      .with_single_cert(certs, key)
      .unwrap();
    std::sync::Arc::new(cfg)
  };

  let tls_acceptor = TlsAcceptor::from(tls_cfg);

  TlsStream::Server(tls_acceptor.accept(stream).await.unwrap())
}

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
  use rustls::RootCertStore;

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

  #[test]
  fn peer_cert_trust_anchor() {
    let mut store = RootCertStore::empty();
    store
      .add(&super::craft_peer_cert())
      .expect("could not use cert as trust anchor");
  }
}
