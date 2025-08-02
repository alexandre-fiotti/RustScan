//! UI Module
//!
//! This module handles all UI rendering using a component-based architecture.
//! Each component is responsible for rendering a specific part of the interface.

use crate::tui::state::AppState;
use ratatui::Frame;

pub mod components;
pub mod layout;

use components::ScanComponents;
use layout::Layout;

/// Main UI coordinator
pub struct UI {
    layout: Layout,
    scan_components: ScanComponents,
}

impl UI {
    /// Create a new UI instance
    pub fn new() -> Self {
        Self {
            layout: Layout::new(),
            scan_components: ScanComponents::new(),
        }
    }

    /// Render the entire UI
    pub fn render(&self, f: &mut Frame, state: &AppState) {
        // Create main layout: banner + content
        let main_chunks = self.layout.main_layout(f.area());

        // Render banner
        self.layout.render_banner(f, main_chunks[0]);

        // Create two-section layout: config (top) + results (bottom)
        let content_chunks = self.layout.two_section_layout(main_chunks[1]);

        // Render scan configuration section
        self.scan_components
            .render_config_section(f, content_chunks[0], state);

        // Render scan results section
        self.scan_components
            .render_results_section(f, content_chunks[1], state);
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}
