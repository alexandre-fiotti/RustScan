//! Model Module
//!
//! TEA Model: owns all UI-visible state.

use crate::input::Opts;
use crate::tui_app::scan_config::ScanConfig;
use crate::tui_app::shared::OutputBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}

pub struct Model {
    running_state: RunningState,
    opts: Opts,
    scan_config: ScanConfig,
    output_buffer: OutputBuffer,
    banner_collapsed: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            output_buffer: OutputBuffer::new(),
            banner_collapsed: false,
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

    pub fn output_buffer(&self) -> &OutputBuffer {
        &self.output_buffer
    }
    pub fn output_buffer_mut(&mut self) -> &mut OutputBuffer {
        &mut self.output_buffer
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
