//! Scan Configuration Module
//!
//! This module contains all components and layout logic for the scan configuration section.
//! It handles targets, ports, and options input with proper layout management.

use crate::tui_app::model::Model;
use ratatui::{layout::Rect, Frame};

pub mod layout;
pub mod options;
pub mod ports;
pub mod scan_button;
pub mod targets;

use layout::ScanConfigLayout;
use options::OptionsComponent;
use ports::PortsComponent;
use scan_button::ScanButtonComponent;
use targets::TargetsComponent;

/// Coordinator for scan configuration components
#[derive(Default)]
pub struct ScanConfigurationComponents {
    targets: TargetsComponent,
    ports: PortsComponent,
    options: OptionsComponent,
    scan_button: ScanButtonComponent,
}

impl ScanConfigurationComponents {
    /// Render the entire scan configuration section
    pub fn render(&self, f: &mut Frame, area: Rect, state: &Model) {
        // Render the section frame and get the inner area
        let inner_area = ScanConfigLayout::render_section_frame(f, area);

        // Create the internal layout for components
        let chunks = ScanConfigLayout::internal_layout(inner_area);

        // Render individual components
        self.targets.render(f, chunks[0], state);
        self.ports.render(f, chunks[1], state);
        self.options.render(f, chunks[2], state);

        // Render button in the bottom action area
        let action_chunks = ScanConfigLayout::bottom_action_area(chunks[3]);
        self.scan_button
            .render(f, action_chunks[1], &state.scan_config().scan_button_mode);
    }
}
