//! TEA Messages and Model alias

use crate::tui_app::scan_config::ScanConfigMsg;

/// Top-level application messages
#[derive(Debug, Clone)]
pub enum AppMsg {
    Quit,
    ToggleBanner,
    StartScan,
}

// Scan configuration messages are defined in scan_config/message.rs and re-exported here

/// Messages for the results/output component
#[derive(Debug, Clone)]
pub enum ResultsMsg {
    ScrollUp(usize),
    ScrollDown(usize),
    ScrollToTop,
    ScrollToBottom,
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
