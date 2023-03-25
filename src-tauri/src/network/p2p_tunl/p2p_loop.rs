use std::{io, time::Duration};

use smoke::{
  messages::{Drain, Source},
  Signal,
};
use tokio::sync::mpsc;
use tokio::{
  io::{AsyncRead, AsyncWrite, BufReader},
  select,
  sync::mpsc::Receiver,
  sync::oneshot,
  time::Instant,
};

use log::error;

use super::vlan;

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

  let mut vlan = None;
  let (vlan_tx, mut vlan_local_rx) = mpsc::channel::<Vec<u8>>(10);

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
          &mut vlan,
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
      // VLAN HACK -------
      Some(data_l) = vlan_local_rx.recv() => {
        next_kap = kap_timeout();
        log::trace!("Sending {} in {emit_identity} vlan: {}", data_l.len() ,String::from_utf8_lossy(&data_l));
        Signal::Vlan(Ok(data_l)).serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
      }
      // --------- VLAN HACK
      Some(msg) = msg_rx.recv() => {
        next_kap = kap_timeout();
        // VLAN HACK ---------
        if let Signal::Chat(msg) = msg.clone() {
          if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlan_accept ")
          .map(|port_string| port_string.parse::<u16>())
          {
            let (tx, rx) = mpsc::channel::<Vec<u8>>(10);
            tokio::spawn(vlan::connect(port, rx, vlan_tx.clone()));
            if vlan.is_some() {
              error!("got vlan-accept while it was already connected");
              return Ok(());
            }
            vlan = Some(tx);
            let msg = Signal::VlanAccept(Ok(port));
            log::trace!("Sending message: {:?} in {}", msg, emit_identity);
            msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          } else if let Some(Ok(port)) = msg // request hack
          .strip_prefix("/vlan ")
          .map(|port_string| port_string.parse::<u16>())
          {
            let (tx, rx) = mpsc::channel::<Vec<u8>>(10);
            tokio::spawn(vlan::listen(port, rx, vlan_tx.clone()));
            if vlan.is_some() {
              error!("made vlan-req while it was already connected");
              return Ok(());
            }
            vlan = Some(tx);
            let msg = Signal::VlanRequest(port);
            log::trace!("Sending message: {:?} in {}", msg, emit_identity);
            msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          }
        }
        // --------- VLAN HACK
        log::trace!("Sending message: {:?} in {}", msg, emit_identity);
        msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?
      },
    }
  }
}
