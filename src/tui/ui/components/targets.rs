//! Targets Component
//!
//! This component handles target selection (IP addresses, hostnames, CIDR ranges).

use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppState, SelectedField};

/// Component for managing scan targets
pub struct TargetsComponent;

impl TargetsComponent {
    /// Create a new targets component
    pub fn new() -> Self {
        Self
    }

    /// Render the targets configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config = state.scan_config();
        let is_selected = matches!(state.selected_field(), SelectedField::Targets);

        // Show input buffer if editing, otherwise show confirmed targets
        let display_text = if !config.targets_input.is_empty() {
            config.targets_input.clone()
        } else if !config.targets.is_empty() {
            config.targets.join(", ")
        } else {
            "Enter targets (e.g., 192.168.1.1, example.com, 10.0.0.0/24)".to_string()
        };

        let style = if !config.targets_input.is_empty() || !config.targets.is_empty() {
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
                .title("Targets")
                .border_style(border_style)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );

        f.render_widget(widget, area);

        // Set cursor position if this field is selected and we're editing
        if is_selected && !config.targets_input.is_empty() {
            f.set_cursor_position(Position::new(
                area.x + config.targets_cursor as u16 + 2,
                area.y + 1,
            ));
        }
    }
}

impl Default for TargetsComponent {
    fn default() -> Self {
        Self::new()
    }
}
