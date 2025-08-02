//! Application State Module
//!
//! This module manages the state of the TUI application.
//! It provides a clean separation between UI logic and state management.

use crate::input::Opts;
use crate::tui_app::models::{OutputBuffer, TextInput};

/// Which input field is currently selected
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedField {
    Targets,
    Ports,
    Options,
}

/// Which component is currently being hovered over by the mouse
#[derive(Debug, Clone, PartialEq)]
pub enum HoveredField {
    None,
    Targets,
    Ports,
    Options,
}

/// Simple scan configuration state
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Target addresses
    pub targets: Vec<String>,
    /// Port configuration
    pub ports: Option<String>,
    /// Timeout in milliseconds
    pub timeout: u32,
    /// Batch size
    pub batch_size: u16,
    /// Text input components
    pub targets_input: TextInput,
    pub ports_input: TextInput,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
            ports: None,
            timeout: 1500,
            batch_size: 4500,
            targets_input: TextInput::new(),
            ports_input: TextInput::new(),
        }
    }
}

/// Main application state
pub struct AppState {
    /// Whether the app should quit
    should_quit: bool,
    /// Configuration options (from CLI)
    opts: Opts,
    /// Scan configuration (for TUI)
    scan_config: ScanConfig,
    /// Currently selected input field
    selected_field: SelectedField,
    /// Currently hovered field (for mouse interaction feedback)
    hovered_field: HoveredField,
    /// Terminal output buffer for displaying all output
    output_buffer: OutputBuffer,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            should_quit: false,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            selected_field: SelectedField::Targets,
            hovered_field: HoveredField::None,
            output_buffer: OutputBuffer::new(),
        }
    }

    /// Check if the app should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Set the quit flag
    pub fn set_should_quit(&mut self, should_quit: bool) {
        self.should_quit = should_quit;
    }

    /// Get the options
    pub fn opts(&self) -> &Opts {
        &self.opts
    }

    /// Get mutable options
    pub fn opts_mut(&mut self) -> &mut Opts {
        &mut self.opts
    }

    /// Get the scan configuration
    pub fn scan_config(&self) -> &ScanConfig {
        &self.scan_config
    }

    /// Get mutable scan configuration
    pub fn scan_config_mut(&mut self) -> &mut ScanConfig {
        &mut self.scan_config
    }

    /// Get the currently selected field
    pub fn selected_field(&self) -> &SelectedField {
        &self.selected_field
    }

    /// Set the selected field
    pub fn set_selected_field(&mut self, field: SelectedField) {
        self.selected_field = field;
    }

    /// Navigate to next field
    pub fn next_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::Targets => SelectedField::Ports,
            SelectedField::Ports => SelectedField::Options,
            SelectedField::Options => SelectedField::Targets,
        };
    }

    /// Navigate to previous field  
    pub fn prev_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::Targets => SelectedField::Options,
            SelectedField::Ports => SelectedField::Targets,
            SelectedField::Options => SelectedField::Ports,
        };
    }

    /// Add character to currently selected input field
    pub fn add_char(&mut self, c: char) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.insert_char(c);
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.insert_char(c);
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
        }
    }

    /// Remove last character from currently selected input field
    pub fn remove_previous_char(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.remove_previous_char();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.remove_previous_char();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
        }
    }

    /// Remove next character from currently selected input field
    pub fn remove_next_char(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.remove_next_char();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.remove_next_char();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
        }
    }

    /// Delete previous word from currently selected input field (Ctrl+Backspace)
    pub fn delete_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.delete_previous_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.delete_previous_word();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
        }
    }

    /// Delete next word from currently selected input field (Alt+Delete)
    pub fn delete_next_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.delete_next_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.delete_next_word();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
        }
    }

    /// Move cursor to previous word in current field (Ctrl+Left)
    pub fn move_cursor_to_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config
                    .targets_input
                    .move_cursor_to_previous_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_to_previous_word();
            }
            SelectedField::Options => {}
        }
    }

    /// Move cursor to next word in current field (Ctrl+Right)
    pub fn move_cursor_to_next_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_to_next_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_to_next_word();
            }
            SelectedField::Options => {}
        }
    }

    /// Move cursor left in current field
    pub fn move_cursor_left(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_left();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_left();
            }
            SelectedField::Options => {}
        }
    }

    /// Move cursor right in current field
    pub fn move_cursor_right(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_right();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_right();
            }
            SelectedField::Options => {}
        }
    }

    /// Confirm input for currently selected field
    pub fn confirm_input(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                if !self.scan_config.targets_input.is_empty() {
                    self.scan_config.targets = self
                        .scan_config
                        .targets_input
                        .text()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                self.scan_config.targets_input.clear();
            }
            SelectedField::Ports => {
                if !self.scan_config.ports_input.is_empty() {
                    self.scan_config.ports = Some(self.scan_config.ports_input.text().to_string());
                } else {
                    self.scan_config.ports = None;
                }
                self.scan_config.ports_input.clear();
            }
            SelectedField::Options => {
                // TODO: Implement scan start functionality later
            }
        }
    }

    /// Get reference to the output buffer
    pub fn output_buffer(&self) -> &OutputBuffer {
        &self.output_buffer
    }

    /// Get mutable reference to the output buffer
    pub fn output_buffer_mut(&mut self) -> &mut OutputBuffer {
        &mut self.output_buffer
    }

    /// Get the currently hovered field
    pub fn hovered_field(&self) -> &HoveredField {
        &self.hovered_field
    }

    /// Set the hovered field
    pub fn set_hovered_field(&mut self, field: HoveredField) {
        self.hovered_field = field;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
