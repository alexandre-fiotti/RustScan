//! Options Component
//!
//! This component handles scan configuration options (timeout, batch size, etc.).

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppState, SelectedField};

/// Component for managing scan options
pub struct OptionsComponent;

impl OptionsComponent {
    /// Create a new options component
    pub fn new() -> Self {
        Self
    }

    /// Render the options configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Options);

        let options_text = format!(
            "Timeout: {}ms | Batch Size: {} | Press [Enter] to start scan | [Tab] to navigate | [Q] to quit",
            config.timeout, config.batch_size
        );

        let border_style = if is_selected {
            Style::default().fg(crate::tui::ui::layout::Layout::banner_green())
        } else {
            Style::default().fg(Color::White)
        };

        let widget = Paragraph::new(options_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Options")
                    .border_style(border_style),
            );

        f.render_widget(widget, area);
    }
}

impl Default for OptionsComponent {
    fn default() -> Self {
        Self::new()
    }
}
