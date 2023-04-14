use std::{
  io::{self, Error, ErrorKind},
  net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};
use tokio::net::UdpSocket;
use tracing::{error, trace};

/** Create a new socket and holepunch it! */
pub async fn punch_hole<A>(server_addr: A, ident: &[u8]) -> Result<UdpSocket, Error>
where
  A: tokio::net::ToSocketAddrs,
{
  trace!("initiating ip exchange");
  // Create, bind, and connect the socket:
  let socket = UdpSocket::bind("0.0.0.0:0").await?;
  socket.connect(server_addr).await?;

  // Send the server our identity (Used to match us with a peer)
  socket.send(ident).await?;

  // Wait for the server to send us a peer:
  let mut b = [0u8; 512];
  let size = socket.recv(&mut b).await?;

  if size == 0 {
    return Err(Error::new(
      ErrorKind::UnexpectedEof,
      "rhizome didn't transmit peer address",
    ));
  }

  // Try parse the recieved peer address.
  let addr = parse_addr(&b, size)?;

  trace!("connecting to peer: {}", &addr);

  // Swap the connection from the server to the peer.
  socket.connect(addr).await?;

  if let Err(err) = ping_pong_peng(&socket).await {
    error!("P3 failure: {}", err);
    return Err(Error::new(ErrorKind::Other, "P3 failure"));
  }

  Ok(socket)
}

async fn ping_pong_peng(socket: &UdpSocket) -> io::Result<()> {
  trace!("initiating PingPongPeng (P3) manouver");
  let mut buf = [0u8; 4];
  socket.send(b"PING").await?;
  let mut ping = false;
  for i in 0..2 {
    socket.recv(&mut buf).await?;
    trace!("{}: got {}", i, String::from_utf8_lossy(&buf));
    match &buf {
      b"PING" => {
        ping = true;
        socket.send(b"PONG").await?;
      }
      b"PONG" => {
        if !ping {
          socket.send(b"PENG").await?;
        }
        return Ok(());
      }
      b"PENG" => {
        return Ok(());
      }
      _ => {
        return Err(Error::new(
          io::ErrorKind::InvalidData,
          "malformed PING/PONG/PENG",
        ))
      }
    }
  }

  Err(Error::new(io::ErrorKind::Other, "got multiple PING"))
}

/** Parse a collection of bytes to a valid IP address. */
fn parse_addr(b: &[u8; 512], size: usize) -> Result<SocketAddr, Error> {
  // Parse the bytes into a valid socket address:
  let ip = match b[0] {
    4 => {
      if size < 7 {
        error!("parsing sock addr: '{:x?}' failed", &b[..size]);
        return Err(Error::new(ErrorKind::InvalidData, "peer address malformed"));
      }
      IpAddr::V4(Ipv4Addr::new(b[1], b[2], b[3], b[4]))
    }
    6 => {
      if size < 19 {
        error!("parsing sock addr: '{:x?}' failed", &b[..size]);
        return Err(Error::new(ErrorKind::InvalidData, "peer address malformed"));
      }
      IpAddr::V6(Ipv6Addr::from([
        b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14],
        b[15], b[16],
      ]))
    }
    _ => {
      error!("parsing peer addr: '{:x?}' failed", &b[..size]);
      return Err(Error::new(ErrorKind::InvalidData, "peer address malformed"));
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
