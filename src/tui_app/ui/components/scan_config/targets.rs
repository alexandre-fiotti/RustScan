//! Targets Component
//!
//! This component handles displaying and managing target configuration.

use ratatui::{layout::Rect, Frame};

use crate::tui_app::model::Model;
use crate::tui_app::scan_config::SelectedField;
use crate::tui_app::ui::theme::text;
use crate::tui_app::ui::widgets::text_input::TextInputWidget;

/// Component for managing scan targets
#[derive(Default)]
pub struct TargetsComponent;

impl TargetsComponent {
    /// Render the targets configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &Model) {
        let config = state.scan_config();
        let is_selected = matches!(state.scan_config().selected_field, SelectedField::Targets);

        let confirmed = (!config.targets.is_empty()).then(|| config.targets.join(", "));
        TextInputWidget::from_model(
            text::TARGETS_TITLE,
            &config.targets_input,
            confirmed,
            is_selected,
            text::TARGETS_PLACEHOLDER,
        )
        .render(f, area);
    }
}
