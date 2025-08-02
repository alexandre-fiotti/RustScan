//! Application State Module
//!
//! This module manages the state of the TUI application.
//! It provides a clean separation between UI logic and state management.

use crate::input::Opts;

/// Which input field is currently selected
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedField {
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
    /// Input buffers for editing
    pub targets_input: String,
    pub ports_input: String,
    /// Cursor positions for input fields
    pub targets_cursor: usize,
    pub ports_cursor: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
            ports: None,
            timeout: 1500,
            batch_size: 4500,
            targets_input: String::new(),
            ports_input: String::new(),
            targets_cursor: 0,
            ports_cursor: 0,
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
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            should_quit: false,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            selected_field: SelectedField::Targets,
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
                let index = self.byte_index_targets();
                self.scan_config.targets_input.insert(index, c);
                self.move_targets_cursor_right();
            }
            SelectedField::Ports => {
                let index = self.byte_index_ports();
                self.scan_config.ports_input.insert(index, c);
                self.move_ports_cursor_right();
            }
            SelectedField::Options => {
                // Options don't support text input yet
            }
        }
    }

    /// Remove last character from currently selected input field
    pub fn remove_char(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.delete_targets_char();
            }
            SelectedField::Ports => {
                self.delete_ports_char();
            }
            SelectedField::Options => {
                // Options don't support text input yet
            }
        }
    }

    /// Move cursor left in current field
    pub fn move_cursor_left(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.move_targets_cursor_left();
            }
            SelectedField::Ports => {
                self.move_ports_cursor_left();
            }
            SelectedField::Options => {}
        }
    }

    /// Move cursor right in current field
    pub fn move_cursor_right(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                self.move_targets_cursor_right();
            }
            SelectedField::Ports => {
                self.move_ports_cursor_right();
            }
            SelectedField::Options => {}
        }
    }

    // Helper methods for targets field
    fn byte_index_targets(&self) -> usize {
        self.scan_config
            .targets_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.scan_config.targets_cursor)
            .unwrap_or(self.scan_config.targets_input.len())
    }

    fn move_targets_cursor_left(&mut self) {
        let cursor_moved_left = self.scan_config.targets_cursor.saturating_sub(1);
        self.scan_config.targets_cursor = self.clamp_targets_cursor(cursor_moved_left);
    }

    fn move_targets_cursor_right(&mut self) {
        let cursor_moved_right = self.scan_config.targets_cursor.saturating_add(1);
        self.scan_config.targets_cursor = self.clamp_targets_cursor(cursor_moved_right);
    }

    fn clamp_targets_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.scan_config.targets_input.chars().count())
    }

    fn delete_targets_char(&mut self) {
        let is_not_cursor_leftmost = self.scan_config.targets_cursor != 0;
        if is_not_cursor_leftmost {
            let current_index = self.scan_config.targets_cursor;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self
                .scan_config
                .targets_input
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.scan_config.targets_input.chars().skip(current_index);

            self.scan_config.targets_input =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_targets_cursor_left();
        }
    }

    // Helper methods for ports field
    fn byte_index_ports(&self) -> usize {
        self.scan_config
            .ports_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.scan_config.ports_cursor)
            .unwrap_or(self.scan_config.ports_input.len())
    }

    fn move_ports_cursor_left(&mut self) {
        let cursor_moved_left = self.scan_config.ports_cursor.saturating_sub(1);
        self.scan_config.ports_cursor = self.clamp_ports_cursor(cursor_moved_left);
    }

    fn move_ports_cursor_right(&mut self) {
        let cursor_moved_right = self.scan_config.ports_cursor.saturating_add(1);
        self.scan_config.ports_cursor = self.clamp_ports_cursor(cursor_moved_right);
    }

    fn clamp_ports_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.scan_config.ports_input.chars().count())
    }

    fn delete_ports_char(&mut self) {
        let is_not_cursor_leftmost = self.scan_config.ports_cursor != 0;
        if is_not_cursor_leftmost {
            let current_index = self.scan_config.ports_cursor;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self
                .scan_config
                .ports_input
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.scan_config.ports_input.chars().skip(current_index);

            self.scan_config.ports_input =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_ports_cursor_left();
        }
    }

    /// Reset cursor when confirming input
    fn reset_targets_cursor(&mut self) {
        self.scan_config.targets_cursor = 0;
    }

    fn reset_ports_cursor(&mut self) {
        self.scan_config.ports_cursor = 0;
    }

    /// Confirm input for currently selected field
    pub fn confirm_input(&mut self) {
        match self.selected_field {
            SelectedField::Targets => {
                if !self.scan_config.targets_input.is_empty() {
                    self.scan_config.targets = self
                        .scan_config
                        .targets_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                self.reset_targets_cursor();
            }
            SelectedField::Ports => {
                if !self.scan_config.ports_input.is_empty() {
                    self.scan_config.ports = Some(self.scan_config.ports_input.clone());
                } else {
                    self.scan_config.ports = None;
                }
                self.reset_ports_cursor();
            }
            SelectedField::Options => {
                // TODO: Start scan
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
