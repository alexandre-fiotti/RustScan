//! TEA Update function: Message -> Model transition

use crate::tui_app::{
    message::{AppMsg, Message},
    model::{FocusedArea, ScanState},
    results::{clear_results_sender, update::update_results, ResultsMsg},
    scan_config::update::update_scan_config,
    Model,
};

/// Handle one message and update the model. Return a follow-up message to support cascading.
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::App(app_msg) => match app_msg {
            // App lifecycle
            AppMsg::Quit => model.set_should_quit(true),
            AppMsg::ToggleBanner => model.toggle_banner_collapsed(),
            AppMsg::SetFocus(area) => model.set_focused_area(area),

            AppMsg::StartScan => match model.scan_state() {
                ScanState::Running | ScanState::Requested => {}
                _ => {
                    model.scan_config_mut().deselect_all();
                    model.set_scan_state(ScanState::Requested);
                }
            },
            AppMsg::StopScan => {
                update_results(
                    model.results_mut(),
                    ResultsMsg::AppendLine("[Scan stopped]".to_string()),
                );
                update_results(model.results_mut(), ResultsMsg::AppendLine("".to_string()));
                let _ = model.take_scan_results_rx();
                clear_results_sender();
                model.set_scan_state(ScanState::Completed);
            }
        },

        // Delegate scan configuration updates to its own update
        Message::ScanConfig(cfg_msg) => {
            model.set_focused_area(FocusedArea::ScanConfig);
            update_scan_config(model.scan_config_mut(), cfg_msg)
        }

        // Delegate results updates
        Message::Results(res_msg) => {
            model.set_focused_area(FocusedArea::Results);
            update_results(model.results_mut(), res_msg)
        }
    }
    None
}
