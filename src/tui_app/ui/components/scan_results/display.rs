//! Results Component
//!
//! This component handles displaying scan results and terminal output in real-time.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui_app::model::Model;
use crate::tui_app::ui::theme::{section_title_style, text};

/// Component for displaying scan results and terminal output
#[derive(Default)]
pub struct ResultsComponent;

impl ResultsComponent {
    /// Render the results display section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &Model) {
        // Get visible output lines from the buffer
        let output_lines = state
            .output_buffer()
            .get_visible_lines(area.height as usize);
        let scroll_info = state.output_buffer().scroll_info(area.height as usize);

        // Convert strings to ratatui Lines
        let text_lines: Vec<Line> = output_lines.into_iter().map(Line::from).collect();

        // Create title with scroll indicator
        let title = if scroll_info.total_lines > 0 {
            if scroll_info.at_bottom {
                format!(
                    "{} (Live - {} lines)",
                    text::SCAN_RESULTS_TITLE,
                    scroll_info.total_lines
                )
            } else {
                format!(
                    "{} ({}/{} lines - ↑↓ to scroll)",
                    text::SCAN_RESULTS_TITLE,
                    scroll_info.total_lines - scroll_info.scroll_position,
                    scroll_info.total_lines
                )
            }
        } else {
            text::SCAN_RESULTS_TITLE.to_string()
        };

        let results_widget = Paragraph::new(text_lines)
            .style(Style::default())
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(title, section_title_style())),
            );

        f.render_widget(results_widget, area);
    }
}
