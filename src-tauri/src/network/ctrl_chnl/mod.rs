mod certs;
mod messages;
mod state;

use std::{
  io::{self, ErrorKind},
  sync::Arc,
};

pub use messages::EmberryMessage;
use rustls::{ClientConfig, RootCertStore, ServerName};
use smoke::messages::RhizMessage;
pub use state::RhizomeConnection;
use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpStream,
};
use tokio::{
  select,
  sync::mpsc::{self, Receiver},
};
use tokio_rustls::{client::TlsStream, TlsConnector};

use self::messages::RhizomeMessage;

#[tauri::command(async)]
pub async fn connect(
  window: tauri::Window,
  state: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let mut root_store = RootCertStore::empty();

  let cert = certs::craft();
  root_store.add(&cert).unwrap(); // unwrap is safe here as invalid certificates wont be returned from load

  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();

  let server_name = get_server_name();
  let conn = TlsConnector::from(Arc::new(config));
  let sock = TcpStream::connect(dotenv!("CONTROL_ADDRESS"))
    .await
    .unwrap();
  let mut tls = BufReader::new(conn.connect(server_name, sock).await.unwrap());

  let mut plaintext = String::new();
  tls.read_line(&mut plaintext).await.unwrap();
  if plaintext != "rhizome v0.2.0\n" {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Server did greet with rhizome signature",
    )));
  }

  tls.write_all(dotenv!("PUBLIC_KEY").as_bytes()).await?;

  let (tx, rx) = mpsc::channel::<EmberryMessage>(25);

  state.channel.lock().unwrap().replace(tx);

  run_channel(window, rx, tls).await?;
  Ok(())
}

async fn run_channel(
  window: tauri::Window,
  rx: Receiver<EmberryMessage>,
  tls: BufReader<TlsStream<TcpStream>>,
) -> io::Result<()> {
  if let Err(err) = run_channel_result(&window, rx, tls).await {
    window
      .emit("rhizome_connection", RhizomeMessage::Error(err.to_string()))
      .expect("failed to emit tauri event");
    return Err(err);
  }

  Ok(())
}

async fn run_channel_result(
  window: &tauri::Window,
  mut rx: Receiver<EmberryMessage>,
  mut tls: BufReader<TlsStream<TcpStream>>,
) -> io::Result<()> {
  let mut buf = vec![];
  loop {
    select! {
      Some(msg) = rx.recv() => {
        match msg {
            EmberryMessage::Direct(msg) => msg.send_with(&mut tls).await?,
            EmberryMessage::Close() => return Ok(()),
        }
      }
      msg = RhizMessage::recv_with(&mut tls, &mut buf) => {
        match msg?{
          RhizMessage::Shutdown() => return Ok(()),
          RhizMessage::HasRoute(usr) => todo!("update visual (attempt is pending)"),
          RhizMessage::NoRoute(usr) => todo!("update visual (user offline)"),
          RhizMessage::WantsRoom(usr) => window.emit("room-request", usr).expect("Failed to emit WantsRoom event"),
          RhizMessage::AcceptedRoom(usr) => {todo!("holepunchin"); todo!("update visual (chat room opens)");},
          RhizMessage::ServerError(err) => return Err(io::Error::new(ErrorKind::Other, err)),
        }
      }
    }
  }
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
