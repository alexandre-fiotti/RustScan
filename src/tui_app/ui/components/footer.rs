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
    /// Render the footer with GitHub and Discord links and version
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33), // Left side for GitHub
                Constraint::Percentage(34), // Center for version
                Constraint::Percentage(33), // Right side for Discord
            ])
            .split(area);

        let github_text = Paragraph::new(text::GITHUB_LINK).style(link_style());
        f.render_widget(github_text, footer_chunks[0]);

        let version_text = Paragraph::new(format!("v{}", env!("CARGO_PKG_VERSION")))
            .style(link_style())
            .alignment(Alignment::Center);
        f.render_widget(version_text, footer_chunks[1]);

        let discord_text = Paragraph::new(text::DISCORD_LINK)
            .style(link_style())
            .alignment(Alignment::Right);
        f.render_widget(discord_text, footer_chunks[2]);
    }
}
