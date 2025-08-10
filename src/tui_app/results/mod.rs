//! Results (scan output) TEA module

pub mod message;
pub mod model;
pub mod update;

pub use message::ResultsMsg;
pub use model::{ResultsModel, ScrollInfo};

use crate::tui_app::message::Message;
use std::sync::{mpsc::Sender, Mutex, OnceLock};

// Optional global sender for streaming lines from non-TUI modules (e.g., scanner)
static RESULTS_SENDER: OnceLock<Mutex<Option<Sender<Message>>>> = OnceLock::new();

pub fn set_results_sender(tx: Sender<Message>) {
    if let Some(m) = RESULTS_SENDER.get() {
        if let Ok(mut guard) = m.lock() {
            *guard = Some(tx);
            return;
        }
    }
    let _ = RESULTS_SENDER.set(Mutex::new(Some(tx)));
}

pub fn clear_results_sender() {
    if let Some(m) = RESULTS_SENDER.get() {
        if let Ok(mut guard) = m.lock() {
            *guard = None;
        }
    }
}

pub fn try_send_line(line: String) {
    if let Some(m) = RESULTS_SENDER.get() {
        if let Ok(guard) = m.lock() {
            if let Some(tx) = &*guard {
                let _ = tx.send(Message::Results(ResultsMsg::AppendLine(line)));
            }
        }
    }
}

pub fn has_results_sender() -> bool {
    RESULTS_SENDER
        .get()
        .and_then(|m| m.lock().ok())
        .map(|g| g.is_some())
        .unwrap_or(false)
}
