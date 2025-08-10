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

use crate::tui_app::model::Model;
use crate::tui_app::scan_config::SelectedField;
use crate::tui_app::ui::theme::{
    active_style, normal_text_style, text, title_selected_style, title_unselected_style,
    BORDER_NORMAL,
};

/// Component for managing scan options
#[derive(Default)]
pub struct OptionsComponent;

impl OptionsComponent {
    /// Render the options configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &Model) {
        let config = state.scan_config();
        let is_selected = matches!(state.scan_config().selected_field, SelectedField::Options);

        let options_text = format!(
            "Timeout: {}ms | Batch Size: {} | {}",
            config.timeout,
            config.batch_size,
            text::NAVIGATION_HELP
        );

        // Choose border and title styles based on selection state only
        let (border_style, title_style) = if is_selected {
            (active_style(), title_selected_style())
        } else {
            (Style::default().fg(BORDER_NORMAL), title_unselected_style())
        };

        let widget = Paragraph::new(options_text)
            .style(normal_text_style())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(text::OPTIONS_TITLE, title_style))
                    .border_style(border_style),
            );

        f.render_widget(widget, area);
    }
}
