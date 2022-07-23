mod certs;
mod messages;
pub mod responses;
mod state;

use std::{
  io::{self, ErrorKind},
  sync::Arc,
};

pub use self::state::RwOption;
pub use messages::EmberryMessage;
use rustls::{ClientConfig, RootCertStore, ServerName};
use smoke::messages::RhizMessage::{self, *};
use smoke::messages::RoomId;
pub use state::RhizomeConnection;
pub use state::State;
use tauri::Window;
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

use super::Networking;

#[tauri::command(async)]
pub async fn connect(
  window: tauri::Window,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  if rc.read().await.is_some() {
    return Err(tauri::Error::Io(io::Error::new(
      io::ErrorKind::Unsupported,
      "Already connected to the server",
    )));
  }

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

  let conn = State { channel: tx };
  rc.write().await.replace(conn);

  let res = match run_channel_result(&window, rx, tls, net, &rc).await {
    Err(err) => {
      window
        .emit("rhizome_connection", RhizomeMessage::Error(err.to_string()))
        .expect("failed to emit tauri event");
      Err(err)
    }
    Ok(_) => Ok(()),
  };

  *rc.write().await = None;

  res
}

async fn run_channel_result<'a>(
  window: &tauri::Window,
  mut rx: Receiver<EmberryMessage>,
  mut tls: BufReader<TlsStream<TcpStream>>,
  net: tauri::State<'_, Networking>,
  rc: &tauri::State<'a, RhizomeConnection>,
) -> tauri::Result<()> {
  let mut buf = vec![];
  loop {
    select! {
      Some(msg) = rx.recv() => {
        match msg {
            EmberryMessage::Direct(msg) => msg.send_with(&mut tls).await?,
            EmberryMessage::Close() => return Ok(()),
        }
      }
      msg = RhizMessage::recv_with(&mut tls, &mut buf) => handle_rhiz_msg(msg, window, &net).await?
    }
  }
}

async fn handle_rhiz_msg(
  msg: Result<RhizMessage, io::Error>,
  window: &Window,
  net: &tauri::State<'_, Networking>,
) -> tauri::Result<()> {
  match msg? {
    Shutdown() => return Ok(()),
    HasRoute(usr) => window
      .emit("room-req-pending", usr)
      .expect("Failed to emit HasRoute"),
    NoRoute(usr) => todo!("update visual (user offline)"),
    WantsRoom(usr) => window
      .emit("room-request", usr)
      .expect("Failed to emit WantsRoom event"),
    AcceptedRoom(id) => try_holepunch(window.clone(), net.clone(), id).await?,
    ServerError(err) => {
      return Err(tauri::Error::Io(io::Error::new(
        ErrorKind::Other,
        format!("Rhizome internal error: {}", err),
      )))
    }
  };

  Ok(())
}

async fn try_holepunch(
  window: tauri::Window,
  net_state: tauri::State<'_, Networking>,
  room_id: Option<RoomId>,
) -> tauri::Result<()> {
  todo!("holepunchin");
  todo!("update visual (chat room opens)");
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
