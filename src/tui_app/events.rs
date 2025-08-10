//! Event → Message mapping (TEA)

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

#[derive(Debug)]
pub enum HandleEventError {
    Io(std::io::Error),
}

impl From<std::io::Error> for HandleEventError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

use crate::tui_app::message::{AppMsg, Message, ResultsMsg};
use crate::tui_app::model::Model;
use crate::tui_app::scan_config::ScanConfigMsg;
use crate::tui_app::scan_config::SelectedField;
use crate::tui_app::ui::theme::layout;

/// Map an input event to an optional Message
pub fn handle_event(model: &Model, event: Event) -> Result<Option<Message>, HandleEventError> {
    let msg = match event {
        Event::Key(key) => handle_key_event(model, key)?,
        Event::Mouse(mouse) => handle_mouse_event(model, mouse)?,
        Event::Resize(_, _) => None,
        _ => None,
    };
    Ok(msg)
}

/// Handle keyboard events
fn handle_key_event(
    model: &Model,
    key: crossterm::event::KeyEvent,
) -> Result<Option<Message>, HandleEventError> {
    if key.kind == KeyEventKind::Press {
        let msg = match key.code {
            // Quit application
            KeyCode::Char('q') | KeyCode::Esc => Some(AppMsg::Quit.into()),

            // Navigation
            KeyCode::Tab => Some(
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    ScanConfigMsg::PrevField
                } else {
                    ScanConfigMsg::NextField
                }
                .into(),
            ),
            KeyCode::Up => Some(if key.modifiers == KeyModifiers::SHIFT {
                ResultsMsg::ScrollUp(3).into()
            } else {
                ScanConfigMsg::PrevField.into()
            }),
            KeyCode::Down => Some(if key.modifiers == KeyModifiers::SHIFT {
                ResultsMsg::ScrollDown(3).into()
            } else {
                ScanConfigMsg::NextField.into()
            }),
            KeyCode::PageUp => Some(ResultsMsg::ScrollUp(10).into()),
            KeyCode::PageDown => Some(ResultsMsg::ScrollDown(10).into()),
            KeyCode::Home => {
                if key.modifiers == KeyModifiers::CONTROL {
                    Some(ResultsMsg::ScrollToTop.into())
                } else {
                    None
                }
            }
            KeyCode::End => {
                if key.modifiers == KeyModifiers::CONTROL {
                    Some(ResultsMsg::ScrollToBottom.into())
                } else {
                    None
                }
            }

            // Cursor movement within field
            KeyCode::Left => Some(
                if key.modifiers == KeyModifiers::CONTROL {
                    ScanConfigMsg::MovePrevWord
                } else {
                    ScanConfigMsg::MoveCursorLeft
                }
                .into(),
            ),
            KeyCode::Right => Some(
                if key.modifiers == KeyModifiers::CONTROL {
                    ScanConfigMsg::MoveNextWord
                } else {
                    ScanConfigMsg::MoveCursorRight
                }
                .into(),
            ),

            // Input handling
            KeyCode::Enter => match model.scan_config().selected_field {
                SelectedField::ScanButton => Some(ScanConfigMsg::ButtonActivate.into()),
                _ => Some(ScanConfigMsg::ConfirmInput.into()),
            },
            KeyCode::Backspace => Some(
                if key.modifiers == KeyModifiers::CONTROL {
                    ScanConfigMsg::DeletePrevWord
                } else {
                    ScanConfigMsg::RemovePrevChar
                }
                .into(),
            ),
            KeyCode::Delete => Some(
                if key.modifiers == KeyModifiers::CONTROL {
                    ScanConfigMsg::DeleteNextWord
                } else {
                    ScanConfigMsg::RemoveNextChar
                }
                .into(),
            ),
            // Simple alternative key combinations
            KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
                Some(ScanConfigMsg::DeletePrevWord.into())
            }
            KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                Some(ScanConfigMsg::DeleteNextWord.into())
            }
            // Handle terminals that send raw ASCII codes for Ctrl+Backspace/Delete
            KeyCode::Char('\u{08}') => Some(ScanConfigMsg::DeletePrevWord.into()),
            KeyCode::Char('\u{7f}') => Some(ScanConfigMsg::DeleteNextWord.into()),
            KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
                Some(ScanConfigMsg::DeletePrevWord.into())
            }
            // Handle character input
            KeyCode::Char(c) if key.modifiers.is_empty() => Some(ScanConfigMsg::AddChar(c).into()),

            _ => None,
        };
        return Ok(msg);
    }
    Ok(None)
}

/// Handle mouse events
fn handle_mouse_event(
    model: &Model,
    mouse: crossterm::event::MouseEvent,
) -> Result<Option<Message>, HandleEventError> {
    let msg = match mouse.kind {
        MouseEventKind::ScrollUp => Some(ResultsMsg::ScrollUp(3).into()),
        MouseEventKind::ScrollDown => Some(ResultsMsg::ScrollDown(3).into()),
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            // Map hit-testing into selection messages
            handle_component_click(model, mouse.column, mouse.row)
        }
        _ => None,
    };
    Ok(msg)
}

/// Handle mouse click to select components
fn handle_component_click(model: &Model, _column: u16, row: u16) -> Option<Message> {
    // Get current header height based on collapse state
    let current_header_height = if model.is_banner_collapsed() {
        layout::HEADER_HEIGHT_COLLAPSED
    } else {
        layout::HEADER_HEIGHT
    };

    // Check if click is in header area
    if row < current_header_height {
        return Some(AppMsg::ToggleBanner.into());
    }

    // Calculate component positions using dynamic header height
    // Layout structure:
    // - Header: current_header_height (dynamic)
    // - Scan config section border: 1 line
    // - Each component: layout::INPUT_COMPONENT_HEIGHT
    // - Button: layout::BUTTON_HEIGHT

    let scan_config_inner_start = current_header_height + 1;

    if row >= scan_config_inner_start {
        let relative_row = row - scan_config_inner_start;

        // Check if click is in button area (bottom right)
        let button_start_row = layout::INPUT_COMPONENT_HEIGHT * 3; // After 3 input components

        if relative_row >= button_start_row
            && relative_row < button_start_row + layout::BUTTON_HEIGHT
        {
            return Some(ScanConfigMsg::ButtonActivate.into());
        }

        // Check for input component clicks
        let component_index = relative_row / layout::INPUT_COMPONENT_HEIGHT;
        let new_field = match component_index {
            0 => Some(SelectedField::Targets),
            1 => Some(SelectedField::Ports),
            2 => Some(SelectedField::Options),
            _ => None, // Click outside valid component area
        };

        if let Some(field) = new_field {
            return Some(ScanConfigMsg::SelectField(field).into());
        }
        return Some(ScanConfigMsg::DeselectAll.into());
    }
    Some(ScanConfigMsg::DeselectAll.into())
}
