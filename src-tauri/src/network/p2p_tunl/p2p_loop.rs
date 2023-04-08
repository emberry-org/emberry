use std::{io, time::Duration};

use smoke::messages::hypha;
use smoke::{
  messages::{Drain, Source},
  Signal,
};

use tokio::{
  io::{AsyncRead, AsyncWrite, BufReader},
  select,
  sync::mpsc::Receiver,
  sync::oneshot,
  time::Instant,
};

use log::error;
use vlink::{Action, TcpBridge};

use tauri::{AppHandle, Window};

use crate::{
  data::{
    sqlite::{exec, user::get},
    IdentifiedUserInfo, UserIdentifier,
  },
  network::p2p_tunl,
};

pub struct EventNames {
  pub msg_recv: String,
  pub usr_name: String,
}

pub async fn p2p_loop<'a, T>(
  emit_identity: &str,
  peer_ident: UserIdentifier<'a>,
  spawn_window: &Window,
  app_handle: &AppHandle,
  stream: &mut BufReader<T>,
  rx: &mut oneshot::Receiver<()>,
  msg_rx: &mut Receiver<Signal>,
) -> Result<(), io::Error>
where
  T: AsyncRead + AsyncWrite + Unpin,
{
  let msg_recv = format!("message_recieved_{}", emit_identity);
  let usr_name = format!("usr_name_{}", peer_ident.bs58);
  let events = EventNames { msg_recv, usr_name };

  let info = exec(get, &peer_ident);

  let mut msg_from = format!("Message from {}", info.username);

  let mut usr_status_cache = IdentifiedUserInfo {
    identifier: peer_ident,
    info,
  };

  let mut ser_buf = [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE];
  let mut de_buf = Vec::with_capacity(smoke::messages::signal::MAX_SIGNAL_BUF_SIZE);

  let mut vlink_buf = [0u8; smoke::messages::signal::MAX_SIGNAL_BUF_SIZE - 64];
  let mut opt_bridge: Option<TcpBridge> = None;

  // Anonymous function to avoid redundant code and have the seconds controlled in a single space
  let kap_timeout = || Instant::now() + Duration::from_secs(20);
  let mut next_kap = kap_timeout();
  loop {
    select! {
      msg = stream.read_message_cancelable(&mut de_buf) => {
        let msg = msg?;
        next_kap = kap_timeout();
        log::trace!("Received message: {:?} in {}", msg, emit_identity);
        if let Err(err) = p2p_tunl::signal::handle_signal(
          &msg,
          spawn_window,
          app_handle,
          &events,
          &mut msg_from,
          &mut usr_status_cache,
          &mut opt_bridge,
        )
        .await
        {
          log::warn!("failed to handle signal: '{:?}' with error: '{}'", msg, err);
        }
      },
      Ok(_) = &mut *rx => {
        log::trace!("p2ploop {} closed by handle", emit_identity);
        return Ok(())
      },
      _ = tokio::time::sleep_until(next_kap) => {
        let msg = Signal::Kap;
        next_kap = kap_timeout();
        log::trace!("Sending message: {:?} in {}", msg, emit_identity);
        msg.serialize_to(stream, &mut ser_buf).expect("unable to serialize kap message").await?
      }
      Some(action) = extract_maybe(opt_bridge.as_mut(), &mut vlink_buf) => {
        next_kap = kap_timeout();
        let hypha = hypha::Signal::from_vlink(&action);
        Signal::Vlink(hypha).serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
      }
      Some(msg) = msg_rx.recv() => {
        next_kap = kap_timeout();
        // VLAN HACK ---------
        if let Signal::Chat(msg) = msg.clone() {
          if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlan_accept ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if opt_bridge.is_some() {
              error!("got vlan-accept while it was already connected");
              return Ok(());
            }
            opt_bridge = Some(TcpBridge::emit_to(port));
            let msg = Signal::AcceptVlink(Ok(port));
            log::trace!("Sending message: {:?} in {}", msg, emit_identity);
            msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          } else if let Some(Ok(port)) = msg // request hack
          .strip_prefix("/vlan ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if opt_bridge.is_some() {
              error!("made vlan-req while it was already connected");
              return Ok(());
            }
            opt_bridge = Some(TcpBridge::accepting_from(port).await.expect("could not bind port")); // TODO nice error for bind err
            let msg = Signal::RequestVlink(port);
            log::trace!("Sending message: {:?} in {}", msg, emit_identity);
            msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          }else if msg == "/vlan_kill" {
            if let Some(kill) = opt_bridge.take() {
              drop(kill); // drop the sender to signal kill
              let msg = Signal::KillVlink;
              log::trace!("Sending message: {:?} in {}", msg, emit_identity);
              msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            }else{
              log::warn!("tried to kill non existing vlan with command");
            }
          }
        }
        // --------- VLAN HACK
        log::trace!("Sending message: {:?} in {}", msg, emit_identity);
        msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?
      },
    }
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
