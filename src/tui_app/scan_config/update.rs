use super::{message::ScanConfigMsg, model::SelectedField, ScanConfig};

pub fn update_scan_config(cfg: &mut ScanConfig, msg: ScanConfigMsg) {
    match msg {
        ScanConfigMsg::SelectField(field) => cfg.set_selected_field(field),
        ScanConfigMsg::DeselectAll => cfg.deselect_all(),
        ScanConfigMsg::NextField => cfg.next_field(),
        ScanConfigMsg::PrevField => cfg.prev_field(),
        ScanConfigMsg::ConfirmInput => match cfg.selected_field {
            SelectedField::Targets => {
                if !cfg.targets_input.is_empty() {
                    let text = cfg.targets_input.text().to_string();
                    cfg.targets = text
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                cfg.targets_input.clear();
            }
            SelectedField::Ports => {
                if !cfg.ports_input.is_empty() {
                    cfg.ports = Some(cfg.ports_input.text().to_string());
                } else {
                    cfg.ports = None;
                }
                cfg.ports_input.clear();
            }
            _ => {}
        },
        ScanConfigMsg::ButtonActivate => cfg.start_button_activation(),
        ScanConfigMsg::AddChar(c) => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.insert_char(c)
            }
        }
        ScanConfigMsg::Paste(s) => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.insert_str(&s)
            }
        }
        ScanConfigMsg::RemovePrevChar => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.remove_previous_char()
            }
        }
        ScanConfigMsg::RemoveNextChar => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.remove_next_char()
            }
        }
        ScanConfigMsg::DeletePrevWord => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.delete_previous_word()
            }
        }
        ScanConfigMsg::DeleteNextWord => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.delete_next_word()
            }
        }
        ScanConfigMsg::MoveCursorLeft => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.move_cursor_left()
            }
        }
        ScanConfigMsg::MoveCursorRight => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.move_cursor_right()
            }
        }
        ScanConfigMsg::MovePrevWord => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.move_cursor_to_previous_word()
            }
        }
        ScanConfigMsg::MoveNextWord => {
            if let Some(input) = cfg.selected_text_input_mut() {
                input.move_cursor_to_next_word()
            }
        }
    }
}
