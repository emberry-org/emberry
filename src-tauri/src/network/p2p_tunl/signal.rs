use std::{borrow::Borrow, io, sync::atomic::Ordering};

use crate::data::{
  sqlite::{try_exec, user::upsert},
  IdentifiedUserInfo,
};

use smoke::{messages::hypha, Signal};
use tauri::{api::notification::Notification, AppHandle, Window};

use log::{error, trace, warn};
use tokio::sync::mpsc::Sender;

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
  app_handle: &AppHandle,
  events: &EventNames,
  msg_from: &mut String,
  cache: &mut IdentifiedUserInfo<'_>,
  vlan: &mut Option<Sender<Vec<u8>>>,
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

      /* Create a new notification for the message */
      if !crate::FOCUS.load(Ordering::SeqCst) {
        Notification::new(&app_handle.config().tauri.bundle.identifier)
          .title(msg_from.as_str())
          .body(text)
          .show()
          .expect("Failed to send desktop notification");
      }
    }
    Signal::Hypha(signal) => {
      handle_hypha(signal, spawn_window, vlan, events).await?;
    }
    _ => emit_msg(spawn_window, &events.msg_recv, signal),
  }

  Ok(())
}

#[inline]
async fn handle_hypha(
  signal: &hypha::Signal,
  spawn_window: &Window,
  vlan: &mut Option<Sender<Vec<u8>>>,
  events: &EventNames,
) -> Result<(), io::Error> {
  match signal {
    hypha::Signal::Data(data_r) => {
      let Some(local_tx) = vlan else {
        warn!("got vlan while is was not available: {data_r:?}");
        return Ok(());
      };

      log::trace!("got {} vlan", data_r.len());
      local_tx
        .send(data_r.clone())
        .await
        .expect("vlan intercom fail");
    }
    hypha::Signal::Accept(data) => {
      let Some(vlan) = vlan else {
        warn!("got vlan_accept while is was not available: {data:?}");
        return Ok(());
      };

      match data {
        Err(err) => {
          trace!("vlan was declined: '{err}'");
          vlan.send(Vec::new()).await.expect("vlan intercom fail");
        }
        Ok(port) => {
          trace!("vlan accept, target port: {port}");
          emit_msg(
          spawn_window,
          &events.msg_recv,
          &Signal::Chat(format!(
            "ACCEPTED A VLAN CONNECTION TO THEIR \"127.0.0.1:{port}\", which is mapped to your \"127.0.0.1:{port}\""
          )),
        );
        }
      }
    }
    hypha::Signal::Request(port) => {
      trace!("vlan requested, target port: {port}");
      spawn_window
        .emit("vlan-req", port)
        .expect("failed to emit vlan-req");

      error!("advertise vlan hack");
      emit_msg(
          spawn_window,
          &events.msg_recv,
          &Signal::Chat(format!(
            "WANTS TO OPEN A VLAN CONNECTION TO CONNECT TO YOUR \"127.0.0.1:{port}\"\n\nTYPE: \"/vlan_accept {port}\" TO ACCEPT THE REQUEST\nYOU CAN ALWAYS CLOSE THE CONNECTION USING: \"/vlan_kill\""
          )),
        );
    }
    hypha::Signal::Kill => {
      let Some(kill) = vlan.take() else {
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
