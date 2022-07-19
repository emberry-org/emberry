use super::EmberryMessage;
use std::sync::Mutex;
use tokio::sync::mpsc::Sender;

pub struct RhizomeConnection{
   pub channel: Mutex<Option<Sender<EmberryMessage>>>
}