//! TEA Update function: Message -> Model transition

use crate::tui_app::message::{AppMsg, Message, ResultsMsg, ScanConfigMsg};
use crate::tui_app::ui::components::scan_configuration::scan_button::State as ScanButtonState;
use crate::tui_app::Model;

/// Handle one message and update the model. Return a follow-up message to support cascading.
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::App(app_msg) => match app_msg {
            // App lifecycle
            AppMsg::Quit => model.set_should_quit(true),
            AppMsg::ToggleBanner => model.toggle_banner_collapsed(),

            // Navigation / focus
            AppMsg::NextField => model.next_field(),
            AppMsg::PrevField => model.prev_field(),
            AppMsg::DeselectAll => model.deselect_all(),
            AppMsg::ConfirmInput => model.confirm_input(),

            // Actions
            AppMsg::ButtonActivate => {
                model.set_scan_button_state(ScanButtonState::Active);
                model.set_scan_button_state(ScanButtonState::Normal);
                return Some(AppMsg::StartScan.into());
            }
            AppMsg::StartScan => {
                model
                    .output_buffer()
                    .push_line("[Scan starting with current configuration]".to_string());
            }
        },

        // Delegate scan configuration updates to closest parent (Model)
        Message::ScanConfig(cfg_msg) => match cfg_msg {
            ScanConfigMsg::SelectField(field) => model.set_selected_field(field),
            ScanConfigMsg::AddChar(c) => model.add_char(c),
            ScanConfigMsg::RemovePrevChar => model.remove_previous_char(),
            ScanConfigMsg::RemoveNextChar => model.remove_next_char(),
            ScanConfigMsg::DeletePrevWord => model.delete_previous_word(),
            ScanConfigMsg::DeleteNextWord => model.delete_next_word(),
            ScanConfigMsg::MoveCursorLeft => model.move_cursor_left(),
            ScanConfigMsg::MoveCursorRight => model.move_cursor_right(),
            ScanConfigMsg::MovePrevWord => model.move_cursor_to_previous_word(),
            ScanConfigMsg::MoveNextWord => model.move_cursor_to_next_word(),
        },

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
