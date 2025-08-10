//! TEA Messages and Model alias

use crate::tui_app::model::SelectedField;

/// Top-level application messages
#[derive(Debug, Clone)]
pub enum AppMsg {
    Quit,
    ToggleBanner,
    NextField,
    PrevField,
    DeselectAll,
    ButtonActivate,
    StartScan,
    ConfirmInput,
}

/// Messages for the scan configuration component
#[derive(Debug, Clone)]
pub enum ScanConfigMsg {
    SelectField(SelectedField),
    AddChar(char),
    RemovePrevChar,
    RemoveNextChar,
    DeletePrevWord,
    DeleteNextWord,
    MoveCursorLeft,
    MoveCursorRight,
    MovePrevWord,
    MoveNextWord,
}

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
