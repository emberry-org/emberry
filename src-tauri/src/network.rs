use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

use tauri::EventHandler;

pub struct Networking {
  pub chats: HashMap<String, EventHandler>
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
pub fn hole_punch(window: tauri::Window, state: tauri::State<Networking>, peer_key: String) -> Result<String, String> {
  /* Get the server ip from .env */
  let env: Config = envy::from_iter([
    (
      String::from("SERVER_ADDRESS"),
      String::from(dotenv!("SERVER_ADDRESS")),
    ),
    (
      String::from("PUBLIC_KEY"),
      String::from(dotenv!("PUBLIC_KEY")),
    ),
  ])
  .expect("Failed to load environment variables");

  let identity: String;

  if env.public_key.as_bytes() < peer_key.as_bytes() {
    identity = format!("{}{}", env.public_key, peer_key);
  } else {
    identity = format!("{}{}", peer_key, env.public_key);
  }

  /* Holepunch using rhizome */
  let socket = match punch_hole(env.server_address, identity.as_bytes()) {
    Ok(socket) => socket,
    Err(e) => return Err(e.to_string()),
  };

  /* Setup the send event for the frontend */
  {
    let socket = socket.try_clone().unwrap();

    let send_handle = window.listen(format!("send_message_{}", identity), move |e| {
      let _ = &socket.send(e.payload().unwrap().as_bytes()).unwrap();
    });

    //state.chats.insert(identity.clone(), send_handle);
  }

  /* Setup the receive loop */
  let mut buf = [0u8; 512];
  let recv_handle = std::thread::spawn(move || loop {
    if let Ok(len) = socket.recv(&mut buf) {
      let msg = String::from_utf8_lossy(&buf[..len]).to_string();

      /* Emit the message_recieved event when a message is recieved */
      window
        .emit("message_recieved", MessageRecievedPayload { message: msg })
        .expect("Failed to emit event");
    }
  });

  Ok(identity)
}

/** Create a new socket and holepunch it! */
fn punch_hole<A>(server_addr: A, ident: &[u8]) -> Result<UdpSocket, Error>
where
  A: std::net::ToSocketAddrs,
{
  // Create, bind, and connect the socket:
  let socket = UdpSocket::bind("0.0.0.0:0")?;
  socket.connect(server_addr)?;

  // Send the server our identity (Used to match us with a peer)
  socket.send(ident)?;

  // Wait for the server to send us a peer:
  let mut b = [0u8; 512];
  let size = socket.recv(&mut b)?;

  // Try parse the recieved peer address.
  let addr = parse_addr(&b, size).expect("Failed to parse address");

  // Swap the connection from the server to the peer.
  socket.connect(addr)?;

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
