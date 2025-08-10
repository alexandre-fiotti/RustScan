//! TEA View: Pure mapping of Model -> UI

use ratatui::Frame;

use crate::tui_app::ui::UI;
use crate::tui_app::Model;

/// Render using existing UI component tree
pub fn view(model: &mut Model, frame: &mut Frame) {
    // UI::render accepts &AppState; passing &*model keeps purity in widgets
    let ui = UI::default();
    ui.render(frame, model);
}
