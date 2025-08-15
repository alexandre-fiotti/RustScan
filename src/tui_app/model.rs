//! Model Module
//!
//! TEA Model: owns all UI-visible state.

use std::{sync::mpsc::Receiver, thread::JoinHandle};

use crate::{
    input::Opts,
    tui_app::{message::Message, results::ResultsModel, scan_config::ScanConfig},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedArea {
    ScanConfig,
    Results,
    Header,
    None,
}

pub struct Model {
    running_state: RunningState,
    opts: Opts,
    scan_config: ScanConfig,
    results: ResultsModel,
    banner_collapsed: bool,
    scan_state: ScanState,
    scan_results_rx: Option<Receiver<Message>>,
    scan_handle: Option<JoinHandle<()>>,
    focused_area: FocusedArea,
}

impl Model {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            opts: Opts::default(),
            scan_config: ScanConfig::default(),
            results: ResultsModel::default(),
            banner_collapsed: false,
            scan_state: ScanState::Idle,
            scan_results_rx: None,
            scan_handle: None,
            focused_area: FocusedArea::ScanConfig,
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

    pub fn results(&self) -> &ResultsModel {
        &self.results
    }
    pub fn results_mut(&mut self) -> &mut ResultsModel {
        &mut self.results
    }

    pub fn is_banner_collapsed(&self) -> bool {
        self.banner_collapsed
    }
    pub fn toggle_banner_collapsed(&mut self) {
        self.banner_collapsed = !self.banner_collapsed
    }

    pub fn scan_state(&self) -> ScanState {
        self.scan_state
    }
    pub fn set_scan_state(&mut self, state: ScanState) {
        self.scan_state = state;
    }

    pub fn take_scan_results_rx(&mut self) -> Option<Receiver<Message>> {
        self.scan_results_rx.take()
    }
    pub fn set_scan_results_rx(&mut self, rx: Receiver<Message>) {
        self.scan_results_rx = Some(rx);
    }

    pub fn scan_results_rx_ref(&mut self) -> Option<&mut Receiver<Message>> {
        self.scan_results_rx.as_mut()
    }

    pub fn set_scan_handle(&mut self, handle: JoinHandle<()>) {
        self.scan_handle = Some(handle);
    }
    pub fn take_scan_handle(&mut self) -> Option<JoinHandle<()>> {
        self.scan_handle.take()
    }

    pub fn focused_area(&self) -> FocusedArea {
        self.focused_area
    }
    pub fn set_focused_area(&mut self, area: FocusedArea) {
        self.focused_area = area;
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanState {
    Idle,
    Requested,
    Running,
    Completed,
}
