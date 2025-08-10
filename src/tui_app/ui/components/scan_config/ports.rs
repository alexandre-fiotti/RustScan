//! Ports Component
//!
//! This component handles displaying and managing port configuration.

use ratatui::{layout::Rect, Frame};

use crate::tui_app::model::Model;
use crate::tui_app::scan_config::SelectedField;
use crate::tui_app::ui::theme::text;
use crate::tui_app::ui::widgets::text_input::TextInputWidget;

/// Component for managing port selection
#[derive(Default)]
pub struct PortsComponent;

impl PortsComponent {
    /// Render the ports configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &Model) {
        let config = state.scan_config();
        let is_selected = matches!(state.scan_config().selected_field, SelectedField::Ports);

        let confirmed = config.ports.clone();
        TextInputWidget::from_model(
            text::PORTS_TITLE,
            &config.ports_input,
            confirmed,
            is_selected,
            text::PORTS_PLACEHOLDER,
        )
        .render(f, area);
    }
}
