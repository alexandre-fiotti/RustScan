//! Ports Component
//!
//! This component handles port configuration (ranges, specific ports, common ports).

use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppState, SelectedField};

/// Component for managing port configuration
pub struct PortsComponent;

impl PortsComponent {
    /// Create a new ports component
    pub fn new() -> Self {
        Self
    }

    /// Render the ports configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Ports);

        // Show input buffer if editing, otherwise show confirmed ports or placeholder
        let display_text = if !config.ports_input.is_empty() {
            config.ports_input.clone()
        } else if let Some(ports) = &config.ports {
            ports.clone()
        } else {
            "All ports (1-65535) - Enter custom ports (e.g., 80,443,22 or 1-1000)".to_string()
        };

        let style = if !config.ports_input.is_empty() || config.ports.is_some() {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };

        let border_style = if is_selected {
            Style::default().fg(crate::tui::ui::layout::Layout::banner_green())
        } else {
            Style::default().fg(Color::White)
        };

        let widget = Paragraph::new(display_text).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ports")
                .border_style(border_style)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

        f.render_widget(widget, area);

        // Set cursor position if this field is selected and we're editing
        if is_selected && !config.ports_input.is_empty() {
            f.set_cursor_position(Position::new(
                area.x + config.ports_cursor as u16 + 2,
                area.y + 1,
            ));
        }
    }
}

impl Default for PortsComponent {
    fn default() -> Self {
        Self::new()
    }
}
