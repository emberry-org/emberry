use std::borrow::Cow;
use std::time::Instant;

use smoke::{messages::EmbMessage, User};
use tauri::Window;
use tracing::{error, trace_span, warn, Instrument};

use crate::data::{config, fetch_userinfo, UserIdentifier};
use crate::network::ctrl_chnl::RhizomeConnection;
use crate::network::{Networking, RRState};

use super::state;

// todo : would be nice if `request_room` called a tauri event if the user was not found. (containing the user pubkey)
#[tauri::command(async)]
pub async fn request_room(
  window: Window,
  bs58cert: String,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()> {
  let ident = UserIdentifier {
    bs58: Cow::Borrowed(&bs58cert),
  };
  let usr: User = (&ident).try_into()?;

  let span;
  if let Some((cert, _)) = config::PEM_DATA.as_ref() {
    let local = User {
      cert_data: cert.0.clone(),
    };
    span = trace_span!("room_req", source = ?local, target = ?usr);
    let _guard = span.enter();

    if cert.0 == usr.cert_data {
      warn!("Cannot request a room with yourself");
      return Ok(());
    } else {
    }
  } else {
    error!("Cannot request a room without being authenticated to the server");
    return Ok(());
  }

  let _guard = span.enter();

  // try to add to pending list
  match net.pending.lock().unwrap().entry(usr.clone()) {
    std::collections::hash_map::Entry::Occupied(mut occupied) => match occupied.get_mut() {
      RRState::Requested(instant) => {
        if instant.elapsed() > smoke::ROOM_REQ_TIMEOUT {
          *instant = Instant::now();
        } else {
          warn!("You have already requested a connection with this user");
          return Ok(());
        }
      }
      RRState::Accepted => {
        warn!("You have already requested a connection with this user");
        return Ok(());
      }
    },
    std::collections::hash_map::Entry::Vacant(e) => {
      e.insert(RRState::Requested(Instant::now()));
    }
  }

  // When we send a room request check if the user is in the database
  // if thats not the case add it and tell the frontend about it
  fetch_userinfo(ident, &window)?;

  drop(_guard);

  state::send(&rc, EmbMessage::Room(usr))
    .instrument(span)
    .await?;
  Ok(())
}
