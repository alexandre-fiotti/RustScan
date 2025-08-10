use std::time::{Duration, Instant};

use crate::tui_app::shared::{button_mode::ButtonMode as ScanButtonMode, TextInput};

#[derive(Debug, Clone, PartialEq)]
pub enum SelectedField {
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
    pub selected_field: SelectedField,
    pub scan_button_mode: ScanButtonMode,
    pub button_activation_until: Option<Instant>,
    pub button_restore_mode: Option<ScanButtonMode>,
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
            selected_field: SelectedField::None,
            scan_button_mode: ScanButtonMode::default(),
            button_activation_until: None,
            button_restore_mode: None,
        }
    }
}

impl ScanConfig {
    pub fn set_selected_field(&mut self, field: SelectedField) {
        self.selected_field = field;
        self.scan_button_mode = if matches!(self.selected_field, SelectedField::ScanButton) {
            ScanButtonMode::Selected
        } else {
            ScanButtonMode::Normal
        };
    }

    pub fn deselect_all(&mut self) {
        self.selected_field = SelectedField::None;
        self.scan_button_mode = ScanButtonMode::Normal;
    }

    pub fn next_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::None => SelectedField::Targets,
            SelectedField::Targets => SelectedField::Ports,
            SelectedField::Ports => SelectedField::Options,
            SelectedField::Options => SelectedField::ScanButton,
            SelectedField::ScanButton => SelectedField::Targets,
        };
        self.scan_button_mode = if matches!(self.selected_field, SelectedField::ScanButton) {
            ScanButtonMode::Selected
        } else {
            ScanButtonMode::Normal
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
        self.scan_button_mode = if matches!(self.selected_field, SelectedField::ScanButton) {
            ScanButtonMode::Selected
        } else {
            ScanButtonMode::Normal
        };
    }

    pub fn start_button_activation(&mut self) {
        let restore = if matches!(self.selected_field, SelectedField::ScanButton) {
            ScanButtonMode::Selected
        } else {
            ScanButtonMode::Normal
        };
        self.scan_button_mode = ScanButtonMode::Active;
        self.button_activation_until = Some(Instant::now() + Duration::from_millis(200));
        self.button_restore_mode = Some(restore);
    }

    pub fn maybe_finish_button_activation(&mut self) -> bool {
        if let Some(until) = self.button_activation_until {
            if Instant::now() >= until {
                let restore = self
                    .button_restore_mode
                    .take()
                    .unwrap_or(ScanButtonMode::Normal);
                self.scan_button_mode = restore;
                self.button_activation_until = None;
                return true;
            }
        }
        false
    }

    pub fn selected_text_input_mut(&mut self) -> Option<&mut TextInput> {
        match self.selected_field {
            SelectedField::Targets => Some(&mut self.targets_input),
            SelectedField::Ports => Some(&mut self.ports_input),
            _ => None,
        }
    }
}
