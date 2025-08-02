//! UI Module
//!
//! This module handles all UI rendering using a component-based architecture.
//! Each component is responsible for rendering a specific part of the interface.

use crate::tui::state::AppState;
use ratatui::Frame;

pub mod components;
pub mod layout;
pub mod theme;

use components::UIComponents;

/// Main UI coordinator
#[derive(Default)]
pub struct UI {
    components: UIComponents,
}

impl UI {
    /// Render the entire UI
    pub fn render(&self, f: &mut Frame, state: &AppState) {
        // Create main layout: banner + content + footer
        let main_chunks = layout::Layout::main_layout(f.area());

        // Render header banner
        self.components.render_header(f, main_chunks[0]);

        // Create two-section layout: config (top) + results (bottom)
        let content_chunks = layout::Layout::two_section_layout(main_chunks[1]);

        // Render scan configuration section
        self.components
            .render_scan_config(f, content_chunks[0], state);

        // Render scan results section
        self.components.render_results(f, content_chunks[1], state);

        // Render footer
        self.components.render_footer(f, main_chunks[2]);
    }
}
