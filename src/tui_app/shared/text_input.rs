//! Text Input primitive (data structure)

#[derive(Debug, Clone)]
pub struct TextInput {
    text: String,
    cursor: usize,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }

    pub fn with_text(text: String) -> Self {
        let cursor = text.chars().count();
        Self { text, cursor }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_text(&mut self, text: String) {
        self.cursor = text.chars().count();
        self.text = text;
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn insert_char(&mut self, c: char) {
        let byte_index = self.byte_index();
        self.text.insert(byte_index, c);
        self.move_cursor_right();
    }

    pub fn remove_previous_char(&mut self) {
        if self.cursor > 0 {
            let current_index = self.cursor;
            let before = self.text.chars().take(current_index - 1);
            let after = self.text.chars().skip(current_index);
            self.text = before.chain(after).collect();
            self.move_cursor_left();
        }
    }

    pub fn remove_next_char(&mut self) {
        if self.cursor < self.text.chars().count() {
            let current_index = self.cursor;
            let before = self.text.chars().take(current_index);
            let after = self.text.chars().skip(current_index + 1);
            self.text = before.chain(after).collect();
        }
    }

    pub fn delete_previous_word(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let chars: Vec<char> = self.text.chars().collect();
        let mut pos = self.cursor;
        while pos > 0 && chars[pos - 1].is_whitespace() {
            pos -= 1;
        }
        while pos > 0 && !chars[pos - 1].is_whitespace() {
            pos -= 1;
        }
        let before: String = chars[..pos].iter().collect();
        let after: String = chars[self.cursor..].iter().collect();
        self.text = format!("{}{}", before, after);
        self.cursor = pos;
    }

    pub fn delete_next_word(&mut self) {
        let chars: Vec<char> = self.text.chars().collect();
        let mut pos = self.cursor;
        if pos >= chars.len() {
            return;
        }
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        while pos < chars.len() && !chars[pos].is_whitespace() {
            pos += 1;
        }
        let before: String = chars[..self.cursor].iter().collect();
        let after: String = chars[pos..].iter().collect();
        self.text = format!("{}{}", before, after);
    }

    pub fn move_cursor_to_previous_word(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let chars: Vec<char> = self.text.chars().collect();
        let mut new_cursor = self.cursor;
        while new_cursor > 0 && chars[new_cursor - 1].is_whitespace() {
            new_cursor -= 1;
        }
        while new_cursor > 0 && !chars[new_cursor - 1].is_whitespace() {
            new_cursor -= 1;
        }
        self.cursor = new_cursor;
    }

    pub fn move_cursor_to_next_word(&mut self) {
        let chars: Vec<char> = self.text.chars().collect();
        if self.cursor >= chars.len() {
            return;
        }
        let mut new_cursor = self.cursor;
        while new_cursor < chars.len() && !chars[new_cursor].is_whitespace() {
            new_cursor += 1;
        }
        while new_cursor < chars.len() && chars[new_cursor].is_whitespace() {
            new_cursor += 1;
        }
        self.cursor = new_cursor;
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
    pub fn move_cursor_right(&mut self) {
        let max_pos = self.text.chars().count();
        self.cursor = (self.cursor + 1).min(max_pos);
    }
    pub fn set_cursor(&mut self, position: usize) {
        let max_pos = self.text.chars().count();
        self.cursor = position.min(max_pos);
    }

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
