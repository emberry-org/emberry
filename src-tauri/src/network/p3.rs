use log::trace;
use std::io::{self, Error};
use tokio::net::UdpSocket;

pub async fn ping_pong_peng(socket: &UdpSocket) -> io::Result<()> {
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
