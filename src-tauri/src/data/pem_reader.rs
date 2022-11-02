use std::{borrow::Cow, path::PathBuf};

use crate::data::UserIdentifier;

use rustls_pemfile::Item::X509Certificate;
use std::io::ErrorKind;

pub struct PemfileReader {
  pub filepath: PathBuf,
}

/// Opens the filepath from [PemfileReader] in readonly mode and reads one
/// X509Certificate and one PKCS8Key from it.
/// The order in which those items are expected is: X509Certificate, PKCS8Key
///
/// # Errors
/// This function will return:</br>
/// Any [std::io::Error] from opening/reading the file</br>
/// [ErrorKind::InvalidData] when the items are malformed or out of order
impl<'a> TryInto<UserIdentifier<'a>> for &PemfileReader {
  type Error = std::io::Error;

  fn try_into(self) -> Result<UserIdentifier<'a>, Self::Error> {
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

    Ok(UserIdentifier {
      bs58: Cow::Owned(bs58::encode(cert.0).into_string()),
    })
  }
}
