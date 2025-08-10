//! Thread-safe output buffer used for displaying output in the TUI

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct OutputBuffer {
    lines: Arc<Mutex<Vec<String>>>,
    scroll_position: Arc<Mutex<usize>>, // 0 = bottom
    max_lines: usize,
}

impl OutputBuffer {
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }
    pub fn with_capacity(max_lines: usize) -> Self {
        Self {
            lines: Arc::new(Mutex::new(Vec::new())),
            scroll_position: Arc::new(Mutex::new(0)),
            max_lines,
        }
    }

    pub fn push_line(&self, line: String) {
        if let Ok(mut lines) = self.lines.lock() {
            lines.push(line);
            let len = lines.len();
            if len > self.max_lines {
                lines.drain(0..len - self.max_lines);
            }
        } else {
            return;
        }

        if let Ok(mut scroll_pos) = self.scroll_position.lock() {
            if *scroll_pos != 0 {
                *scroll_pos = (*scroll_pos).saturating_sub(1);
            }
        }
    }

    pub fn get_visible_lines(&self, area_height: usize) -> Vec<String> {
        let lines = if let Ok(lines) = self.lines.lock() {
            lines
        } else {
            return vec!["[No output yet]".to_string()];
        };

        let scroll_pos = if let Ok(pos) = self.scroll_position.lock() {
            *pos
        } else {
            0
        };

        let total = lines.len();
        if total == 0 {
            return vec!["[No output yet]".to_string()];
        }
        let visible = area_height.saturating_sub(2);
        if visible == 0 {
            return vec![];
        }
        let start = if scroll_pos == 0 {
            total.saturating_sub(visible)
        } else {
            total.saturating_sub(visible + scroll_pos)
        };
        let end = (start + visible).min(total);
        lines[start..end].to_vec()
    }

    pub fn scroll_up(&self, lines: usize) {
        let total = if let Ok(guard) = self.lines.lock() {
            guard.len()
        } else {
            return;
        };
        if let Ok(mut pos) = self.scroll_position.lock() {
            *pos = (*pos + lines).min(total.saturating_sub(1));
        }
    }
    pub fn scroll_down(&self, lines: usize) {
        if let Ok(mut pos) = self.scroll_position.lock() {
            *pos = (*pos).saturating_sub(lines);
        }
    }
    pub fn scroll_to_bottom(&self) {
        if let Ok(mut pos) = self.scroll_position.lock() {
            *pos = 0;
        }
    }
    pub fn scroll_to_top(&self) {
        let total = if let Ok(guard) = self.lines.lock() {
            guard.len()
        } else {
            return;
        };
        if let Ok(mut pos) = self.scroll_position.lock() {
            *pos = total.saturating_sub(1);
        }
    }

    pub fn scroll_info(&self, area_height: usize) -> ScrollInfo {
        let (total, pos) = match (self.lines.lock(), self.scroll_position.lock()) {
            (Ok(lines), Ok(pos_guard)) => (lines.len(), *pos_guard),
            _ => (0, 0),
        };
        let visible = area_height.saturating_sub(2);
        ScrollInfo {
            total_lines: total,
            scroll_position: pos,
            at_bottom: pos == 0,
            at_top: pos >= total.saturating_sub(visible),
        }
    }

    pub fn clear(&self) {
        if let Ok(mut lines) = self.lines.lock() {
            lines.clear();
        }
        if let Ok(mut pos) = self.scroll_position.lock() {
            *pos = 0;
        }
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ScrollInfo {
    pub total_lines: usize,
    pub scroll_position: usize,
    pub at_bottom: bool,
    pub at_top: bool,
}
