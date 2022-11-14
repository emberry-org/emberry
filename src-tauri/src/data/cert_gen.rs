use rcgen::generate_simple_self_signed;
use std::fs::OpenOptions;
use std::{io, path::PathBuf};
use std::io::Write;
#[allow(unused_imports)] // doc import
use rustls::{PrivateKey, Certificate};

/// Generates a new pair of self signed [`X509 Certificate`](Certificate) and [`PKCS8 Key`](PrivateKey)
/// and stores them in pemfile in the order []
///
/// Certificate will have "rhizome" as subject name
///
/// # Errors
/// Will return any errors from creating and writing "certfile"/"keyfile"
pub fn generate_cert(pemfile: &PathBuf) -> io::Result<()> {
    let subject_alt_names = vec!["embery_user".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names).unwrap();

    let mut pemfile = OpenOptions::new().create(true).write(true).open(pemfile)?;
    pemfile.write_all(cert.serialize_pem().unwrap().as_bytes())?;

    pemfile.write_all(cert.serialize_private_key_pem().as_bytes())?;
    Ok(())
}