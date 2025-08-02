//! Footer Component
//!
//! This component handles displaying footer information with links.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::tui_app::ui::theme::{link_style, text};

/// Component for displaying footer links
#[derive(Default)]
pub struct FooterComponent;

impl FooterComponent {
    /// Render the footer with GitHub and Discord links
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left side for GitHub
                Constraint::Percentage(50), // Right side for Discord
            ])
            .split(area);

        let github_text = Paragraph::new(text::GITHUB_LINK).style(link_style());
        f.render_widget(github_text, footer_chunks[0]);

        let discord_text = Paragraph::new(text::DISCORD_LINK)
            .style(link_style())
            .alignment(Alignment::Right);
        f.render_widget(discord_text, footer_chunks[1]);
    }
}
