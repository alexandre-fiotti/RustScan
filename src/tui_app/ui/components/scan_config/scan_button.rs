//! Scan Button Component

use ratatui::{layout::Rect, Frame};

use crate::tui_app::shared::button_mode::ButtonMode;
use crate::tui_app::ui::widgets::button::ButtonWidget;

/// Component for managing the scan button
#[derive(Default)]
pub struct ScanButtonComponent;

impl ScanButtonComponent {
    /// Render the scan button
    pub fn render(&self, f: &mut Frame, area: Rect, mode: &ButtonMode) {
        let button = ButtonWidget::new("Scan").mode(mode);
        f.render_widget(button, area);
    }
}
