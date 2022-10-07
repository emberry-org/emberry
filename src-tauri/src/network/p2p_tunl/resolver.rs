use std::sync::Arc;

use rustls::{client::ResolvesClientCert, sign::CertifiedKey, Certificate, PrivateKey};

pub struct ClientCertResolver {
  cert_key: Arc<CertifiedKey>,
}

impl ClientCertResolver {
  pub fn new(cert: Certificate, key: PrivateKey) -> ClientCertResolver {
    let sig_key = rustls::sign::any_ecdsa_type(&key).unwrap();
    let cert_key = CertifiedKey::new(vec![cert], sig_key);

    ClientCertResolver {
      cert_key: Arc::new(cert_key),
    }
  }
}

impl ResolvesClientCert for ClientCertResolver {
  fn resolve(
    &self,
    _acceptable_issuers: &[&[u8]],
    _sigschemes: &[rustls::SignatureScheme],
  ) -> Option<std::sync::Arc<rustls::sign::CertifiedKey>> {
    Some(self.cert_key.clone())
  }

  fn has_certs(&self) -> bool {
    true
  }
}
