use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

use rustls::Certificate;
use smoke::messages::RoomId;
use smoke::Signal;
use smoke::User;
use tauri::EventHandler;

use lazy_static::lazy_static;
use tokio::io::BufReader;
use tokio::sync::{mpsc, oneshot};

use tokio_kcp::{KcpConfig, KcpStream};

use log::error;

use crate::data::UserIdentifier;

use self::holepunch::punch_hole;

pub mod ctrl_chnl;
mod holepunch;
mod p2p_tunl;
use p2p_tunl::{p2p_loop, tls_kcp};

lazy_static! {
  pub static ref ENV: Config = Config {
    user_cert: tls_kcp::craft_cert(),
    server_address: dotenv!("SERVER_ADDRESS"),
  };
}

/// Default kcp conf as from KcpConfig::default()
/// default is not const and therefore needs to be inlined manually
const KCP_CONF: KcpConfig = KcpConfig {
  mtu: 1400,
  nodelay: tokio_kcp::KcpNoDelayConfig {
    nodelay: false,
    interval: 40,
    resend: 0,
    nc: false,
  },
  wnd_size: (256, 256),
  session_expire: std::time::Duration::from_secs(90),
  flush_write: false,
  flush_acks_input: false,
  stream: false,
};

type ConnectionMap = HashMap<RoomId, Connection>;
pub struct Connection {
  pub send_handle: EventHandler,
  pub recv_handle: oneshot::Sender<()>,
}

pub enum RRState {
  Pending,
  Agreement,
}

pub struct Networking {
  pub chats: Mutex<ConnectionMap>,
  pub pending: Mutex<HashMap<User, RRState>>,
}

pub struct Config {
  server_address: &'static str,
  user_cert: Certificate,
}

#[derive(Clone, serde::Serialize)]
struct NewRoomPayload {
  room_id: String,
  peer_id: String,
}

pub async fn hole_punch(
  window: tauri::Window,
  app_handle: &tauri::AppHandle,
  state: tauri::State<'_, Networking>,
  room_id: RoomId,
  peer: User,
) -> tauri::Result<()> {
  /* Get the server ip from .env */

  let identity = bs58::encode(&room_id.0).into_string();

  window
    .emit("punching", &identity)
    .expect("Failed to emit WantsRoom event");

  /* Holepunch using rhizome */
  let socket = punch_hole(ENV.server_address, &room_id.0).await?;
  let addr = socket.peer_addr()?;

  let stream = KcpStream::connect_with_socket(&KCP_CONF, socket, addr)
    .await
    .map_err(|e| {
      error!("Kcp error: {}", e);
      Error::new(ErrorKind::Other, "Kcp error")
    })?;

  let stream = if peer.cert_data < ENV.user_cert.0 {
    tls_kcp::wrap_client(stream).await
  } else {
    tls_kcp::wrap_server(stream).await
  };

  let mut stream = BufReader::new(stream);

  /* Setup the send event for the frontend */
  let (sender, mut msg_rx) = mpsc::channel::<Signal>(100);
  let send_handle = window.listen(format!("send_message_{}", identity), move |e| {
    let sender = sender.clone();
    let msg = serde_json::from_str::<Signal>(
      e.payload()
        .expect("Invalid payload in send_message_<id> event"),
    )
    .expect("Invalid Json inside of payload from send_message_<id> event");
    tokio::spawn(async move { sender.send(msg).await });
  });

  /* Setup the receive loop */
  let (recv_handle, mut rx) = oneshot::channel::<()>();
  let emit_identity = identity.clone();
  let spawn_window = window.clone();
  let app_handle = app_handle.clone();
  let ident = UserIdentifier::from(&peer);
  tokio::spawn(async move {
    if let Err(err) = p2p_loop(
      &emit_identity,
      ident,
      &spawn_window,
      &app_handle,
      &mut stream,
      &mut rx,
      &mut msg_rx,
    )
    .await
    {
      log::error!(
        "receive loop for identity '{}' crashed with '{}'",
        emit_identity,
        err
      );
    }
  });

  let con = Connection {
    recv_handle,
    send_handle,
  };
  state.chats.lock().unwrap().insert(room_id.clone(), con);

  window
    .emit(
      "new-room",
      NewRoomPayload {
        room_id: identity,
        peer_id: UserIdentifier::from(&peer).bs58.into_owned(),
      },
    )
    .expect("Failed to emit WantsRoom event");
  Ok(())
}

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: RoomId) -> bool {
  // Check if the store contains the key for this chat.
  match state.chats.lock() {
    Ok(chats) => chats.contains_key(&id),
    Err(_) => false,
  }
}
