mod certs;
mod messages;
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

  let res = run_channel_result(&window, rx, tls, net, &rc).await;

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
    NoRoute(usr) => {
      if net.pending.lock().unwrap().remove(&usr) {
        //if there was a pending signal fontend
        todo!("update visual (user offline)");
      }
    }
    WantsRoom(usr) => {
      //check before emiting the event
      window
      .emit("room-request", usr)
      .expect("Failed to emit WantsRoom event");
      todo!("check if there is already a req. pending. & investigate what strategy to use to resolve this collision");
    },
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

async fn request_room(usr: User, net: &tauri::State<'_, Networking>) {
  if !net.pending.lock().unwrap().insert(usr) {
    // return if the request is already pending
    return;
  }
  todo!("send room request to server");
}

async fn accept_room(usr: User, accepted: bool, net: &tauri::State<'_, Networking>) {
  if accepted{
  if !net.pending.lock().unwrap().insert(usr) {
    // return if the request is already pending
    return;
  }
  todo!("send room affirmation to server");

  }else{
    todo!("send room decline to the server");
    todo!("investigate how to handle if already pending in this situation");
  }
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
    eprintln!("Rhizome just sent a malicious room opening packet (this should not happen)");
    return Ok(());
  }

  if let Some(room_id) = room_id {
    hole_punch(window, net_state, room_id).await?;
    todo!("reinvestigate if this works as intended");
  }
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
