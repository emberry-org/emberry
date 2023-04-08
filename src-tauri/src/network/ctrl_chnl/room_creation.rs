use std::io::{Error, ErrorKind};
use std::thread;

use rustls::Certificate;
use smoke::messages::RoomId;
use smoke::Signal;
use smoke::User;

use tokio::io::BufReader;
use tokio::sync::{mpsc, oneshot};

use tokio_kcp::{KcpConfig, KcpStream};

use log::error;

use crate::data::UserIdentifier;
use crate::network::RRState;
use crate::network::UserIdentification;
use crate::network::{Connection, Networking};

use super::super::holepunch::punch_hole;
use super::super::p2p_tunl::{p2p_loop, tls_kcp};

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

#[derive(Clone, serde::Serialize)]
struct NewRoomPayload {
  room_id: String,
  peer_id: String,
}

pub async fn try_holepunch(
  window: tauri::Window,
  app_handle: &tauri::AppHandle,
  net_state: tauri::State<'_, Networking>,
  room_id: Option<RoomId>,
  usr: &User,
  identification: &UserIdentification,
  priority: bool,
) -> tauri::Result<()> {
  if let Some(room_id) = room_id {
    if net_state.pending.lock().unwrap().remove(usr).is_some() {
      // only hole punch if there is a connection pending
      hole_punch(
        window,
        app_handle,
        net_state,
        room_id,
        usr,
        identification,
        priority,
      )
      .await?;
    } else {
      // This is rather weak protection as a compromized rhizome server could still just send a different room id with a valid user
      // Room id procedure is subject to change in the future. (plan is to use cryptographic signatures to mitigated unwanted ip leak)
      return Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "Rhizome just sent a malicious room opening packet (this should not happen)",
      )));
    }
  } else {
    let mut guard = net_state.pending.lock().unwrap();
    if let Some(kv) = guard.get_key_value(usr) {
      match kv.1 {
        RRState::Agreement => return Ok(()), // We got the edgecase of colliding requests, throw away this one
        RRState::Pending => {
          guard.remove(usr); // This case is a normal rejection
        }
      }
    }
  }

  Ok(())
}

async fn hole_punch(
  window: tauri::Window,
  app_handle: &tauri::AppHandle,
  state: tauri::State<'_, Networking>,
  room_id: RoomId,
  peer: &User,
  identification: &UserIdentification,
  priority: bool,
) -> tauri::Result<()> {
  /* Get the server ip from .env */

  let identity = bs58::encode(&room_id.0).into_string();

  window
    .emit("punching", &identity)
    .expect("Failed to emit punching event");

  /* Holepunch using rhizome */
  let socket = punch_hole(dotenv!("SERVER_ADDRESS"), &room_id.0).await?;
  let addr = socket.peer_addr()?;

  let stream = KcpStream::connect_with_socket(&KCP_CONF, socket, addr)
    .await
    .map_err(|e| {
      error!("Kcp error: {}", e);
      Error::new(ErrorKind::Other, "Kcp error")
    })?;

  let peer_cert = Certificate(peer.cert_data.clone());
  let stream = if priority {
    tls_kcp::wrap_client(stream, &peer_cert, identification).await
  } else {
    tls_kcp::wrap_server(stream, &peer_cert, identification).await
  };

  let stream = stream.map_err(|err| {
    log::error!("Unable to start TLS on KCP stream, Err: '{}'", err);
    Error::new(ErrorKind::Other, "TLS could not be established")
  })?;

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
  let ident = UserIdentifier::from(peer);
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
        peer_id: UserIdentifier::from(peer).bs58.into_owned(),
      },
    )
    .expect("Failed to emit WantsRoom event");
  Ok(())
}
