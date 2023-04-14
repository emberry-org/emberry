use std::sync::atomic::Ordering;

use log::{trace, warn};
use tauri::api::notification::Notification;

use crate::APPID;

/// Creates a new [Notification] with the apps identifier
pub fn notification() -> Notification {
  Notification::new(&*APPID)
}

/// Tries to show the given notification when the window is out of
/// focus
pub fn os_notify(notify: Notification) {
  /* Create a new notification for the message */
  if !crate::FOCUS.load(Ordering::SeqCst) {
    if let Err(err) = notify.show() {
      warn!("could not display notification with error {err}");
    }
  } else {
    trace!("notification skipped due to window state, {notify:?}");
  }
}

/// Tries to show the given notification, regardless of the window state
pub fn force_os_notify(notify: Notification) {
  if let Err(err) = notify.show() {
    warn!("could not display notification with error {err}");
  }
}