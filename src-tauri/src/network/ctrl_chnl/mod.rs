mod certs;
mod channel;
mod messages;
pub mod requests;
pub mod responses;
mod room_creation;
mod state;

use std::{
  io::{self, ErrorKind},
  sync::Arc,
  time::Instant,
};

use crate::data::config;

use self::channel::ControlChannel;
pub use self::state::RwOption;
use log::error;
pub use messages::EmberryMessage;
use rustls::{ClientConfig, RootCertStore, ServerName};
pub use state::RhizomeConnection;
pub use state::State;
use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpStream,
  sync::mpsc,
};
use tokio_rustls::TlsConnector;

use super::{Networking, RRState};

#[tauri::command(async)]
pub async fn connect(
  window: tauri::Window,
  app_handle: tauri::AppHandle,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let start = Instant::now();
  if rc.read().await.is_some() {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Already connected to the server",
    )));
  }

  let server_cert = certs::craft();
  let client_cert = config::PEM.parse();
  let (client_cert, _) = match client_cert {
    Ok(data) => data,
    Err(err) => {
      return Err(tauri::Error::Io(io::Error::new(
        io::ErrorKind::Unsupported,
        format!("Identity needed to connect with rhizome. Unable to parse Identity file: {err}"),
      )))
    }
  };

  let mut root_store = RootCertStore::empty();
  root_store
    .add(&server_cert)
    .map_err(|err| tauri::Error::Io(io::Error::new(ErrorKind::InvalidData, err)))?;

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();

  let server_name = get_server_name();
  let conn = TlsConnector::from(Arc::new(config));
  let sock = TcpStream::connect(dotenv!("CONTROL_ADDRESS")).await?;
  let mut tls = BufReader::new(conn.connect(server_name, sock).await?);

  let mut plaintext = String::new();
  tls.read_line(&mut plaintext).await?;
  if plaintext != "rhizome v0.3.0\n" {
    error!("invalid rhizome version");
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Server did greet with rhizome signature",
    )));
  }
  window
    .emit("rz-con", start.elapsed().as_millis() as u64)
    .expect("Failed to emit event");

  let cobs_cert = match postcard::to_vec_cobs::<Vec<u8>, 1024>(&client_cert.0) {
    Ok(cobs) => cobs,
    Err(err) => {
      error!("Error serializing USER_CERT in 1024 bytes: {}", err);
      return Err(tauri::Error::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "cannot serialize USER_CERT",
      )));
    }
  };
  tls.write_all(&cobs_cert).await?;

  let (tx, rx) = mpsc::channel::<EmberryMessage>(25);

  let conn = State { channel: tx };
  rc.write().await.replace(conn);

  let chnl = ControlChannel {
    window: &window,
    app: &app_handle,
    rx,
    tls,
    net,
    rc: &rc,
    identity: client_cert,
  };

  let res = chnl.spin().await;

  *rc.write().await = None;

  window
    .emit("rz-dc", start.elapsed().as_millis() as u64)
    .expect("Failed to emit event");

  res
}

#[inline]
fn get_server_name() -> ServerName {
  // unwrap is ok here as this is tested with tests on build time
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
