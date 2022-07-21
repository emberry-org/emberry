use std::sync::{Mutex, Arc};

use crate::network::ConnectionMap;

use super::EmberryMessage;
use tauri::Window;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

pub type RwOption<T> = RwLock<Option<T>>;
pub type RhizomeConnection = RwOption<State>;

pub struct State {
  pub channel: Sender<EmberryMessage>,
}
