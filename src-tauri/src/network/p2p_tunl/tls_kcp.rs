use std::{
  io::{self, ErrorKind},
  sync::Arc,
};

use super::resolver::ClientCertResolver;
use crate::data::config::PEM_DATA;
use once_cell::sync::Lazy;
use rustls::{server::AllowAnyAuthenticatedClient, Certificate, ClientConfig, RootCertStore};
use tokio_kcp::KcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector, TlsStream};

static CAC_RESOLVER: Lazy<Arc<ClientCertResolver>> = Lazy::new(cacr);

fn cacr() -> Arc<ClientCertResolver> {
  //                    we can unsafe unwrap here because we know that PEM_DATA is not None because the ctrl_chnl loop
  //                    only starts if PEM_DATA is Some() (from which this func is called)
  let (cert, key) = unsafe { &PEM_DATA.as_ref().unwrap_unchecked() };
  Arc::new(ClientCertResolver::new(cert.clone(), key.clone()))
}

pub async fn wrap_client(
  stream: KcpStream,
  peer_cert: &Certificate,
) -> Result<TlsStream<KcpStream>, io::Error> {
  let mut root_store = RootCertStore::empty();

  root_store.add(peer_cert).map_err(|err| {
    log::error!("Error creating root store for peer_cert, Err: '{}'", err);
    io::Error::new(ErrorKind::InvalidData, "Invalid peer cert")
  })?;

  let cac_resolver = CAC_RESOLVER.clone();

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
) -> Result<TlsStream<KcpStream>, io::Error> {
  let mut client_cert_store = RootCertStore::empty();
  client_cert_store.add(peer_cert).map_err(|err| {
    log::error!("Error creating root store for peer_cert, Err: '{}'", err);
    io::Error::new(ErrorKind::InvalidData, "Invalid peer cert")
  })?;

  let client_cert_verifier = AllowAnyAuthenticatedClient::new(client_cert_store);
  //                    we can unsafe unwrap here because we know that PEM_DATA is not None because the ctrl_chnl loop
  //                    only starts if PEM_DATA is Some() (from which this func is called)
  let (cert, key) = unsafe { &PEM_DATA.as_ref().unwrap_unchecked() };

  // Build TLS configuration.
  let tls_cfg = {
    // Load public certificate.
    let certs = vec![cert.clone()];
    // Load private key.
    let key = key.clone();
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
