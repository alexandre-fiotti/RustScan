//! Event Handling Module
//!
//! This module handles all input events for the TUI application.
//! It processes keyboard input and updates the application state accordingly.

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use std::io;

use crate::tui_app::ui::theme::layout;
use crate::tui_app::{AppState, SelectedField};

/// Event handler for TUI input processing
pub struct EventHandler;

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self
    }

    /// Handle input events and update application state
    pub fn handle_event(&self, state: &mut AppState, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key) => self.handle_key_event(state, key),
            Event::Mouse(mouse) => self.handle_mouse_event(state, mouse),
            _ => Ok(()),
        }
    }

    /// Handle keyboard events
    fn handle_key_event(
        &self,
        state: &mut AppState,
        key: crossterm::event::KeyEvent,
    ) -> io::Result<()> {
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
                    if key.modifiers == KeyModifiers::SHIFT {
                        // Shift+Up: scroll output up
                        state.output_buffer().scroll_up(3);
                    } else {
                        // Regular Up: navigate fields
                        state.prev_field();
                    }
                }
                KeyCode::Down => {
                    if key.modifiers == KeyModifiers::SHIFT {
                        // Shift+Down: scroll output down
                        state.output_buffer().scroll_down(3);
                    } else {
                        // Regular Down: navigate fields
                        state.next_field();
                    }
                }
                KeyCode::PageUp => {
                    // Page Up: scroll output up by page
                    state.output_buffer().scroll_up(10);
                }
                KeyCode::PageDown => {
                    // Page Down: scroll output down by page
                    state.output_buffer().scroll_down(10);
                }
                KeyCode::Home => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+Home: scroll to top of output
                        state.output_buffer().scroll_to_top();
                    }
                }
                KeyCode::End => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+End: scroll to bottom of output (live view)
                        state.output_buffer().scroll_to_bottom();
                    }
                }

                // Cursor movement within field
                KeyCode::Left => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+Left: move to previous word
                        state.move_cursor_to_previous_word();
                    } else {
                        // Regular left: move cursor left
                        state.move_cursor_left();
                    }
                }
                KeyCode::Right => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+Right: move to next word
                        state.move_cursor_to_next_word();
                    } else {
                        // Regular right: move cursor right
                        state.move_cursor_right();
                    }
                }

                // Input handling
                KeyCode::Enter => {
                    state.confirm_input();
                }
                KeyCode::Backspace => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+Backspace: delete previous word
                        state.delete_previous_word();
                    } else {
                        // Regular backspace: delete previous character
                        state.remove_previous_char();
                    }
                }
                KeyCode::Delete => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        // Ctrl+Delete: delete next word
                        state.delete_next_word();
                    } else {
                        // Regular delete: delete next character
                        state.remove_next_char();
                    }
                }
                // Simple alternative key combinations
                KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
                    // Ctrl+W: delete previous word
                    state.delete_previous_word();
                }
                KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                    // Ctrl+D: delete next word
                    state.delete_next_word();
                }
                // Handle terminals that send raw ASCII codes for Ctrl+Backspace/Delete
                KeyCode::Char('\u{08}') => {
                    // ASCII BS (backspace) - some terminals send this for Ctrl+Backspace
                    state.delete_previous_word();
                }
                KeyCode::Char('\u{7f}') => {
                    // ASCII DEL - some terminals send this for Ctrl+Delete
                    state.delete_next_word();
                }
                KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
                    // Ctrl+H - some terminals send this for Ctrl+Backspace
                    state.delete_previous_word();
                }
                // Handle character input
                KeyCode::Char(c) if key.modifiers.is_empty() => {
                    state.add_char(c);
                }

                _ => {}
            }
        }
        Ok(())
    }

    /// Handle mouse events
    fn handle_mouse_event(
        &self,
        state: &mut AppState,
        mouse: crossterm::event::MouseEvent,
    ) -> io::Result<()> {
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                state.output_buffer().scroll_up(3);
            }
            MouseEventKind::ScrollDown => {
                state.output_buffer().scroll_down(3);
            }
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                // Handle left mouse click for component selection
                self.handle_component_click(state, mouse.column, mouse.row);
            }
            // Removed mouse movement tracking to prevent any interference
            _ => {}
        }
        Ok(())
    }

    /// Handle mouse click to select components
    fn handle_component_click(&self, state: &mut AppState, _column: u16, row: u16) {
        // Get current header height based on collapse state
        let current_header_height = if state.is_banner_collapsed() {
            layout::HEADER_HEIGHT_COLLAPSED
        } else {
            layout::HEADER_HEIGHT
        };

        // Check if click is in header area
        if row < current_header_height {
            // Click on header - toggle banner collapsed state
            state.toggle_banner_collapsed();
            return;
        }

        // Calculate component positions using dynamic header height
        // Layout structure:
        // - Header: current_header_height (dynamic)
        // - Scan config section border: 1 line
        // - Scan config internal margin: layout::STANDARD_MARGIN
        // - Each component: layout::INPUT_COMPONENT_HEIGHT

        let scan_config_inner_start = current_header_height + 1 + layout::STANDARD_MARGIN;

        if row >= scan_config_inner_start {
            let relative_row = row - scan_config_inner_start;
            let component_index = relative_row / layout::INPUT_COMPONENT_HEIGHT;

            let new_field = match component_index {
                0 => Some(SelectedField::Targets),
                1 => Some(SelectedField::Ports),
                2 => Some(SelectedField::Options),
                _ => None, // Click outside valid component area
            };

            if let Some(field) = new_field {
                state.set_selected_field(field);
            } else {
                // Click outside component area - deselect
                state.deselect_all();
            }
        } else {
            // Click outside scan config area - deselect
            state.deselect_all();
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
