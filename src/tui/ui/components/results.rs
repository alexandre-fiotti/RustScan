//! Results Component
//!
//! This component handles displaying scan results and progress.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::AppState;

/// Component for displaying scan results
pub struct ResultsComponent;

impl ResultsComponent {
    /// Create a new results component
    pub fn new() -> Self {
        Self
    }

    /// Render the results display section
    pub fn render(&self, f: &mut Frame, area: Rect, _state: &AppState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Results content
                Constraint::Length(1), // Footer for links
            ])
            .split(area);

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
            .style(Style::default().fg(Color::White))
            .block(
                Block::default().borders(Borders::ALL).title(Span::styled(
                    "Scan Results",
                    Style::default()
                        .fg(crate::tui::ui::layout::Layout::banner_green())
                        .add_modifier(Modifier::BOLD),
                )),
            );

        f.render_widget(results_widget, chunks[0]);

        // Footer with links
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left side for GitHub
                Constraint::Percentage(50), // Right side for Discord
            ])
            .split(chunks[1]);

        let github_text = Paragraph::new("GitHub: https://github.com/RustScan/RustScan")
            .style(Style::default().fg(crate::tui::ui::layout::Layout::banner_blue()));
        f.render_widget(github_text, footer_chunks[0]);

        let discord_text = Paragraph::new("Discord: http://discord.skerritt.blog")
            .style(Style::default().fg(crate::tui::ui::layout::Layout::banner_blue()))
            .alignment(ratatui::layout::Alignment::Right);
        f.render_widget(discord_text, footer_chunks[1]);
    }
}

impl Default for ResultsComponent {
    fn default() -> Self {
        Self::new()
    }
}
