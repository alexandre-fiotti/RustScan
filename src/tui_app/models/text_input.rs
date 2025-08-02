//! Text Input Component
//!
//! A reusable text input component that handles cursor management,
//! character insertion, and deletion operations.

/// A text input field with cursor management
#[derive(Debug, Clone)]
pub struct TextInput {
    /// The text content
    text: String,
    /// Current cursor position (character index)
    cursor: usize,
}

impl TextInput {
    /// Create a new empty text input
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }

    /// Create a text input with initial content
    pub fn with_text(text: String) -> Self {
        let cursor = text.chars().count();
        Self { text, cursor }
    }

    /// Get the current text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the current cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Set the text content and reset cursor to end
    pub fn set_text(&mut self, text: String) {
        self.cursor = text.chars().count();
        self.text = text;
    }

    /// Clear the text content and reset cursor
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    /// Check if the input is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Insert a character at the current cursor position
    pub fn insert_char(&mut self, c: char) {
        let byte_index = self.byte_index();
        self.text.insert(byte_index, c);
        self.move_cursor_right();
    }

    /// Remove the character before the cursor (backspace)
    pub fn remove_previous_char(&mut self) {
        if self.cursor > 0 {
            let current_index = self.cursor;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.text.chars().skip(current_index);

            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    /// Remove the character at the cursor position (delete)
    pub fn remove_next_char(&mut self) {
        if self.cursor < self.text.chars().count() {
            let current_index = self.cursor;

            let before_char_to_delete = self.text.chars().take(current_index);
            let after_char_to_delete = self.text.chars().skip(current_index + 1);

            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            // Note: cursor stays in the same position since we deleted the char ahead
        }
    }

    /// Move cursor left by one character
    pub fn move_cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Move cursor right by one character
    pub fn move_cursor_right(&mut self) {
        let max_pos = self.text.chars().count();
        self.cursor = (self.cursor + 1).min(max_pos);
    }

    /// Set cursor to a specific position (clamped to valid range)
    pub fn set_cursor(&mut self, position: usize) {
        let max_pos = self.text.chars().count();
        self.cursor = position.min(max_pos);
    }

    /// Get the byte index for the current cursor position
    fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.text.len())
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}
