//! TEA Update function: Message -> Model transition

use crate::tui_app::message::{AppMsg, Message, ResultsMsg};
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
                model
                    .output_buffer()
                    .push_line("[Scan starting with current configuration]".to_string());
            }
        },

        // Delegate scan configuration updates to its own update
        Message::ScanConfig(cfg_msg) => update_scan_config(model.scan_config_mut(), cfg_msg),

        // Delegate scrolling to results/output parent
        Message::Results(res_msg) => match res_msg {
            ResultsMsg::ScrollUp(n) => model.output_buffer().scroll_up(n),
            ResultsMsg::ScrollDown(n) => model.output_buffer().scroll_down(n),
            ResultsMsg::ScrollToTop => model.output_buffer().scroll_to_top(),
            ResultsMsg::ScrollToBottom => model.output_buffer().scroll_to_bottom(),
        },
    }
    None
}
