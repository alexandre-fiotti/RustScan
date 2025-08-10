use super::model::SelectedField;

#[derive(Debug, Clone)]
pub enum ScanConfigMsg {
    NextField,
    PrevField,
    DeselectAll,
    ConfirmInput,
    ButtonActivate,
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
    Paste(String),
}
