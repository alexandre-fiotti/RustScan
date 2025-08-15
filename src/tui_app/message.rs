//! TEA Messages and Model alias

use crate::tui_app::{model::FocusedArea, results::ResultsMsg, scan_config::ScanConfigMsg};

/// Top-level application messages
#[derive(Debug, Clone)]
pub enum AppMsg {
    Quit,
    ToggleBanner,
    StartScan,
    StopScan,
    SetFocus(FocusedArea),
}

/// Unified message for the application that wraps component messages
#[derive(Debug, Clone)]
pub enum Message {
    App(AppMsg),
    ScanConfig(ScanConfigMsg),
    Results(ResultsMsg),
}

impl From<AppMsg> for Message {
    fn from(value: AppMsg) -> Self {
        Self::App(value)
    }
}
impl From<ScanConfigMsg> for Message {
    fn from(value: ScanConfigMsg) -> Self {
        Self::ScanConfig(value)
    }
}
impl From<ResultsMsg> for Message {
    fn from(value: ResultsMsg) -> Self {
        Self::Results(value)
    }
}
