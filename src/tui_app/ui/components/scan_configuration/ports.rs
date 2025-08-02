//! Ports Component
//!
//! This component handles displaying and managing port configuration.

use ratatui::{
    layout::{Position, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui_app::ui::theme::{
    active_style, border_normal, normal_text_style, placeholder_style, text, title_style,
};
use crate::tui_app::{AppState, SelectedField};

/// Component for managing port selection
#[derive(Default)]
pub struct PortsComponent;

impl PortsComponent {
    /// Render the ports configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Ports);

        // Show input buffer if editing, otherwise show confirmed ports or placeholder
        let display_text = if !config.ports_input.is_empty() {
            config.ports_input.text().to_string()
        } else if let Some(ports) = &config.ports {
            ports.clone()
        } else {
            text::PORTS_PLACEHOLDER.to_string()
        };

        let style = if !config.ports_input.is_empty() || config.ports.is_some() {
            normal_text_style()
        } else {
            placeholder_style()
        };

        let border_style = if is_selected {
            active_style()
        } else {
            Style::default().fg(border_normal())
        };

        let widget = Paragraph::new(display_text).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(text::PORTS_TITLE, title_style()))
                .border_style(border_style)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

        f.render_widget(widget, area);

        // Set cursor position if this field is selected and we're editing
        if is_selected && !config.ports_input.is_empty() {
            f.set_cursor_position(Position::new(
                area.x + config.ports_input.cursor() as u16 + 2,
                area.y + 1,
            ));
        }
    }
}
