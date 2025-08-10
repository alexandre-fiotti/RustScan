#[derive(Debug, Clone)]
pub struct ResultsModel {
    pub lines: Vec<String>,
    pub scroll_position: usize, // 0 = bottom
    pub max_lines: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollInfo {
    pub total_lines: usize,
    pub scroll_position: usize,
    pub at_bottom: bool,
    pub at_top: bool,
}

impl Default for ResultsModel {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            scroll_position: 0,
            max_lines: 10_000,
        }
    }
}

impl ResultsModel {
    pub fn push_line(&mut self, line: String) {
        // Ensure each line is a separate entry and always start on new line
        for (i, l) in line.split('\n').enumerate() {
            if i == 0 {
                self.lines.push(l.to_string());
            } else {
                // Ensure subsequent segments start on a new line cleanly
                self.lines.push(l.to_string());
            }
        }
        let len = self.lines.len();
        if len > self.max_lines {
            let excess = len - self.max_lines;
            self.lines.drain(0..excess);
        }
        if self.scroll_position != 0 {
            self.scroll_position = self.scroll_position.saturating_sub(1);
        }
    }

    pub fn push_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            self.push_line(line);
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.scroll_position = 0;
    }

    pub fn get_visible_lines(&self, area_height: usize) -> Vec<String> {
        if self.lines.is_empty() {
            return vec!["[No output yet]".to_string()];
        }
        let visible = area_height.saturating_sub(2);
        if visible == 0 {
            return vec![];
        }
        let total = self.lines.len();
        let start = if self.scroll_position == 0 {
            total.saturating_sub(visible)
        } else {
            total.saturating_sub(visible + self.scroll_position)
        };
        let end = (start + visible).min(total);
        self.lines[start..end].to_vec()
    }

    pub fn scroll_up(&mut self, lines: usize) {
        let total = self.lines.len();
        self.scroll_position = (self.scroll_position + lines).min(total.saturating_sub(1));
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_position = self.scroll_position.saturating_sub(lines);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_position = 0;
    }

    pub fn scroll_to_top(&mut self) {
        let total = self.lines.len();
        self.scroll_position = total.saturating_sub(1);
    }

    pub fn scroll_info(&self, area_height: usize) -> ScrollInfo {
        let total = self.lines.len();
        let visible = area_height.saturating_sub(2);
        ScrollInfo {
            total_lines: total,
            scroll_position: self.scroll_position,
            at_bottom: self.scroll_position == 0,
            at_top: self.scroll_position >= total.saturating_sub(visible),
        }
    }
}
