use std::path::PathBuf;

use super::resolver::ClientCertResolver;
use rustls_pemfile::Item::{PKCS8Key, X509Certificate};
use std::io::ErrorKind;

pub struct PemfileReader {
  pub filepath: PathBuf,
}

impl TryInto<ClientCertResolver> for PemfileReader {
  type Error = std::io::Error;

  /// Opens the filepath from [PemfileReader] in readonly mode and reads one
  /// PKCS8Key and one X509Certificate from it.
  /// THe order in which those items are expected is: PKCS8Key, X509Certificate
  ///
  /// # Errors
  /// This function will return:</br>
  /// Any [std::io::Error] from opening/reading the file</br>
  /// [ErrorKind::InvalidData] when the items are malformed or out of order
  ///
  /// # Panics
  /// If "self" was to large to be serialized within [EMB_MESSAGE_BUF_SIZE].
  /// As of writing, this is technically impossible
  fn try_into(self) -> Result<ClientCertResolver, Self::Error> {
    (&self).try_into()
  }
}

impl TryInto<ClientCertResolver> for &PemfileReader {
  type Error = std::io::Error;

  /// Opens the filepath from [PemfileReader] in readonly mode and reads one
  /// X509Certificate and one PKCS8Key from it.
  /// THe order in which those items are expected is: X509Certificate, PKCS8Key
  ///
  /// # Errors
  /// This function will return:</br>
  /// Any [std::io::Error] from opening/reading the file</br>
  /// [ErrorKind::InvalidData] when the items are malformed or out of order
  ///
  /// # Panics
  /// If "self" was to large to be serialized within [EMB_MESSAGE_BUF_SIZE].
  /// As of writing, this is technically impossible
  fn try_into(self) -> Result<ClientCertResolver, Self::Error> {
    let certfile = std::fs::OpenOptions::new()
      .read(true)
      .open(&self.filepath)?;
    let mut reader = std::io::BufReader::new(certfile);

    let cert = if let Some(X509Certificate(key)) = rustls_pemfile::read_one(&mut reader)? {
      rustls::Certificate(key)
    } else {
      return Err(Self::Error::new(
        ErrorKind::InvalidData,
        format!(
          "File: '{}' did not contain X509Certificate as first element",
          self.filepath.to_string_lossy()
        ),
      ));
    };

    let key = if let Some(PKCS8Key(key)) = rustls_pemfile::read_one(&mut reader)? {
      rustls::PrivateKey(key)
    } else {
      return Err(Self::Error::new(
        ErrorKind::InvalidData,
        format!(
          "File: '{}' did not contain PKCS8Key as second element",
          self.filepath.to_string_lossy()
        ),
      ));
    };

    let resolver = ClientCertResolver::new(cert, key);
    Ok(resolver)
  }
}

#[cfg(test)]
mod tests {
  use std::{
    fs::remove_file,
    fs::OpenOptions,
    io::{BufWriter, Error, Write},
  };

  use crate::network::p2p_tunl::resolver::ClientCertResolver;

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

    let resolver: Result<ClientCertResolver, Error> = reader.try_into();

    remove_file(FILENAME).expect("!!! ERROR DELETING './randomfilename' MANUAL DELETION NECESSARY");

    resolver.expect("test cert/key malformed");
  }
}
