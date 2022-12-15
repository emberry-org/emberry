use log::info;
use rcgen::generate_simple_self_signed;
#[allow(unused_imports)] // doc import
use rustls::{Certificate, PrivateKey};
use std::fs::{DirBuilder, OpenOptions};
use std::io::{ErrorKind, Write};
use std::{io, path::PathBuf};

/// Generates a new pair of self signed [`X509 Certificate`](Certificate) and [`PKCS8 Key`](PrivateKey)
/// and stores them in pemfile in the order Certificate, PrivateKey
///
/// Certificate will have "emberry_user" as subject name
///
/// # Errors
/// Will return any errors from creating and writing "pemfile"
pub fn generate_cert(pemfile: &PathBuf) -> io::Result<()> {
  let subject_alt_names = vec!["embery_user".to_string()];
  let cert = generate_simple_self_signed(subject_alt_names).unwrap();

  // since every path that is a filepath has at least "/" as parent unchecked unwrap is ok here
  let dir = match pemfile.parent() {
    Some(dir) => dir,
    None => {
      return Err(io::Error::new(
        ErrorKind::Other,
        "cannot use '/' as path for a file",
      ))
    }
  };

  DirBuilder::new().recursive(true).create(dir)?;

  let mut pemfile = OpenOptions::new().create(true).write(true).open(pemfile)?;
  info!("[created/overwritten] file: {pemfile:?}");

  pemfile.write_all(cert.serialize_pem().unwrap().as_bytes())?;

  pemfile.write_all(cert.serialize_private_key_pem().as_bytes())?;
  Ok(())
}
