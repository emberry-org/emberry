use crate::{
  data::{config, UserIdentifier},
  network::{Networking, RRState},
};

use super::{state, RhizomeConnection};
use smoke::messages::EmbMessage;
use smoke::User;
use std::io::{Error, ErrorKind};
use tauri::Result;
use tracing::{trace, trace_span, warn, Instrument};

#[tauri::command(async)]
pub async fn accept_room(
  bs58cert: String,
  accepted: bool,
  net: tauri::State<'_, Networking>,
  rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()> {
  let local = if let Some((cert, _)) = config::PEM_DATA.as_ref() {
    User {
      cert_data: cert.0.clone(),
    }
  } else {
    return Err(tauri::Error::Io(Error::new(
      ErrorKind::Other,
      "cannot send accept room if not connected to rhizome",
    )));
  };

  let ident = UserIdentifier { bs58: bs58cert };
  let usr: User = (&ident).try_into()?;
  let err = || {
    Err(tauri::Error::Io(Error::new(
        ErrorKind::Other,
        "It should not be possible to interact with the room request popup if the request is already pending"
      )))
  };

  let span = trace_span!("room_req", source = ?usr, target = ?local);

  {
    let _guard = span.enter();

    let mut guard = net.pending.lock().unwrap();
    match guard.entry(usr) {
      std::collections::hash_map::Entry::Occupied(mut entry) => match entry.get() {
        RRState::Requested(timer) => {
          if timer.elapsed() > smoke::ROOM_REQ_TIMEOUT {
            entry.insert(RRState::Accepted);
            trace!("agree to room request");
          } else {
            warn!("tried accepting room that was already in agreement");
            return err();
          }
        }
        RRState::Accepted => return err(),
      },
      std::collections::hash_map::Entry::Vacant(entry) => {
        entry.insert(RRState::Accepted);
        trace!("agree to room request");
      }
    }
  }

  let msg = EmbMessage::Accept(accepted);
  state::send(&rc, msg).instrument(span).await?;
  Ok(())
}
