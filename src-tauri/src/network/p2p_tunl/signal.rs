use std::{borrow::Borrow, io, sync::atomic::Ordering};

use crate::{data::{
  sqlite::{try_exec, user::upsert},
  IdentifiedUserInfo,
}, frontend::{notification, os_notify}};

use smoke::Signal;
use tauri::{api::notification::Notification, AppHandle, Window};

use log::{error, trace, warn};

use super::p2p_loop::EventNames;

#[derive(Clone, serde::Serialize)]
struct MessageRecievedPayload<'a> {
  message: &'a Signal,
}

#[derive(Clone, serde::Serialize)]
struct UsernameChangedPayload<'a> {
  username: &'a str,
}

pub async fn handle_signal(
  signal: &Signal,
  spawn_window: &Window,
  events: &EventNames,
  msg_from: &mut String,
  cache: &mut IdentifiedUserInfo<'_>,
  opt_bridge: &mut Option<vlink::TcpBridge>,
) -> Result<(), io::Error> {
  match signal {
    Signal::Kap => (),
    Signal::Username(name) => {
      if &cache.info.username != name {
        cache.info.username = name.to_string();
        let input = (cache.borrow(), |info: &IdentifiedUserInfo| {
          *msg_from = format!("Message from {}", cache.info.username);
          emit_username(spawn_window, &events.usr_name, &info.info.username)
        });
        try_exec(upsert, input)?;
      }
    }
    Signal::Chat(text) => {
      emit_msg(spawn_window, &events.msg_recv, signal);

      os_notify(notification().title(&*msg_from).body(text));
    }
    Signal::Vlink(internal) => {
      let data_r = match internal {
        smoke::messages::hypha::Signal::Connect(_) => {
          trace!("connect");
          return Ok(());
        }
        smoke::messages::hypha::Signal::Data(_port, data_r) => data_r,
        smoke::messages::hypha::Signal::Error(_, _) => {
          error!("error");
          return Ok(());
        }
        smoke::messages::hypha::Signal::AcceptError(_) => {
          error!("accept error");
          return Ok(());
        }
      };

      let Some(bridge) = opt_bridge else {
        warn!("got vlan while is was not available: {data_r:?}");
        return Ok(());
      };

      log::trace!("got {} vlan", data_r.len());
      bridge.input(internal.as_vlink()).await;
    }
    Signal::RequestVlink(port) => {
      trace!("vlan requested, target port: {port}");
      spawn_window
        .emit("vlan-req", port)
        .expect("failed to emit vlan-req)");

      // TODO remove vlan hack
      emit_msg(
        spawn_window,
        &events.msg_recv,
        &Signal::Chat(format!(
          "WANTS TO OPEN A VLAN CONNECTION TO CONNECT TO YOUR \"127.0.0.1:{port}\"\n\nTYPE: \"/vlan_accept {port}\" TO ACCEPT THE REQUEST\nYOU CAN ALWAYS CLOSE THE CONNECTION USING: \"/vlan_kill\""
        )),
      );
    }
    Signal::AcceptVlink(res) => match res {
      Ok(port) => {
        trace!("vlan accept, port {port}");
        emit_msg(spawn_window, &events.msg_recv, &Signal::Chat(format!("ACCEPTED A VLAN CONNECTION, MAPPING (your)\"127.0.0.1:{port}\" -> (their)\"127.0.0.1:{port}\"")))
      }
      Err(err) => {
        trace!("vlan was declined: '{err}'");
        drop(opt_bridge.take());
      }
    },
    Signal::KillVlink => {
      let Some(kill) = opt_bridge.take() else {
        warn!("got vlan_kill while is was not available");
        return Ok(());
      };
      drop(kill); // drop the sender to signal closing
      trace!("killed vlan");
      emit_msg(
        spawn_window,
        &events.msg_recv,
        &Signal::Chat("HAS KILLED THE VLAN".to_string()),
      );
    }
    Signal::EOC => todo!(),
  }

  Ok(())
}

#[inline]
fn emit_username(window: &Window, event_name: &str, name: &str) {
  if let Err(err) = window.emit(event_name, name) {
    log::error!("Failed to emit event: '{}'", err);
  }
}

#[inline]
fn emit_msg(window: &Window, event_name: &str, signal: &Signal) {
  window
    .emit(event_name, MessageRecievedPayload { message: signal })
    .expect("Failed to emit event")
}
