//! Shared visual mode for buttons

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonMode {
    Normal,
    Selected,
    Active,
}

impl Default for ButtonMode {
    fn default() -> Self {
        Self::Normal
    }
}
