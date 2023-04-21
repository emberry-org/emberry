use std::io;
use tauri::EventHandler;
use tokio::sync::oneshot;

use smoke::{Signal, User};
use tauri::Window;
use tokio::io::BufReader;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;

use crate::data::UserIdentifier;

use super::runtime::PeerTunnelRuntimeBuilder;

pub struct PeerTunnelBuilder<T> {
  pub window: Window,
  pub room_id: String,
  pub peer: User,
  pub stream: BufReader<T>,
}

impl<T> PeerTunnelBuilder<T>
where
  T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
  pub fn build(self) -> io::Result<PeerTunnel> {
    /* Setup the send event for the frontend */
    let (sender, user_input) = mpsc::channel::<Signal>(5);
    let user_input_handle =
      self
        .window
        .listen(format!("send_message_{}", self.room_id), move |e| {
          let sender = sender.clone();
          let msg = serde_json::from_str::<Signal>(
            e.payload()
              .expect("Invalid payload in send_message_<id> event"),
          )
          .expect("Invalid Json inside of payload from send_message_<id> event");
          tokio::spawn(async move { sender.send(msg).await });
        });

    /* Setup the receive loop */
    let (canceller, cancellation) = oneshot::channel::<()>();

    let runtime = PeerTunnelRuntimeBuilder {
      room_id: self.room_id,
      peer_ident: UserIdentifier::from(&self.peer),
      window: self.window.clone(),
      stream: self.stream,
      cancellation,
      user_input,
    }
    .build()?;

    tokio::spawn(runtime.execute());

    Ok(PeerTunnel {
      canceller: Some(canceller),
      user_input_handle,
      window: self.window,
    })
  }
}

pub struct PeerTunnel {
  canceller: Option<oneshot::Sender<()>>,
  user_input_handle: EventHandler,
  window: Window,
}

impl Drop for PeerTunnel {
  fn drop(&mut self) {
    self.window.unlisten(self.user_input_handle);
    _ = self.canceller.take().map(|tx| tx.send(()));
  }
}
