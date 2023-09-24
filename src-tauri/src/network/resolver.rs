use std::sync::Arc;

use rustls::{client::ResolvesClientCert, sign::CertifiedKey, Certificate, PrivateKey};

use crate::{data::PemfileReader, network::UserIdentification};

pub struct ClientCertResolver {
  cert_key: Arc<CertifiedKey>,
}

impl ClientCertResolver {
  pub fn new(cert: Certificate, key: &PrivateKey) -> Arc<ClientCertResolver> {
    let sig_key = rustls::sign::any_ecdsa_type(key).unwrap();
    let cert_key = CertifiedKey::new(vec![cert], sig_key);

    Arc::new(ClientCertResolver {
      cert_key: Arc::new(cert_key),
    })
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

impl From<&UserIdentification> for Arc<ClientCertResolver> {
  fn from(value: &UserIdentification) -> Self {
    ClientCertResolver::new(value.certificate.clone(), &value.private_key)
  }
}

impl TryFrom<&PemfileReader> for Arc<ClientCertResolver> {
  type Error = std::io::Error;

  /// Opens the filepath from [PemfileReader] in readonly mode and reads one
  /// X509Certificate and one PKCS8Key from it.
  /// The order in which those items are expected is: X509Certificate, PKCS8Key
  ///
  /// # Errors
  /// This function will return:</br>
  /// Any [std::io::Error] from opening/reading the file</br>
  /// [ErrorKind::InvalidData] when the items are malformed or out of order
  fn try_from(value: &PemfileReader) -> Result<Arc<ClientCertResolver>, Self::Error> {
    value
      .parse()
      .map(|(cert, key)| ClientCertResolver::new(cert, &key))
  }
}

#[cfg(test)]
mod tests {
  use std::{
    fs::remove_file,
    fs::OpenOptions,
    io::{BufWriter, Write},
    sync::Arc,
  };

  use super::ClientCertResolver;

  use super::PemfileReader;

  const FILENAME: &str = "randomfilename";
  /// Private key has been specifically genereated for use in
  /// this testcase. Its appearance in the official repository
  /// is intended and does therefore not represent a leak.
  const TEST_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgpKoKc3yAbCmIY2uB
Boz5WeTHsTKNTDkSeXp2qmxU5e+hRANCAASvuVVKbJKZjVELm56MYWOHhzPZtD2i
5fFXVVebg6+lhPwVCpSQKHIDjh6AQ0Lb08aDGSirIjCxTHTwRNaG+Iwr
-----END PRIVATE KEY-----
"#;
  const TEST_CERT: &str = r#"-----BEGIN CERTIFICATE-----
MIIBZDCCAQqgAwIBAgIJALT7z0J3GjlPMAoGCCqGSM49BAMCMCExHzAdBgNVBAMM
FnJjZ2VuIHNlbGYgc2lnbmVkIGNlcnQwIBcNNzUwMTAxMDAwMDAwWhgPNDA5NjAx
MDEwMDAwMDBaMCExHzAdBgNVBAMMFnJjZ2VuIHNlbGYgc2lnbmVkIGNlcnQwWTAT
BgcqhkjOPQIBBggqhkjOPQMBBwNCAASvuVVKbJKZjVELm56MYWOHhzPZtD2i5fFX
VVebg6+lhPwVCpSQKHIDjh6AQ0Lb08aDGSirIjCxTHTwRNaG+IwroykwJzAlBgNV
HREEHjAcggd0ZXN0aW5nghFsYWQudGVzdGluZy5sb2NhbDAKBggqhkjOPQQDAgNI
ADBFAiEAgvqO1UREpw0pO6qRBzPBl/yKm4D4BODf6zcANW0L3uQCIALKWZpUxgrR
dyOIEqrkHHicDvb8zi40n682DWVFUDQu
-----END CERTIFICATE-----
"#;

  #[test]
  fn pem_reader() {
    {
      let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(FILENAME)
        .expect("Could not create tmpfile './randomfilename'");

      let mut writer = BufWriter::new(file);
      let _ = writer
        .write(TEST_CERT.as_bytes())
        .expect("write error to './randomfilename'");
      let _ = writer
        .write(TEST_KEY.as_bytes())
        .expect("write error to './randomfilename'");
    }

    let reader = PemfileReader {
      filepath: FILENAME.into(),
    };

    let resolver = Arc::<ClientCertResolver>::try_from(&reader);

    remove_file(FILENAME).expect("!!! ERROR DELETING './randomfilename' MANUAL DELETION NECESSARY");

    resolver.expect("test cert/key malformed");
  }
}
