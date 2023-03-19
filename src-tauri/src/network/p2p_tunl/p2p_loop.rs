use std::{io, time::Duration};

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
      Some(msg) = msg_rx.recv() => {
        next_kap = kap_timeout();
        log::trace!("Sending message: {:?} in {}", msg, emit_identity);
        msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?
      },
    }
  }
}
