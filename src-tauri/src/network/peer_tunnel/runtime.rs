use std::io;

use smoke::messages::signal::KAP_TIMEOUT;
use smoke::messages::vlink::Signal as SmokeVlink;
use smoke::messages::{Drain, Source};
use smoke::Signal;
use tauri::Window;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::Instant;
use tokio::{io::BufReader, sync::oneshot};
use tracing::{error, instrument, trace, warn};
use vlink::{Action, TcpBridge};

use crate::data::sqlite::try_exec;
use crate::data::sqlite::user::upsert;
use crate::data::{self, IdentifiedUserInfo, UserIdentifier};
use crate::frontend::{notification, os_notify};

pub struct PeerTunnelRuntimeBuilder<T> {
  pub room_id: String,
  pub peer_ident: UserIdentifier<'static>,
  pub window: Window,
  pub stream: BufReader<T>,
  pub cancellation: oneshot::Receiver<()>,
  pub user_input: Receiver<Signal>,
}

impl<T> PeerTunnelRuntimeBuilder<T> {
  pub fn build(self) -> io::Result<PeerTunnelRuntime<T>> {
    let usr_name_evnt = format!("usr_name_{}", self.peer_ident.bs58);
    let peer = data::fetch_userinfo(self.peer_ident, &self.window)?;
    Ok(PeerTunnelRuntime {
      msg_recv_evnt: format!("message_recieved_{}", self.room_id),
      room_id: self.room_id,
      notify_title: format!("Message from {}", peer.info.username),
      peer,
      usr_name_evnt,
      ser_buf: [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE],
      de_buf: Vec::with_capacity(smoke::messages::signal::MAX_SIGNAL_BUF_SIZE),
      vlink_buf: [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE - 64],
      opt_bridge: None,
      io: self.stream,
      user_input: self.user_input,
      window: self.window,
      cancellation: self.cancellation,
    })
  }
}

pub struct PeerTunnelRuntime<T> {
  room_id: String,
  msg_recv_evnt: String,
  notify_title: String,
  usr_name_evnt: String,
  peer: IdentifiedUserInfo<'static>,
  ser_buf: [u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE],
  de_buf: Vec<u8>,
  vlink_buf: [u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE - 64],
  opt_bridge: Option<TcpBridge>,

  io: BufReader<T>,

  user_input: Receiver<Signal>,
  window: Window,

  cancellation: oneshot::Receiver<()>,
}

impl<T> PeerTunnelRuntime<T>
where
  T: AsyncRead + AsyncWrite + Unpin,
{
  #[instrument(skip_all, fields(room = %self.room_id))]
  pub async fn execute(mut self) -> io::Result<()> {
    // Anonymous function to avoid redundant code and have the seconds controlled in a single space
    let kap_timeout = || Instant::now() + KAP_TIMEOUT;
    let mut next_kap = kap_timeout();
    loop {
      select! {
        msg = self.io.read_message_cancelable(&mut self.de_buf) => {
          let msg = msg?;
          next_kap = kap_timeout();
          trace!("Received message");
          if let Err(err) = self.handle_signal(&msg) .await
          {
            warn!("failed to handle signal: '{:?}' with error: '{}'", msg, err);
          }
        },
        Ok(_) = &mut self.cancellation => {
          trace!("PeerTunnelRuntime ended with Cancellation");
          return Ok(())
        },
        _ = tokio::time::sleep_until(next_kap) => {
          let msg = Signal::Kap;
          next_kap = kap_timeout();
          trace!("Sending message: {:?} in {}", msg, self.room_id);
          msg.serialize_to(&mut self.io, &mut self.ser_buf).expect("unable to serialize kap message").await?
        }
        Some(action) = extract_maybe(self.opt_bridge.as_mut(), &mut self.vlink_buf) => {
          next_kap = kap_timeout();
          let hypha = SmokeVlink::from_vlink(&action);
          Signal::Vlink(hypha).serialize_to(&mut self.io, &mut self.ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
        }
        Some(msg) = self.user_input.recv() => {
          next_kap = kap_timeout();

        // VLINK HACK ---------
        if let Signal::Message(msg) = msg.clone() {
          if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlink_open ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if self.opt_bridge.is_some() {
              error!("got vlink_open while it was already open");
              return Ok(());
            }
            self.opt_bridge = Some(TcpBridge::emit_to(port));
            // todo system message
            self.emit_msg(
              &format!(
                "system: YOU OPENED A VLINK WITH NAME: \"default\" AT YOUR PORT \"{port}\"\n\nTYPE: \"/vlink_close\" TO TERMINATE"
              ),
            );
            trace!("Sending message: {:?} in {}", msg, self.room_id);
            let msg = Signal::VlinkOpen("default".into());
            msg.serialize_to(&mut self.io, &mut self.ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          }else if msg == "/vlink_close" {
            if let Some(kill) = self.opt_bridge.take() {
              if kill.is_listening() {
                let msg = Signal::VlinkCut;
                trace!("Sending message: {:?} in {}", msg, self.room_id);
                msg.serialize_to(&mut self.io, &mut self.ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
              }
            }else{
              warn!("tried to close non existing vlink bridge with command");
            }
          }else if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlink_connect ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if self.opt_bridge.is_some() {
              error!("got vlink_open while it was already open");
              return Ok(());
            }
            self.opt_bridge = Some(TcpBridge::accepting_from(port).await?); // TODO just print an error that we cannot bind that port instead of terminating
            self.emit_msg(
              &format!(
                "system: YOU CONNECTED TO Peer's VLINK.\nIT IS AVAILABLE AT YOUR PORT \"{port}\"\n\nTYPE: \"/vlink_close\" TO TERMINATE"
              ),
            );
            continue;
          }
        }
        // --------- VLINK HACK

          trace!("Sending message: {:?} in {}", msg, self.room_id);
          msg.serialize_to(&mut self.io, &mut self.ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?
        },
      }
    }
  }

  async fn handle_signal(&mut self, signal: &Signal) -> Result<(), io::Error> {
    match signal {
      Signal::Kap => (),
      Signal::Username(name) => {
        self.update_peer_name(name.to_string())?;
      }
      Signal::Vlink(internal) => {
        let Some(bridge) = &mut self.opt_bridge else {
        warn!("got vlink package while bridge was not available: {internal:?}");
        return Ok(());
      };

        bridge.input(internal.as_vlink()).await;
      }
      Signal::VlinkOpen(name) => {
        trace!("vlink opened by remote, name: {name}");
        self
          .window
          .emit("vlink-available", name)
          .expect("failed to emit vlink-available)");

        // TODO remove vlink hack

        // TODO system message
        self.emit_msg(
        &format!(
          "HAS OPENED A VLINK WITH NAME: \"{name}\"\n\nTYPE: \"/vlink_connect {name}\" TO ENABLE THE VLINK ON YOUR LOCAL PORT \"8080\"\nYOU CAN ALWAYS CLOSE THE CONNECTION USING: \"/vlink_close\""
        ),
      );
      }
      Signal::VlinkCut => {
        self.opt_bridge.take();
        trace!("dropped potential vlink bridge");
        //todo system messsage
        self.emit_msg("HAS REVOKED THE VLINK");
      }
      Signal::ChangeContext(new_peer_context) => todo!("create context/campfire system"),
      // CONTEXT SENSITIVE SIGNALS
      Signal::Message(text) => {
        self.emit_msg(text);
      }
    }

    Ok(())
  }
}

/// runtime actions
impl<T> PeerTunnelRuntime<T>
where
  T: AsyncRead + AsyncWrite + Unpin,
{
  fn update_peer_name(&mut self, new_name: String) -> io::Result<&str> {
    if self.peer.info.username != new_name {
      self.peer.info.username = new_name;
      let input = (&self.peer, |peer: &IdentifiedUserInfo| {
        self.notify_title = format!("Message from {}", peer.info.username);
        if let Err(err) = self.window.emit(&self.usr_name_evnt, &peer.info.username) {
          error!("Failed to emit event: '{}'", err);
        }
      });
      try_exec(upsert, input)?;
    }

    Ok(&self.peer.info.username)
  }

  fn emit_msg(&mut self, message: &str) {
    self
      .window
      .emit(&self.msg_recv_evnt, message)
      .expect("Failed to emit event");

    os_notify(notification().title(&self.notify_title).body(message));
  }
}

async fn extract_maybe<'a>(
  bridge: Option<&mut TcpBridge>,
  buf: &'a mut [u8],
) -> Option<Action<'a>> {
  let Some(bridge) = bridge else {
    return std::future::pending().await;
  };

  bridge.extract(buf).await
}
