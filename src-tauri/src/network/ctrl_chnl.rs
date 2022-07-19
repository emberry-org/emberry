use std::{sync::Arc, io};

use rustls::{ClientConfig, RootCertStore, ServerName};
use tauri::utils::platform;
use tokio::{io::{AsyncBufReadExt, BufReader, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::TlsConnector;

use crate::network::certs;

#[tauri::command(async)]
pub async fn connect() -> tauri::Result<()> {
  let mut root_store = RootCertStore::empty();

  let cert = certs::craft();
  root_store.add(&cert).unwrap(); // unwrap is safe here as invalid certificates wont be returned from load

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();

  let server_name = get_server_name();
  let conn = TlsConnector::from(Arc::new(config));
  let sock = TcpStream::connect("127.0.0.1:9999").await.unwrap();
  let mut tls = BufReader::new(conn.connect(server_name, sock).await.unwrap());

  let mut plaintext = String::new();
  tls.read_line(&mut plaintext).await.unwrap();
  if plaintext != "rhizome v0.2.0\n" {
    return Err(tauri::Error::Io(io::Error::new(io::ErrorKind::Unsupported, "Server did greet with rhizome signature")));
  }

  tls.write_all(dotenv!("PUBLIC_KEY").as_bytes()).await?;

  Ok(())
}

#[inline]
fn get_server_name() -> ServerName {
  dotenv!("SERVER_DOMAIN").try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::get_server_name;

  #[test]
  fn server_name_creation() {
    get_server_name();
  }
}
