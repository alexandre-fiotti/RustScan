//! Layout Component
//!
//! This component handles the overall layout structure of the TUI.

use ratatui::{
    layout::{Constraint, Direction, Layout as RatatuiLayout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Layout manager for the TUI
pub struct Layout;

impl Layout {
    /// Create a new layout manager
    pub fn new() -> Self {
        Self
    }

    /// Create the main layout with banner and content areas
    pub fn main_layout(&self, area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Banner
                Constraint::Min(0),    // Content
            ])
            .split(area)
            .to_vec()
    }

    /// Create a two-section layout (config top, results bottom)
    pub fn two_section_layout(&self, area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(14), // Config section
                Constraint::Min(0),     // Results section
            ])
            .split(area)
            .to_vec()
    }

    /// Render the RustScan banner with gradient colors
    pub fn render_banner(&self, f: &mut Frame, area: Rect) {
        let width = area.width as usize;

        let ascii_lines = [
            ".----. .-. .-. .----..---.  .----. .---.   .--.  .-. .-.",
            "| {}  }| { } |{ {__ {_   _}{ {__  /  ___} / {} \\ |  `| |",
            "| .-. \\| {_} |.-._} } | |  .-._} }\\     }/  /\\  \\| |\\  |",
            "`-' `-'`-----'`----'  `-'  `----'  `---' `-'  `-'`-' `-'",
        ];

        let mut banner_lines = Vec::new();

        for ascii_line in ascii_lines.iter() {
            let padding = if width > ascii_line.len() {
                (width - ascii_line.len()) / 2
            } else {
                0
            };

            let display_line = if width < ascii_line.len() {
                &ascii_line[..width.min(ascii_line.len())]
            } else {
                ascii_line
            };

            let mut spans = vec![Span::styled(" ".repeat(padding), Style::default())];

            for (char_index, ch) in display_line.chars().enumerate() {
                let progress = char_index as f32 / display_line.len().max(1) as f32;
                let color = self.interpolate_green_gradient(progress);

                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            }

            banner_lines.push(Line::from(spans));
        }

        // Add subtitle with gradient
        let subtitle = "The Modern Day Port Scanner";
        let subtitle_padding = if width > subtitle.len() {
            (width - subtitle.len()) / 2
        } else {
            0
        };

        let mut subtitle_spans = vec![Span::styled(" ".repeat(subtitle_padding), Style::default())];

        for (char_index, ch) in subtitle.chars().enumerate() {
            let progress = char_index as f32 / subtitle.len().max(1) as f32;
            let color = self.interpolate_green_gradient(progress);

            subtitle_spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(color).add_modifier(Modifier::ITALIC),
            ));
        }

        banner_lines.push(Line::from(subtitle_spans));

        let banner = Paragraph::new(banner_lines);
        f.render_widget(banner, area);
    }

    /// Create a green to electric blue gradient
    fn interpolate_green_gradient(&self, progress: f32) -> Color {
        // Bright Green (0, 255, 0) to Electric Blue (0, 150, 255)
        let red_component = 0;
        let green_component = ((1.0 - progress) * 255.0 + progress * 150.0) as u8;
        let blue_component = (progress * 255.0) as u8;
        Color::Rgb(red_component, green_component, blue_component)
    }

    /// Get the banner green color for UI consistency
    pub fn banner_green() -> Color {
        Color::Rgb(0, 255, 0)
    }

    /// Get the banner blue color for UI consistency
    pub fn banner_blue() -> Color {
        Color::Rgb(0, 150, 255)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}
