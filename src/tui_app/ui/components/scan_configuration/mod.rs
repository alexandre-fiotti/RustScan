//! Scan Configuration Module
//!
//! This module contains all components and layout logic for the scan configuration section.
//! It handles targets, ports, and options input with proper layout management.

use crate::tui_app::AppState;
use ratatui::{layout::Rect, Frame};

pub mod layout;
pub mod options;
pub mod ports;
pub mod targets;

use layout::ScanConfigLayout;
use options::OptionsComponent;
use ports::PortsComponent;
use targets::TargetsComponent;

/// Coordinator for scan configuration components
#[derive(Default)]
pub struct ScanConfigurationComponents {
    targets: TargetsComponent,
    ports: PortsComponent,
    options: OptionsComponent,
}

impl ScanConfigurationComponents {
    /// Render the entire scan configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        // Render the section frame and get the inner area
        let inner_area = ScanConfigLayout::render_section_frame(f, area);

        // Create the internal layout for components
        let chunks = ScanConfigLayout::internal_layout(inner_area);

        // Render individual components
        self.targets.render(f, chunks[0], state);
        self.ports.render(f, chunks[1], state);
        self.options.render(f, chunks[2], state);
    }
}
