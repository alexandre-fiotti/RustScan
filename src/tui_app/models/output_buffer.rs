//! Output Buffer Module
//!
//! This module provides a thread-safe buffer for capturing and storing
//! all terminal output (stdout, stderr, logs, external command output).

use std::sync::{Arc, Mutex};

/// Thread-safe buffer for storing terminal output lines
#[derive(Debug, Clone)]
pub struct OutputBuffer {
    /// Lines of output, stored as strings
    lines: Arc<Mutex<Vec<String>>>,
    /// Current scroll position (0 = bottom, higher = scrolled up)
    scroll_position: Arc<Mutex<usize>>,
    /// Maximum number of lines to keep in memory
    max_lines: usize,
}

impl OutputBuffer {
    /// Create a new output buffer with default capacity
    pub fn new() -> Self {
        Self::with_capacity(10000) // 10k lines should be plenty
    }

    /// Create a new output buffer with specified max capacity
    pub fn with_capacity(max_lines: usize) -> Self {
        Self {
            lines: Arc::new(Mutex::new(Vec::new())),
            scroll_position: Arc::new(Mutex::new(0)),
            max_lines,
        }
    }

    /// Add a line of output to the buffer
    pub fn push_line(&self, line: String) {
        let mut lines = self.lines.lock().unwrap();
        lines.push(line);

        // Trim buffer if it exceeds max size
        let lines_len = lines.len();
        if lines_len > self.max_lines {
            lines.drain(0..lines_len - self.max_lines);
        }

        // Auto-scroll to bottom if we're already at the bottom
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        if *scroll_pos == 0 {
            // Stay at bottom
        } else {
            // Maintain scroll position relative to new content
            *scroll_pos = (*scroll_pos).saturating_sub(1);
        }
    }

    /// Get visible lines for the given area height
    pub fn get_visible_lines(&self, area_height: usize) -> Vec<String> {
        let lines = self.lines.lock().unwrap();
        let scroll_pos = *self.scroll_position.lock().unwrap();

        let total_lines = lines.len();
        if total_lines == 0 {
            return vec!["[No output yet]".to_string()];
        }

        // Calculate which lines to show
        let visible_count = area_height.saturating_sub(2); // Account for borders
        if visible_count == 0 {
            return vec![];
        }

        let start_idx = if scroll_pos == 0 {
            // At bottom - show the last `visible_count` lines
            total_lines.saturating_sub(visible_count)
        } else {
            // Scrolled up - calculate start position
            total_lines.saturating_sub(visible_count + scroll_pos)
        };

        let end_idx = (start_idx + visible_count).min(total_lines);

        lines[start_idx..end_idx].to_vec()
    }

    /// Scroll up by the specified number of lines
    pub fn scroll_up(&self, lines: usize) {
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        let total_lines = self.lines.lock().unwrap().len();

        *scroll_pos = (*scroll_pos + lines).min(total_lines.saturating_sub(1));
    }

    /// Scroll down by the specified number of lines
    pub fn scroll_down(&self, lines: usize) {
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        *scroll_pos = (*scroll_pos).saturating_sub(lines);
    }

    /// Jump to the bottom (most recent output)
    pub fn scroll_to_bottom(&self) {
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        *scroll_pos = 0;
    }

    /// Jump to the top (oldest output)
    pub fn scroll_to_top(&self) {
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        let total_lines = self.lines.lock().unwrap().len();
        *scroll_pos = total_lines.saturating_sub(1);
    }

    /// Get current scroll position info for status display
    pub fn scroll_info(&self, area_height: usize) -> ScrollInfo {
        let lines = self.lines.lock().unwrap();
        let scroll_pos = *self.scroll_position.lock().unwrap();
        let total_lines = lines.len();
        let visible_count = area_height.saturating_sub(2);

        ScrollInfo {
            total_lines,
            scroll_position: scroll_pos,
            at_bottom: scroll_pos == 0,
            at_top: scroll_pos >= total_lines.saturating_sub(visible_count),
        }
    }

    /// Clear all output
    pub fn clear(&self) {
        let mut lines = self.lines.lock().unwrap();
        lines.clear();
        let mut scroll_pos = self.scroll_position.lock().unwrap();
        *scroll_pos = 0;
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about current scroll state
#[derive(Debug)]
pub struct ScrollInfo {
    pub total_lines: usize,
    pub scroll_position: usize,
    pub at_bottom: bool,
    pub at_top: bool,
}
