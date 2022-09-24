use std::io::{self, ErrorKind};

use tokio::net::UdpSocket;
use Message::*;
use log::trace;
use tokio_kcp::KcpStream;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Message {
  Kap,               // &[]
  Username(String),  // 2
  Chat(String),      // 1
  EndOfConversation, // 0
  InvalidType(u8),   // (something not in the list above)
}

impl Message {
  pub async fn recv_from(socket: &mut KcpStream, buf: &mut [u8]) -> io::Result<Message> {
    let len = socket.recv(buf).await?;

    trace!("got msg");

    if len == 0 {
      return Ok(Kap);
    }

    Ok(match buf[0] {
      0 => EndOfConversation,
      1 => Chat(to_string(&buf[1..len])),
      2 => Username(to_string(&buf[1..len])),
      _ => InvalidType(buf[0]),
    })
  }

  pub async fn send_with(&self, socket: &mut KcpStream) -> io::Result<usize> {
    let mut buf = vec![];
    match self {
      Kap => (),
      Username(name) => to_packet(&mut buf, name, 2),
      Chat(message) => to_packet(&mut buf, message, 1),
      EndOfConversation => buf.push(0),
      InvalidType(_) => {
        return Err(io::Error::new(
          ErrorKind::InvalidInput,
          "Cannot send message of invalid type",
        ))
      }
    };

    trace!("sent msg");
    socket.send(&buf).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
  }
}

fn to_string(buf: &[u8]) -> String {
  String::from_utf8_lossy(buf).into()
}

fn to_packet(buf: &mut Vec<u8>, text: &str, msg_type: u8) {
  buf.push(msg_type);
  buf.extend_from_slice(text.as_bytes());
}
