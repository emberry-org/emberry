use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

use smoke::messages::RoomId;
use smoke::Signal;
use smoke::{PubKey, User};
use tauri::EventHandler;

use tokio::io::BufReader;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::{mpsc, oneshot};

use kcp::Error as KcpError;
use tokio_kcp::{KcpConfig, KcpStream};

use log::{error, trace};

use self::holepunch::punch_hole;

pub mod ctrl_chnl;
mod holepunch;

pub const ENV: Config = Config {
  public_key: dotenv!("PUBLIC_KEY"),
  server_address: dotenv!("SERVER_ADDRESS"),
};

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
  stream: true,
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

pub struct Config<'a> {
  server_address: &'a str,
  public_key: &'a str,
}

#[derive(Clone, serde::Serialize)]
struct MessageRecievedPayload {
  message: Signal,
}

pub async fn hole_punch(
  window: tauri::Window,
  state: tauri::State<'_, Networking>,
  room_id: RoomId,
  other_key: PubKey,
) -> tauri::Result<()> {
  /* Get the server ip from .env */

  let identity = bs58::encode(&room_id.0).into_string();

  window
    .emit("punching", &identity)
    .expect("Failed to emit WantsRoom event");

  /* Holepunch using rhizome */
  let socket = punch_hole(ENV.server_address, &room_id.0).await?;

  let mut stream = wrap_kcp(socket, other_key.as_ref() < ENV.public_key.as_bytes())
    .await
    .map_err(|e| {
      error!("Kcp error: {}", e);
      Error::new(ErrorKind::Other, "Kcp error")
    })?;

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
  let mut buf = Vec::new();
  let emit_identity = identity.clone();
  let spawn_window = window.clone();
  tokio::spawn(async move {
    let event_name = format!("message_recieved_{}", emit_identity);
    loop {
      select! {
        Ok(msg) = Signal::recv_with(&mut stream, &mut buf) => {
          /* Emit the message_recieved event when a message is recieved */
          trace!("Received message: {:?}", msg);
          spawn_window
            .emit(&event_name, MessageRecievedPayload { message: msg })
            .expect("Failed to emit event");
        },
        Ok(_) = &mut rx => {
          break;
        },
        Some(msg) = msg_rx.recv() => {
          if let Err(e) = msg.send_with(&mut stream).await{
            error!("Kcp send error: {}", e);
            break;
          }
        },
      }
    }
  });

  let con = Connection {
    recv_handle,
    send_handle,
  };
  state.chats.lock().unwrap().insert(room_id.clone(), con);

  window
    .emit("new-room", &identity)
    .expect("Failed to emit WantsRoom event");
  Ok(())
}

/// Creates a new KcpStream wrapping the given socket.
/// Uses the underlying wrap_client/wrap_server methods from [KcpStream]
/// depending on the "is_client" argument.
///
/// # Cancel safety
/// This method is probably not cancellation safe and is therefore to be treated
/// as if it were not. If it is used as the event in a tokio::select statement
/// and some other branch completes first, the kcp may be partially initialized
/// and subsequent calls do not reset the remote end of the connection.
///
/// # Errors
/// This function will return:</br>
/// The first error returned by calling wrap_client/wrap_server on [KcpStream]
async fn wrap_kcp(socket: UdpSocket, is_client: bool) -> Result<BufReader<KcpStream>, KcpError> {
  if is_client {
    trace!("client mode");
    let mut tmp = BufReader::new(KcpStream::wrap_client(&KCP_CONF, socket).await?);
    // send kap as initializer for the wrap server
    Signal::Kap
      .send_with(&mut tmp)
      .await
      .map_err(KcpError::IoError)?;
    Ok(tmp)
  } else {
    trace!("server mode");
    Ok(BufReader::new(
      KcpStream::wrap_server(&KCP_CONF, socket).await?,
    ))
  }
}

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: RoomId) -> bool {
  // Check if the store contains the key for this chat.
  match state.chats.lock() {
    Ok(chats) => chats.contains_key(&id),
    Err(_) => false,
  }
}
