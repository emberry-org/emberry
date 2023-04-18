use std::{io, time::Duration};

use smoke::messages::vlink::Signal as SmokeVlink;
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

use tracing::{error, instrument, trace, warn};
use vlink::{Action, TcpBridge};

use tauri::Window;

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

#[instrument(skip_all, fields(room = %emit_identity))]
pub async fn p2p_loop<'a, T>(
  emit_identity: &str,
  peer_ident: UserIdentifier<'a>,
  spawn_window: &Window,
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
        trace!("Received message: {:?} in {}", msg, emit_identity);
        if let Err(err) = p2p_tunl::signal::handle_signal(
          &msg,
          spawn_window,
          &events,
          &mut msg_from,
          &mut usr_status_cache,
          &mut opt_bridge,
        )
        .await
        {
          warn!("failed to handle signal: '{:?}' with error: '{}'", msg, err);
        }
      },
      Ok(_) = &mut *rx => {
        trace!("p2ploop {} closed by handle", emit_identity);
        return Ok(())
      },
      _ = tokio::time::sleep_until(next_kap) => {
        let msg = Signal::Kap;
        next_kap = kap_timeout();
        trace!("Sending message: {:?} in {}", msg, emit_identity);
        msg.serialize_to(stream, &mut ser_buf).expect("unable to serialize kap message").await?
      }
      Some(action) = extract_maybe(opt_bridge.as_mut(), &mut vlink_buf) => {
        next_kap = kap_timeout();
        let hypha = SmokeVlink::from_vlink(&action);
        Signal::Vlink(hypha).serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
      }
      Some(msg) = msg_rx.recv() => {
        next_kap = kap_timeout();
        // VLINK HACK ---------
        if let Signal::Message(msg) = msg.clone() {
          if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlink_open ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if opt_bridge.is_some() {
              error!("got vlink_open while it was already open");
              return Ok(());
            }
            opt_bridge = Some(TcpBridge::emit_to(port));
            emit_msg(
              spawn_window,
              &events.msg_recv,
              &Signal::Message(format!(
                "system: YOU OPENED A VLINK WITH NAME: \"default\" AT YOUR PORT \"{port}\"\n\nTYPE: \"/vlink_close\" TO TERMINATE"
              )),
            );
            trace!("Sending message: {:?} in {}", msg, emit_identity);
            let msg = Signal::VlinkOpen("default".into());
            msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
            continue;
          }else if msg == "/vlink_close" {
            if let Some(kill) = opt_bridge.take() {
              if kill.is_listening() {
                let msg = Signal::VlinkCut;
                trace!("Sending message: {:?} in {}", msg, emit_identity);
                msg.serialize_to(stream, &mut ser_buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?.await?;
              }
            }else{
              warn!("tried to close non existing vlink bridge with command");
            }
          }else if let Some(Ok(port)) = msg // accept hack
          .strip_prefix("/vlink_connect ")
          .map(|port_string| port_string.parse::<u16>())
          {
            if opt_bridge.is_some() {
              error!("got vlink_open while it was already open");
              return Ok(());
            }
            opt_bridge = Some(TcpBridge::accepting_from(port).await?); // TODO just print an error that we cannot bind that port instead of terminating
            emit_msg(
              spawn_window,
              &events.msg_recv,
              &Signal::Message(format!(
                "system: YOU CONNECTED TO Peer's VLINK.\nIT IS AVAILABLE AT YOUR PORT \"{port}\"\n\nTYPE: \"/vlink_close\" TO TERMINATE"
              )),
            );
            continue;
          }
        }
        // --------- VLINK HACK
        trace!("Sending message: {:?} in {}", msg, emit_identity);
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

// PART OF VLINK HACK
#[derive(Clone, serde::Serialize)]
struct MessageRecievedPayload<'a> {
  message: &'a Signal,
}
#[inline]
fn emit_msg(window: &Window, event_name: &str, signal: &Signal) {
  window
    .emit(event_name, MessageRecievedPayload { message: signal })
    .expect("Failed to emit event")
}
// PART OF VLINK HACK
