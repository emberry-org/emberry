use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use tauri::EventHandler;

use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::oneshot;

type ConnectionMap = HashMap<String, Connection>;
pub struct Connection {
  pub send_handle: EventHandler,
  pub recv_handle: oneshot::Sender<()>,
}

pub struct Networking {
  pub chats: Mutex<ConnectionMap>,
}

#[derive(serde::Deserialize, Debug)]
struct Config {
  server_address: String,
  public_key: String,
}

#[derive(Clone, serde::Serialize)]
struct MessageRecievedPayload {
  message: String,
}

#[tauri::command(async)]
pub async fn hole_punch(
  window: tauri::Window,
  state: tauri::State<'_, Networking>,
  peer_key: String,
) -> Result<String, String> {
  /* Get the server ip from .env */
  let env = Config {
    public_key: dotenv!("PUBLIC_KEY").into(),
    server_address: dotenv!("SERVER_ADDRESS").into(),
  };

  if env.public_key == peer_key {
    return Err("Cannot connect to oneself".into());
  }

  let identity = if env.public_key.as_bytes() < peer_key.as_bytes() {
    format!("{}{}", env.public_key, peer_key)
  } else {
    format!("{}{}", peer_key, env.public_key)
  };

  /* Holepunch using rhizome */
  // let socket = match punch_hole(env.server_address, identity.as_bytes()).await {
  //   Ok(socket) => socket,
  //   Err(e) => return Err(e.to_string()),
  // };

  //let arc_sock = Arc::new(socket);

  /* Setup the send event for the frontend */
  //let sender = arc_sock.clone();
  let send_handle = window.listen(format!("send_message_{}", identity), move |e| {
    //let sender = sender.clone();
    //tokio::spawn(async move { sender.send(e.payload().unwrap().as_bytes()).await });
  });

  /* Setup the receive loop */
  let (recv_handle, mut rx) = oneshot::channel::<()>();
  // let mut buf = [0u8; 512];
  // let emit_identity = identity.clone();
  // tokio::spawn(async move {
  //   loop {
  //     select! {
  //       Ok(len) = arc_sock.recv(&mut buf) => {
  //       let msg = String::from_utf8_lossy(&buf[..len]).to_string();

  //       /* Emit the message_recieved event when a message is recieved */
  //       window
  //         .emit(format!("message_recieved_{}", &emit_identity).as_str(), MessageRecievedPayload { message: msg })
  //         .expect("Failed to emit event");

  //       },
  //       Ok(_) = &mut rx => {
  //         break;
  //       }
  //     }
  //   }
  // });

  let con = Connection {
    recv_handle,
    send_handle,
  };
  state.chats.lock().unwrap().insert(identity.clone(), con);

  Ok(identity)
}

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: String) -> bool {
  // Check if the store contains the key for this chat.
  match state.chats.lock() {
    Ok(chats) => chats.contains_key(&id),
    Err(_) => false
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

  // Wait for the server to send us a peer:
  let mut b = [0u8; 512];
  let size = socket.recv(&mut b).await?;

  // Try parse the recieved peer address.
  let addr = parse_addr(&b, size).expect("Failed to parse address");

  // Swap the connection from the server to the peer.
  socket.connect(addr).await?;

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
