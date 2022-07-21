use super::EmberryMessage;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;

pub struct RhizomeConnection{
   pub channel: RwLock<Option<Sender<EmberryMessage>>>
}