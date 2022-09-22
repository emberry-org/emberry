use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use smoke::messages::RoomId;
use smoke::{PubKey, User};
use tauri::EventHandler;

use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::oneshot;

use log::{error, trace};

pub mod ctrl_chnl;
pub mod message;
use message::Message;

pub const ENV: Config = Config {
  public_key: dotenv!("PUBLIC_KEY"),
  server_address: dotenv!("SERVER_ADDRESS"),
};

const HELLO: [u8; 6] = [1, 3, 3, 7, 4, 2];

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
  message: Message,
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

  let arc_sock = Arc::new(socket);
  let arc_sock2 = arc_sock.clone();

  /* Setup the send event for the frontend */
  let sender = arc_sock.clone();
  let send_handle = window.listen(format!("send_message_{}", identity), move |e| {
    let sender = sender.clone();
    let msg = serde_json::from_str::<Message>(
      e.payload()
        .expect("Invalid payload in send_message_<id> event"),
    )
    .expect("Invalid Json inside of payload from send_message_<id> event");
    tokio::spawn(async move { msg.send_with(&sender).await });
  });

  /* Setup the receive loop */
  let (recv_handle, mut rx) = oneshot::channel::<()>();
  let mut buf = [0u8; 512];
  let emit_identity = identity.clone();
  let spawn_window = window.clone();
  tokio::spawn(async move {
    let event_name = format!("message_recieved_{}", emit_identity);
    loop {
      select! {
          Ok(msg) = Message::recv_from(&arc_sock, &mut buf) => {

        /* Emit the message_recieved event when a message is recieved */
        spawn_window
          .emit(&event_name, MessageRecievedPayload { message: msg })
          .expect("Failed to emit event");

        },
        Ok(_) = &mut rx => {
          break;
        }
      }
    }
  });

  let con = Connection {
    recv_handle,
    send_handle,
  };
  state.chats.lock().unwrap().insert(room_id.clone(), con);

  let mut buf = [0u8; 6];
  if other_key.as_ref() < ENV.public_key.as_bytes() {
    trace!("client mode");
    arc_sock2.send(&HELLO).await?;
    arc_sock2.recv(&mut buf).await?;
    if buf != HELLO {
      error!("error hello unequal");
      Err(Error::new(ErrorKind::Other, "unmatched hello"))?;
    }
    trace!("hello matched");

  } else {
    trace!("server mode");
    arc_sock2.recv(&mut buf).await?;
    if buf != HELLO {
      error!("error hello unequal");
      Err(Error::new(ErrorKind::Other, "unmatched hello"))?;
    }
    arc_sock2.send(&HELLO).await?;
    trace!("hello matched");
  }

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
