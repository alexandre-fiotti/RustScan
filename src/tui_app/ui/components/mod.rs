//! Components Module
//!
//! This module contains all UI components for the TUI interface.

use crate::tui_app::AppState;
use ratatui::{layout::Rect, Frame};

pub mod footer;
pub mod header;
pub mod scan_configuration;
pub mod scan_results;

use footer::FooterComponent;
use header::HeaderComponent;
use scan_configuration::ScanConfigurationComponents;
use scan_results::ResultsComponent;

/// Main UI components coordinator
#[derive(Default)]
pub struct UIComponents {
    header: HeaderComponent,
    scan_config: ScanConfigurationComponents,
    results: ResultsComponent,
    footer: FooterComponent,
}

impl UIComponents {
    /// Render the header section
    pub fn render_header(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.header.render(f, area, state);
    }

    /// Render the scan configuration section
    pub fn render_scan_config(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.scan_config.render(f, area, state);
    }

    /// Render the scan results section
    pub fn render_results(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.results.render(f, area, state);
    }

    /// Render the footer section
    pub fn render_footer(&self, f: &mut Frame, area: Rect) {
        self.footer.render(f, area);
    }
}
