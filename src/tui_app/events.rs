//! Event → Message mapping (TEA)

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::tui_app::{
    message::{AppMsg, Message},
    model::{FocusedArea, Model, ScanState},
    results::ResultsMsg,
    scan_config::{ScanConfigMsg, SelectedField},
    ui::theme::layout,
};

#[derive(Debug)]
pub enum HandleEventError {
    Io(std::io::Error),
}

impl From<std::io::Error> for HandleEventError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Map an input event to an optional Message
pub fn handle_event(model: &Model, event: Event) -> Result<Option<Message>, HandleEventError> {
    let msg = match event {
        Event::Key(key) => handle_key_event(model, key)?,
        Event::Mouse(mouse) => handle_mouse_event(model, mouse)?,
        Event::Paste(pasted) => Some(ScanConfigMsg::Paste(pasted).into()),
        Event::Resize(_, _) => None,
        _ => None,
    };
    Ok(msg)
}

/// Handle keyboard events
fn handle_key_event(model: &Model, key: KeyEvent) -> Result<Option<Message>, HandleEventError> {
    if key.kind == KeyEventKind::Press {
        // 1) Global shortcuts (independent of focus)
        if let Some(global) = handle_key_global(model, key) {
            return Ok(Some(global));
        }
        // 2) Route by focused area
        let routed = match model.focused_area() {
            FocusedArea::ScanConfig => handle_key_scan_config(key),
            FocusedArea::Results => handle_key_results(key),
            FocusedArea::Header => handle_key_header(key),
            FocusedArea::None => handle_key_none(key),
        };
        return Ok(routed);
    }
    Ok(None)
}

// Global shortcuts: quit, stop scan, scrolling with PageUp/PageDown and Ctrl+Home/End,
// Shift+Up/Down scroll results, Enter starts scan
fn handle_key_global(model: &Model, key: KeyEvent) -> Option<Message> {
    match key.code {
        // Quit application
        KeyCode::Char('q') | KeyCode::Esc => Some(AppMsg::Quit.into()),
        // Stop scan (only when a scan is active)
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => match model.scan_state() {
            ScanState::Running | ScanState::Requested => Some(AppMsg::StopScan.into()),
            _ => None,
        },
        // Enter: Start scan
        KeyCode::Enter => Some(AppMsg::StartScan.into()),

        // Results scrolling (global)
        KeyCode::PageUp => Some(ResultsMsg::ScrollUp(10).into()),
        KeyCode::PageDown => Some(ResultsMsg::ScrollDown(10).into()),
        KeyCode::Home if key.modifiers == KeyModifiers::CONTROL => {
            Some(ResultsMsg::ScrollToTop.into())
        }
        KeyCode::End if key.modifiers == KeyModifiers::CONTROL => {
            Some(ResultsMsg::ScrollToBottom.into())
        }
        KeyCode::Up if key.modifiers == KeyModifiers::SHIFT => Some(ResultsMsg::ScrollUp(3).into()),
        KeyCode::Down if key.modifiers == KeyModifiers::SHIFT => {
            Some(ResultsMsg::ScrollDown(3).into())
        }

        _ => None,
    }
}

// Keys for scan configuration area
fn handle_key_scan_config(key: KeyEvent) -> Option<Message> {
    match key.code {
        // Intra-form navigation
        KeyCode::Tab => Some(
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                ScanConfigMsg::PrevField
            } else {
                ScanConfigMsg::NextField
            }
            .into(),
        ),
        KeyCode::Up if key.modifiers.is_empty() => Some(ScanConfigMsg::PrevField.into()),
        KeyCode::Down if key.modifiers.is_empty() => Some(ScanConfigMsg::NextField.into()),

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

        // Editing
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
        // Alternative key combos
        KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
            Some(ScanConfigMsg::DeletePrevWord.into())
        }
        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
            Some(ScanConfigMsg::DeleteNextWord.into())
        }
        // Terminals sending raw ASCII for Ctrl+Backspace/Delete
        KeyCode::Char('\u{08}') => Some(ScanConfigMsg::DeletePrevWord.into()),
        KeyCode::Char('\u{7f}') => Some(ScanConfigMsg::DeleteNextWord.into()),
        KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
            Some(ScanConfigMsg::DeletePrevWord.into())
        }
        // Plain character input
        KeyCode::Char(c) if key.modifiers.is_empty() => Some(ScanConfigMsg::AddChar(c).into()),

        _ => None,
    }
}

// Keys for results area (beyond global scrolling if needed)
fn handle_key_results(_key: KeyEvent) -> Option<Message> {
    // No extra per-results keys beyond global ones for now
    None
}

// Keys when no area is focused: allow basic navigation for ScanConfig
fn handle_key_none(key: KeyEvent) -> Option<Message> {
    match (key.code, key.modifiers) {
        (KeyCode::Up, m) if m.is_empty() => Some(ScanConfigMsg::PrevField.into()),
        (KeyCode::Down, m) if m.is_empty() => Some(ScanConfigMsg::NextField.into()),
        _ => None,
    }
}

// Keys for header area
fn handle_key_header(_key: KeyEvent) -> Option<Message> {
    // No header-specific keys for now
    None
}

/// Handle mouse events
fn handle_mouse_event(
    model: &Model,
    mouse: MouseEvent,
) -> Result<Option<Message>, HandleEventError> {
    let msg = match mouse.kind {
        MouseEventKind::ScrollUp => Some(ResultsMsg::ScrollUp(3).into()),
        MouseEventKind::ScrollDown => Some(ResultsMsg::ScrollDown(3).into()),
        MouseEventKind::Down(MouseButton::Left) => {
            // Map hit-testing into selection messages
            handle_component_click(model, mouse.column, mouse.row)
        }
        _ => None,
    };
    Ok(msg)
}

// Clipboard access is deliberately not implemented to avoid extra dependencies.

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
