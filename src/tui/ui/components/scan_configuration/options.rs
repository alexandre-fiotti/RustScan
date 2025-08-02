//! Options Component
//!
//! This component handles displaying and managing scan options.

use ratatui::{
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppState, SelectedField};
use crate::tui::ui::theme::{active_style, border_normal, normal_text_style, text, title_style};

/// Component for managing scan options
#[derive(Default)]
pub struct OptionsComponent;

impl OptionsComponent {
    /// Render the options configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Options);

        let options_text = format!(
            "Timeout: {}ms | Batch Size: {} | {}",
            config.timeout,
            config.batch_size,
            text::NAVIGATION_HELP
        );

        let border_style = if is_selected {
            active_style()
        } else {
            Style::default().fg(border_normal())
        };

        let widget = Paragraph::new(options_text)
            .style(normal_text_style())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(text::OPTIONS_TITLE, title_style()))
                    .border_style(border_style),
            );

        f.render_widget(widget, area);
    }
}
