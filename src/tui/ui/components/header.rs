//! Header Component
//!
//! This component handles displaying the RustScan banner with gradient colors.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::ui::theme::text;

/// Component for displaying the header banner
#[derive(Default)]
pub struct HeaderComponent;

impl HeaderComponent {
    /// Render the RustScan banner with gradient colors
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let width = area.width as usize;

        let banner_lines: Vec<Line> = text::ASCII_LINES
            .iter()
            .map(|line| Self::create_gradient_line(line, width))
            .chain(std::iter::once(Self::create_gradient_line(
                text::BANNER_SUBTITLE,
                width,
            )))
            .collect();

        let banner = Paragraph::new(banner_lines);
        f.render_widget(banner, area);
    }

    /// Create a line with gradient coloring and proper centering
    fn create_gradient_line(text: &str, width: usize) -> Line {
        let padding = Self::calculate_center_padding(text.len(), width);
        let display_text = Self::truncate_if_needed(text, width);

        let mut spans = Vec::with_capacity(padding + display_text.len());

        // Add padding
        if padding > 0 {
            spans.push(Span::styled(" ".repeat(padding), Style::default()));
        }

        // Add gradient-colored characters
        spans.extend(display_text.chars().enumerate().map(|(index, ch)| {
            let progress = index as f32 / display_text.len().max(1) as f32;
            let color = Self::interpolate_gradient(progress);
            let modifier = if text == text::BANNER_SUBTITLE {
                Modifier::ITALIC
            } else {
                Modifier::BOLD
            };

            Span::styled(
                ch.to_string(),
                Style::default().fg(color).add_modifier(modifier),
            )
        }));

        Line::from(spans)
    }

    /// Calculate padding needed to center text
    fn calculate_center_padding(text_len: usize, width: usize) -> usize {
        if width > text_len {
            (width - text_len) / 2
        } else {
            0
        }
    }

    /// Truncate text if it's longer than available width
    fn truncate_if_needed(text: &str, width: usize) -> &str {
        if width < text.len() {
            &text[..width.min(text.len())]
        } else {
            text
        }
    }

    /// Create a green to electric blue gradient color
    fn interpolate_gradient(progress: f32) -> Color {
        // Interpolate from Bright Green to Electric Blue using theme colors
        let green_rgb = (0, 255, 0); // theme::primary_green() RGB values
        let blue_rgb = (0, 150, 255); // theme::primary_blue() RGB values

        let green_component =
            ((1.0 - progress) * green_rgb.1 as f32 + progress * blue_rgb.1 as f32) as u8;
        let blue_component = (progress * blue_rgb.2 as f32) as u8;
        Color::Rgb(0, green_component, blue_component)
    }
}
