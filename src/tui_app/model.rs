//! Model Module
//!
//! TEA Model: owns all UI-visible state.

use crate::input::Opts;
use crate::tui_app::shared::{OutputBuffer, TextInput};
use crate::tui_app::ui::components::scan_configuration::scan_button::State as ScanButtonState;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectedField {
    None,
    Targets,
    Ports,
    Options,
    ScanButton,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HoveredField {
    None,
    Targets,
    Ports,
    Options,
    ScanButton,
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub targets: Vec<String>,
    pub ports: Option<String>,
    pub timeout: u32,
    pub batch_size: u16,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}

pub struct Model {
    running_state: RunningState,
    opts: Opts,
    scan_config: ScanConfig,
    selected_field: SelectedField,
    hovered_field: HoveredField,
    output_buffer: OutputBuffer,
    banner_collapsed: bool,
    scan_button_state: ScanButtonState,
}

impl Model {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            selected_field: SelectedField::None,
            hovered_field: HoveredField::None,
            output_buffer: OutputBuffer::new(),
            banner_collapsed: false,
            scan_button_state: ScanButtonState::default(),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.running_state == RunningState::Done
    }
    pub fn set_should_quit(&mut self, should_quit: bool) {
        self.running_state = if should_quit {
            RunningState::Done
        } else {
            RunningState::Running
        };
    }

    pub fn opts(&self) -> &Opts {
        &self.opts
    }
    pub fn opts_mut(&mut self) -> &mut Opts {
        &mut self.opts
    }

    pub fn scan_config(&self) -> &ScanConfig {
        &self.scan_config
    }
    pub fn scan_config_mut(&mut self) -> &mut ScanConfig {
        &mut self.scan_config
    }

    pub fn selected_field(&self) -> &SelectedField {
        &self.selected_field
    }
    pub fn scan_button_state(&self) -> &ScanButtonState {
        &self.scan_button_state
    }
    pub fn set_scan_button_state(&mut self, state: ScanButtonState) {
        self.scan_button_state = state;
    }

    pub fn set_selected_field(&mut self, field: SelectedField) {
        self.selected_field = field;
        if let SelectedField::ScanButton = self.selected_field {
            self.scan_button_state = ScanButtonState::Selected;
        }
    }

    pub fn deselect_all(&mut self) {
        self.selected_field = SelectedField::None;
        self.scan_button_state = ScanButtonState::Normal;
    }

    pub fn next_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::None => SelectedField::Targets,
            SelectedField::Targets => SelectedField::Ports,
            SelectedField::Ports => SelectedField::Options,
            SelectedField::Options => SelectedField::ScanButton,
            SelectedField::ScanButton => SelectedField::Targets,
        };
    }

    pub fn prev_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::None => SelectedField::ScanButton,
            SelectedField::Targets => SelectedField::ScanButton,
            SelectedField::Ports => SelectedField::Targets,
            SelectedField::Options => SelectedField::Ports,
            SelectedField::ScanButton => SelectedField::Options,
        };
    }

    pub fn add_char(&mut self, c: char) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.insert_char(c),
            SelectedField::Ports => self.scan_config.ports_input.insert_char(c),
            _ => {}
        }
    }

    pub fn remove_previous_char(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.remove_previous_char(),
            SelectedField::Ports => self.scan_config.ports_input.remove_previous_char(),
            _ => {}
        }
    }

    pub fn remove_next_char(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.remove_next_char(),
            SelectedField::Ports => self.scan_config.ports_input.remove_next_char(),
            _ => {}
        }
    }

    pub fn delete_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.delete_previous_word(),
            SelectedField::Ports => self.scan_config.ports_input.delete_previous_word(),
            _ => {}
        }
    }

    pub fn delete_next_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.delete_next_word(),
            SelectedField::Ports => self.scan_config.ports_input.delete_next_word(),
            _ => {}
        }
    }

    pub fn move_cursor_to_previous_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self
                .scan_config
                .targets_input
                .move_cursor_to_previous_word(),
            SelectedField::Ports => self.scan_config.ports_input.move_cursor_to_previous_word(),
            _ => {}
        }
    }

    pub fn move_cursor_to_next_word(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.move_cursor_to_next_word(),
            SelectedField::Ports => self.scan_config.ports_input.move_cursor_to_next_word(),
            _ => {}
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.move_cursor_left(),
            SelectedField::Ports => self.scan_config.ports_input.move_cursor_left(),
            _ => {}
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.selected_field {
            SelectedField::Targets => self.scan_config.targets_input.move_cursor_right(),
            SelectedField::Ports => self.scan_config.ports_input.move_cursor_right(),
            _ => {}
        }
    }

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
            _ => {}
        }
    }

    pub fn start_scan(&mut self) {}

    pub fn output_buffer(&self) -> &OutputBuffer {
        &self.output_buffer
    }
    pub fn output_buffer_mut(&mut self) -> &mut OutputBuffer {
        &mut self.output_buffer
    }

    pub fn hovered_field(&self) -> &HoveredField {
        &self.hovered_field
    }
    pub fn set_hovered_field(&mut self, field: HoveredField) {
        self.hovered_field = field;
    }

    pub fn is_banner_collapsed(&self) -> bool {
        self.banner_collapsed
    }
    pub fn toggle_banner_collapsed(&mut self) {
        self.banner_collapsed = !self.banner_collapsed;
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
