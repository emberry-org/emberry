use std::{sync::Arc};

use super::resolver::ClientCertResolver;
use rustls::{server::AllowAnyAuthenticatedClient, Certificate, ClientConfig, RootCertStore};
use tokio_kcp::KcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector, TlsStream};
use crate::data::config::PEM_DATA;
use lazy_static::lazy_static;

lazy_static! {
  static ref CAC_RESOLVER: Option<Arc<ClientCertResolver>> = maybe_cacr();
}

fn maybe_cacr()-> Option<Arc<ClientCertResolver>> {
  let (cert, key) = PEM_DATA.as_ref()?;  
  Some(Arc::new(ClientCertResolver::new(cert.clone(), key.clone())))
}

pub async fn wrap_client(stream: KcpStream, peer_cert: &Certificate) -> TlsStream<KcpStream> {
  let mut root_store = RootCertStore::empty();

  root_store.add(peer_cert).unwrap();

  let (cert, key) = (&*PEM_DATA).as_ref().unwrap();
  let cac_resolver = CAC_RESOLVER.as_ref().unwrap().clone();

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_client_cert_resolver(cac_resolver);

  let server_name = dotenv!("PEER_NAME").try_into().unwrap();
  let conn = TlsConnector::from(Arc::new(config));

  TlsStream::Client(conn.connect(server_name, stream).await.unwrap())
}

pub async fn wrap_server(stream: KcpStream, peer_cert: &Certificate) -> TlsStream<KcpStream> {
  let mut client_cert_store = RootCertStore::empty();
  client_cert_store.add(peer_cert).unwrap();

  let client_cert_verifier = AllowAnyAuthenticatedClient::new(client_cert_store);
  let (cert, key) = PEM_DATA.as_ref().unwrap();

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
      .unwrap();
    std::sync::Arc::new(cfg)
  };

  let tls_acceptor = TlsAcceptor::from(tls_cfg);

  TlsStream::Server(tls_acceptor.accept(stream).await.unwrap())
}
