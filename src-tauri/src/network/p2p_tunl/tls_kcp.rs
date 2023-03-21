use std::{
  io::{self, ErrorKind},
  sync::Arc,
};

use crate::network::UserIdentification;

use super::resolver::ClientCertResolver;
use rustls::{server::AllowAnyAuthenticatedClient, Certificate, ClientConfig, RootCertStore};
use tokio_kcp::KcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector, TlsStream};

pub async fn wrap_client(
  stream: KcpStream,
  peer_cert: &Certificate,
  identification: &UserIdentification,
) -> Result<TlsStream<KcpStream>, io::Error> {
  let mut root_store = RootCertStore::empty();

  root_store.add(peer_cert).map_err(|err| {
    log::error!("Error creating root store for peer_cert, Err: '{}'", err);
    io::Error::new(ErrorKind::InvalidData, "Invalid peer cert")
  })?;

  let cac_resolver = Arc::<ClientCertResolver>::from(identification);

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_client_cert_resolver(cac_resolver);

  let cert_name = "emberry_user".try_into().unwrap(); // FIXME get peername from certificate
  let conn = TlsConnector::from(Arc::new(config));

  Ok(TlsStream::Client(conn.connect(cert_name, stream).await?))
}

pub async fn wrap_server(
  stream: KcpStream,
  peer_cert: &Certificate,
  identification: &UserIdentification,
) -> Result<TlsStream<KcpStream>, io::Error> {
  let mut client_cert_store = RootCertStore::empty();
  client_cert_store.add(peer_cert).map_err(|err| {
    log::error!("Error creating root store for peer_cert, Err: '{}'", err);
    io::Error::new(ErrorKind::InvalidData, "Invalid peer cert")
  })?;

  let client_cert_verifier = AllowAnyAuthenticatedClient::new(client_cert_store);

  // Build TLS configuration.
  let tls_cfg = {
    // Load public certificate.
    let certs = vec![identification.certificate.clone()];
    // Load private key.
    let key = identification.private_key.clone();
    // Do not use client certificate authentication.
    let cfg = rustls::ServerConfig::builder()
      .with_safe_defaults()
      .with_client_cert_verifier(client_cert_verifier)
      .with_single_cert(certs, key)
      .map_err(|err| {
        log::error!("Error creating rustls::ServerConfig '{}'", err);
        io::Error::new(ErrorKind::InvalidData, "Invalid local pem")
      })?;
    std::sync::Arc::new(cfg)
  };

  let tls_acceptor = TlsAcceptor::from(tls_cfg);

  Ok(TlsStream::Server(tls_acceptor.accept(stream).await?))
}
