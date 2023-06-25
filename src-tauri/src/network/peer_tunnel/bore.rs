use std::io::{self, Error, ErrorKind};
use std::net::SocketAddr;
use std::time::Duration;

use rustls::Certificate;
use smoke::{messages::RoomId, User};
use tokio::io::BufReader;
use tokio_kcp::{KcpConfig, KcpNoDelayConfig, KcpStream};
use tracing::error;

use crate::network::rhizome::punch_hole;
use crate::network::tls_kcp;
use crate::network::UserIdentification;

use super::tunnel::{PeerTunnel, PeerTunnelBuilder};

pub struct TunnelBore<'a> {
  pub window: tauri::Window,
  pub room_id: RoomId,
  pub peer: &'a User,
  pub identification: &'a UserIdentification,
}

impl<'a> TunnelBore<'a> {
  pub async fn drill(self) -> io::Result<PeerTunnel> {
    let priority = self.identification.certificate.0 < self.peer.cert_data;
    let identity = bs58::encode(&self.room_id.0).into_string();
    let me = User {
      cert_data: self.identification.certificate.0.clone(),
    };

    self
      .window
      .emit("punching", &identity)
      .expect("Failed to emit punching event");

    /* Holepunch using rhizome */
    let server_addr = dotenv!("SERVER_ADDRESS")
      .parse()
      .map_err(|err| io::Error::new(ErrorKind::Other, err))?;
    let socket = punch_hole(server_addr, &self.room_id.0).await?;
    let addr = socket.peer_addr()?;

    let config = KcpConfig {
      mtu: 1400,
      nodelay: KcpNoDelayConfig {
        nodelay: false,
        interval: 10,
        resend: 0,
        nc: false,
      },
      wnd_size: (1024, 1024),
      session_expire: Duration::from_secs(90),
      flush_write: false,
      flush_acks_input: false,
      stream: true,
    };

    let stream = KcpStream::connect_with_socket(&config, socket, addr)
      .await
      .map_err(|e| {
        error!("Kcp error: {}", e);
        Error::new(ErrorKind::Other, "Kcp error")
      })?;

    let peer_cert = Certificate(self.peer.cert_data.clone());
    let stream = if priority {
      tls_kcp::wrap_client(stream, &peer_cert, self.identification).await
    } else {
      tls_kcp::wrap_server(stream, &peer_cert, self.identification).await
    };

    let stream = stream.map_err(|err| {
      error!("Unable to start TLS on KCP stream, Err: '{}'", err);
      Error::new(ErrorKind::Other, "TLS could not be established")
    })?;

    let stream = BufReader::new(stream);

    PeerTunnelBuilder {
      window: self.window,
      room_id: identity,
      peer: self.peer.clone(),
      stream,
      me,
    }
    .build()
  }
}
