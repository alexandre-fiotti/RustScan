//! Application State Module
//!
//! This module manages the state of the TUI application.
//! It provides a clean separation between UI logic and state management.

use crate::input::Opts;
use crate::tui_app::models::{OutputBuffer, TextInput};
use crate::tui_app::ui::components::scan_configuration::scan_button::State as ScanButtonState;

/// Which input field is currently selected
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedField {
    None,
    Targets,
    Ports,
    Options,
    ScanButton,
}

/// Which component is currently being hovered over by the mouse
#[derive(Debug, Clone, PartialEq)]
pub enum HoveredField {
    None,
    Targets,
    Ports,
    Options,
    ScanButton,
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
    /// Whether the banner is collapsed to a single line
    banner_collapsed: bool,
    /// Whether the scan button is currently being pressed (active state)
    scan_button_state: ScanButtonState,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            should_quit: false,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            selected_field: SelectedField::None,
            hovered_field: HoveredField::None,
            output_buffer: OutputBuffer::new(),
            banner_collapsed: false,
            scan_button_state: ScanButtonState::default(),
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

    /// Get the scan button state
    pub fn scan_button_state(&self) -> &ScanButtonState {
        &self.scan_button_state
    }

    /// Set the scan button state
    pub fn set_scan_button_state(&mut self, state: ScanButtonState) {
        self.scan_button_state = state;
    }

    /// Set the selected field
    pub fn set_selected_field(&mut self, field: SelectedField) {
        self.selected_field = field;
        match self.selected_field {
            SelectedField::ScanButton => {
                self.scan_button_state = ScanButtonState::Selected;
            }
            _ => {}
        }
    }

    /// Deselect all components
    pub fn deselect_all(&mut self) {
        self.selected_field = SelectedField::None;
        self.scan_button_state = ScanButtonState::Normal;
    }

    /// Navigate to next field
    pub fn next_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::None => SelectedField::Targets, // Arrow down from no selection goes to top (Targets)
            SelectedField::Targets => SelectedField::Ports,
            SelectedField::Ports => SelectedField::Options,
            SelectedField::Options => SelectedField::ScanButton,
            SelectedField::ScanButton => SelectedField::Targets,
        };
    }

    /// Navigate to previous field  
    pub fn prev_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::None => SelectedField::ScanButton, // Arrow up from no selection goes to bottom (Button)
            SelectedField::Targets => SelectedField::ScanButton,
            SelectedField::Ports => SelectedField::Targets,
            SelectedField::Options => SelectedField::Ports,
            SelectedField::ScanButton => SelectedField::Options,
        };
    }

    /// Add character to currently selected input field
    pub fn add_char(&mut self, c: char) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.insert_char(c);
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.insert_char(c);
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
            _ => {}
        }
    }

    /// Remove last character from currently selected input field
    pub fn remove_previous_char(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.remove_previous_char();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.remove_previous_char();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
            _ => {}
        }
    }

    /// Remove next character from currently selected input field
    pub fn remove_next_char(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.remove_next_char();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.remove_next_char();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
            _ => {}
        }
    }

    /// Delete previous word from currently selected input field (Ctrl+Backspace)
    pub fn delete_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.delete_previous_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.delete_previous_word();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
            _ => {}
        }
    }

    /// Delete next word from currently selected input field (Alt+Delete)
    pub fn delete_next_word(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.delete_next_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.delete_next_word();
            }
            SelectedField::Options => {
                // TODO: Implement options functionality later
            }
            _ => {}
        }
    }

    /// Move cursor to previous word in current field (Ctrl+Left)
    pub fn move_cursor_to_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config
                    .targets_input
                    .move_cursor_to_previous_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_to_previous_word();
            }
            SelectedField::Options => {}
            _ => {}
        }
    }

    /// Move cursor to next word in current field (Ctrl+Right)
    pub fn move_cursor_to_next_word(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_to_next_word();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_to_next_word();
            }
            SelectedField::Options => {}
            _ => {}
        }
    }

    /// Move cursor left in current field
    pub fn move_cursor_left(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_left();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_left();
            }
            SelectedField::Options => {}
            _ => {}
        }
    }

    /// Move cursor right in current field
    pub fn move_cursor_right(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
            SelectedField::Targets => {
                self.scan_config.targets_input.move_cursor_right();
            }
            SelectedField::Ports => {
                self.scan_config.ports_input.move_cursor_right();
            }
            SelectedField::Options => {}
            _ => {}
        }
    }

    /// Confirm input for currently selected field
    pub fn confirm_input(&mut self) {
        match self.selected_field {
            SelectedField::None => {
                // Do nothing when no field is selected
            }
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
                // TODO: Implement options functionality later
            }
            SelectedField::ScanButton => {
                // Scan button activation is now handled in the event handler
                // This case should not be reached anymore
            }
        }
    }

    /// Start a scan with current configuration
    pub fn start_scan(&mut self) {
        // TODO: Implement actual scan functionality
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

    /// Get whether the banner is collapsed
    pub fn is_banner_collapsed(&self) -> bool {
        self.banner_collapsed
    }

    /// Toggle the banner collapsed state
    pub fn toggle_banner_collapsed(&mut self) {
        self.banner_collapsed = !self.banner_collapsed;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
