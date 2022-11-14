use std::io::{self, Error, ErrorKind};

use super::EmberryMessage;
use log::trace;
use smoke::messages::EmbMessage;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

pub type RwOption<T> = RwLock<Option<T>>;
pub type RhizomeConnection = RwOption<State>;

pub struct State {
  pub channel: Sender<EmberryMessage>,
}

/// Tries to send msg using the inner "channel" if it is there returning io error if not
pub async fn send(rc: &RhizomeConnection, msg: EmbMessage) -> io::Result<()> {
  let guard = rc.read().await;

  let tx = match &*guard {
    Some(rc) => &rc.channel,
    None => return Err(Error::new(ErrorKind::Other, "No connection to rhizome")),
  };

  tx.send(EmberryMessage::Direct(msg))
    .await
    .map_err(|_| Error::new(ErrorKind::ConnectionReset, "Rhizome connection closed"))
}
