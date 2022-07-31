mod certs;
mod messages;
pub mod requests;
pub mod responses;
mod state;

use std::{
  io::{self, ErrorKind},
  sync::Arc,
};

use crate::network::hole_punch;

pub use self::state::RwOption;
pub use messages::EmberryMessage;
use rustls::{ClientConfig, RootCertStore, ServerName};
use serde_json::json;
use smoke::messages::RoomId;
use smoke::{
  messages::RhizMessage::{self, *},
  User,
};
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
  let sock = TcpStream::connect(dotenv!("CONTROL_ADDRESS")).await?;
  let mut tls = BufReader::new(conn.connect(server_name, sock).await?);

  let mut plaintext = String::new();
  tls.read_line(&mut plaintext).await?;
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

  let res = run_channel_result(&window, rx, tls, net).await;

  *rc.write().await = None;

  res
}

async fn run_channel_result<'a>(
  window: &tauri::Window,
  mut rx: Receiver<EmberryMessage>,
  mut tls: BufReader<TlsStream<TcpStream>>,
  net: tauri::State<'_, Networking>,
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
    HasRoute(usr) => {
      let pending = net.pending.lock().unwrap().remove(&usr);
      window
        .emit("has-route", json!({ "pending": pending, "usr": usr, }))
        .expect("Failed to emit NoRoute")
    }
    NoRoute(usr) => {
      let pending = net.pending.lock().unwrap().remove(&usr);
      window
        .emit("no-route", json!({ "pending": pending, "usr": usr, }))
        .expect("Failed to emit NoRoute")
    }
    WantsRoom(usr) => {
      if net.pending.lock().unwrap().remove(&usr) {
        window
          .emit("wants-room", usr)
          .expect("Failed to emit WantsRoom event");
      } else {
        // Here we get a WantsRoom while we already want a room with them
        todo!("form a strategy to use to resolve this collision");
      }
    }
    AcceptedRoom(id, usr) => try_holepunch(window.clone(), net.clone(), id, usr).await?,
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
  usr: User,
) -> tauri::Result<()> {
  let pending = net_state.pending.lock().unwrap().remove(&usr);
  if !pending {
    // This is rather weak protection as a compromized rhizome server could still just send a different room id with a valid user
    // Room id procedure is subject to change in the future. (plan is to use cryptographic signatures to mitigated unwanted ip leak)
    const MSG: &str = "Rhizome just sent a malicious room opening packet (this should not happen)";
    window
      .emit("warning", MSG)
      .expect("Failed to emit WantsRoom event");

    eprintln!("{}", MSG);
    return Ok(());
  }

  if let Some(room_id) = room_id {
    hole_punch(window, net_state, room_id).await?;
  }

  Ok(())
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
