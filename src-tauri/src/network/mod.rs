use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use smoke::messages::RoomId;
use smoke::{PubKey, User};
use smoke::Signal;
use tauri::EventHandler;

use tokio::io::BufReader;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::{mpsc, oneshot};

use tokio_kcp::{KcpConfig, KcpStream};

use log::{error, trace};

pub mod ctrl_chnl;

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

  let mut buf = [0u8; 4];
  socket.send(b"PING").await.unwrap();
  trace!("sent ping");
  let mut ping = false;
  let mut success = false;
  for i in 0..3 {
    socket.recv(&mut buf).await.unwrap();
    trace!("{}: got {}", i, String::from_utf8_lossy(&buf));
    match &buf {
      b"PING" => {
        ping = true;
        socket.send(b"PONG").await.unwrap();
      }
      b"PONG" => {
        if !ping {
          socket.send(b"PENG").await.unwrap();
        }
        success = true;
        break;
      }
      b"PENG" => {
        success = true;
        break;
      }
      _ => panic!("neither PING nor PONG during greet"),
    }
  }
  assert!(success, "PING PONG manouver was not successfull");

  let stream: KcpStream;
  if other_key.as_ref() < ENV.public_key.as_bytes() {
    trace!("client mode");
    stream = KcpStream::wrap_client(&KCP_CONF, socket)
      .await
      .map_err(|e| Error::new(ErrorKind::Other, e))
      .unwrap(); //TODO: error handle
  } else {
    trace!("server mode");
    stream = KcpStream::wrap_server(&KCP_CONF, socket)
      .await
      .map_err(|e| Error::new(ErrorKind::Other, e))
      .unwrap(); //TODO: error handle
  }
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
          msg.send_with(&mut stream).await.unwrap();
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

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: RoomId) -> bool {
  // Check if the store contains the key for this chat.
  match state.chats.lock() {
    Ok(chats) => chats.contains_key(&id),
    Err(_) => false,
  }
}

/** Create a new socket and holepunch it! */
async fn punch_hole<A>(server_addr: A, ident: &[u8]) -> Result<UdpSocket, Error>
where
  A: tokio::net::ToSocketAddrs,
{
  // Create, bind, and connect the socket:
  let socket = UdpSocket::bind("0.0.0.0:0").await?;
  socket.connect(server_addr).await?;

  // Send the server our identity (Used to match us with a peer)
  socket.send(ident).await?;

  trace!("HolePunching: Waiting on response from server");
  // Wait for the server to send us a peer:
  let mut b = [0u8; 512];
  let size = socket.recv(&mut b).await?;

  // Try parse the recieved peer address.
  let addr = parse_addr(&b, size).expect("Failed to parse address");

  trace!("HolePunching: connect to {}", &addr);

  // Swap the connection from the server to the peer.
  socket.connect(addr).await?;

  trace!("HolePunching: connected");

  Ok(socket)
}

/** Parse a collection of bytes to a valid IP address. */
fn parse_addr(b: &[u8; 512], size: usize) -> Result<SocketAddr, Error> {
  // Parse the bytes into a valid socket address:
  let ip = match b[0] {
    4 => {
      if size < 7 {
        return Err(Error::new(
          ErrorKind::InvalidData,
          "Not enough bytes to parse to an Ipv4",
        ));
      }
      IpAddr::V4(Ipv4Addr::new(b[1], b[2], b[3], b[4]))
    }
    6 => {
      if size < 19 {
        return Err(Error::new(
          ErrorKind::InvalidData,
          "Not enough bytes to parse to an Ipv6",
        ));
      }
      IpAddr::V6(Ipv6Addr::from([
        b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14],
        b[15], b[16],
      ]))
    }
    _ => {
      return Err(Error::new(
        ErrorKind::InvalidData,
        "Ip format not recognized",
      ))
    }
  };

  // Parse the remaining bytes to a valid port number:
  let port = match b[0] {
    4 => ((b[5] as u16) << 8) | b[6] as u16,
    6 => ((b[17] as u16) << 8) | b[18] as u16,
    _ => unreachable!(),
  };

  Ok(SocketAddr::new(ip, port))
}
