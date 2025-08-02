//! Results Component
//!
//! This component handles displaying scan results and progress.

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::AppState;
use crate::tui::ui::theme::{normal_text_style, text, title_style};

/// Component for displaying scan results
#[derive(Default)]
pub struct ResultsComponent;

impl ResultsComponent {
    /// Render the results display section
    pub fn render(&self, f: &mut Frame, area: Rect, _state: &AppState) {
        // Main results content
        let results_text = vec![
            Line::from(""),
            Line::from(
                "No scan results yet. Configure targets above and press [Enter] to start scanning.",
            ),
            Line::from(""),
            Line::from("Scan results will appear here in real-time:"),
            Line::from("• Open ports and services"),
            Line::from("• Response times"),
            Line::from("• Progress information"),
            Line::from(""),
        ];

        let results_widget = Paragraph::new(results_text)
            .style(normal_text_style())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(text::SCAN_RESULTS_TITLE, title_style())),
            );

        f.render_widget(results_widget, area);
    }
}
