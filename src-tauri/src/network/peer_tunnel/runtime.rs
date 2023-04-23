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
use tracing::{error, info, instrument, trace, warn};
use vlink::{Action, TcpBridge};

use crate::data::sqlite::try_exec;
use crate::data::sqlite::user::upsert;
use crate::data::{self, IdentifiedUserInfo, UserIdentifier};
use crate::frontend::{notification, os_notify};

use super::addons::{Capture, SlashCommands};

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
      msg_recv_evnt: format!("user_msg_{}", self.room_id),
      sys_msg_evnt: format!("sys_msg_{}", self.room_id),
      room_id: self.room_id,
      notify_title: format!("Message from {}", peer.info.username),
      sys_notify_title: format!("{} - System Notification", peer.info.username),
      peer,
      usr_name_evnt,
      ser_buf: [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE],
      de_buf: Vec::with_capacity(smoke::messages::signal::MAX_SIGNAL_BUF_SIZE),
      vlink_buf: [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE - 64],
      opt_bridge: None,
      scheduled: Vec::new(),
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
  sys_msg_evnt: String,
  sys_notify_title: String,
  usr_name_evnt: String,
  peer: IdentifiedUserInfo<'static>,
  ser_buf: [u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE],
  de_buf: Vec<u8>,
  vlink_buf: [u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE - 64],
  pub opt_bridge: Option<TcpBridge>,
  scheduled: Vec<Signal>,

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
      while let Some(io) = self.scheduled.pop() {
        self.send_io(&io).await?;
      }
      select! {
        msg = self.io.read_message_cancelable(&mut self.de_buf) => {
          let msg = msg?;
          next_kap = kap_timeout();
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
          self.send_io(&msg).await?;
        }
        Some(action) = extract_maybe(self.opt_bridge.as_mut(), &mut self.vlink_buf) => {
          next_kap = kap_timeout();
          let hypha = SmokeVlink::from_vlink(&action);
          self.send_io(&Signal::Vlink(hypha)).await?;
        }
        Some(msg) = self.user_input.recv() => {
          next_kap = kap_timeout();

          if let Signal::Message(maybe_command) = &msg {
            if let Capture = self.try_execute(maybe_command){
              continue;
            }
          }

          self.send_io(&msg).await?;
        },
      }
    }
  }

  async fn handle_signal(&mut self, signal: &Signal) -> Result<(), io::Error> {
    let discriminant = std::mem::discriminant(signal);
    trace!("Received signal {discriminant:?}");

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

        self.sys_msg(&format!("Your peer has opened a Vlink called '{name}'"));
      }
      Signal::VlinkCut => {
        if let Some(mut bridge) = self.opt_bridge.take() {
          if bridge.is_listening() {
            bridge.remote_disconnect();
            info!("peer has closed vlink killed all active connections. Keep listening for new");
            self.opt_bridge = Some(bridge);
          } else {
            info!("peer has closed vlink");
            self.sys_msg("Vlink has been terminated by peer");
          }
        }
      }
      Signal::ChangeContext(new_peer_context) => todo!("create context/campfire system"),
      // CONTEXT SENSITIVE SIGNALS
      Signal::Message(text) => self.emit_msg(text),
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

  pub fn sys_msg(&mut self, message: &str) {
    self
      .window
      .emit(&self.sys_msg_evnt, message)
      .expect("Failed to emit event");

    os_notify(notification().title(&self.sys_notify_title).body(message));
  }

  fn emit_msg(&mut self, message: &str) {
    self
      .window
      .emit(&self.msg_recv_evnt, message)
      .expect("Failed to emit event");

    os_notify(notification().title(&self.notify_title).body(message));
  }

  async fn send_io(&mut self, signal: &Signal) -> io::Result<()> {
    let discriminant = std::mem::discriminant(signal);
    trace!("Sending signal: {:?}", discriminant);
    signal
      .serialize_to(&mut self.io, &mut self.ser_buf)
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
      .await
  }

  pub fn schedule_io(&mut self, signal: Signal) {
    self.scheduled.push(signal);
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
