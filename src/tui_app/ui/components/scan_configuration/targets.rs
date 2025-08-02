//! Targets Component
//!
//! This component handles displaying and managing target configuration.

use ratatui::{
    layout::{Position, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui_app::ui::theme::{
    active_style, border_normal, normal_text_style, placeholder_style, text, title_selected_style,
    title_unselected_style,
};
use crate::tui_app::{AppState, SelectedField};

/// Component for managing scan targets
#[derive(Default)]
pub struct TargetsComponent;

impl TargetsComponent {
    /// Render the targets configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Targets);

        // Show input buffer if editing, otherwise show confirmed targets
        let display_text = if !config.targets_input.is_empty() {
            config.targets_input.text().to_string()
        } else if !config.targets.is_empty() {
            config.targets.join(", ")
        } else {
            text::TARGETS_PLACEHOLDER.to_string()
        };

        let style = if !config.targets_input.is_empty() || !config.targets.is_empty() {
            normal_text_style()
        } else {
            placeholder_style()
        };

        // Choose border and title styles based on selection state only
        let (border_style, title_style) = if is_selected {
            (active_style(), title_selected_style())
        } else {
            (
                Style::default().fg(border_normal()),
                title_unselected_style(),
            )
        };

        let widget = Paragraph::new(display_text).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(text::TARGETS_TITLE, title_style))
                .border_style(border_style)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

        f.render_widget(widget, area);

        // Set cursor position if this field is selected and we're editing
        if is_selected && !config.targets_input.is_empty() {
            f.set_cursor_position(Position::new(
                area.x + config.targets_input.cursor() as u16 + 2,
                area.y + 1,
            ));
        }
    }
}
