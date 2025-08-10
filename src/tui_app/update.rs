//! TEA Update function: Message -> Model transition

use crate::tui_app::message::{AppMsg, Message};
use crate::tui_app::results::update::update_results;
use crate::tui_app::scan_config::update::update_scan_config;
use crate::tui_app::Model;

/// Handle one message and update the model. Return a follow-up message to support cascading.
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::App(app_msg) => match app_msg {
            // App lifecycle
            AppMsg::Quit => model.set_should_quit(true),
            AppMsg::ToggleBanner => model.toggle_banner_collapsed(),

            AppMsg::StartScan => {
                // Only start if not already running/requested
                match model.scan_state() {
                    crate::tui_app::model::ScanState::Running
                    | crate::tui_app::model::ScanState::Requested => {}
                    _ => {
                        model.scan_config_mut().deselect_all();
                        model.set_scan_state(crate::tui_app::model::ScanState::Requested);
                    }
                }
            }
            AppMsg::StopScan => {
                use crate::tui_app::results::ResultsMsg;
                // Inform user and stop (no blank line before; one after)
                update_results(
                    model.results_mut(),
                    ResultsMsg::AppendLine("[Scan stopped]".to_string()),
                );
                update_results(model.results_mut(), ResultsMsg::AppendLine("".to_string()));
                // Drop receiver so worker sends fail and stop flooding
                let _ = model.take_scan_results_rx();
                // Also clear any result sender to avoid stale handle
                crate::tui_app::results::clear_results_sender();
                model.set_scan_state(crate::tui_app::model::ScanState::Completed);
            }
        },

        // Delegate scan configuration updates to its own update
        Message::ScanConfig(cfg_msg) => update_scan_config(model.scan_config_mut(), cfg_msg),

        // Delegate results updates
        Message::Results(res_msg) => update_results(model.results_mut(), res_msg),
    }
    None
}
