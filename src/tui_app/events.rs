//! Event Handling Module
//!
//! This module handles all input events for the TUI application.
//! It processes keyboard input and updates the application state accordingly.

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;

use crate::tui_app::AppState;

/// Event handler for TUI input processing
pub struct EventHandler;

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self
    }

    /// Handle input events and update application state
    pub fn handle_event(&self, state: &mut AppState, event: Event) -> io::Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    // Quit application
                    KeyCode::Char('q') | KeyCode::Esc => {
                        state.set_should_quit(true);
                    }

                    // Navigation
                    KeyCode::Tab => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            state.prev_field();
                        } else {
                            state.next_field();
                        }
                    }
                    KeyCode::Up => {
                        state.prev_field();
                    }
                    KeyCode::Down => {
                        state.next_field();
                    }

                    // Cursor movement within field
                    KeyCode::Left => {
                        state.move_cursor_left();
                    }
                    KeyCode::Right => {
                        state.move_cursor_right();
                    }

                    // Input handling
                    KeyCode::Enter => {
                        state.confirm_input();
                    }
                    KeyCode::Backspace => {
                        state.remove_previous_char();
                    }
                    KeyCode::Delete => {
                        state.remove_next_char();
                    }
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.add_char(c);
                    }

                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
