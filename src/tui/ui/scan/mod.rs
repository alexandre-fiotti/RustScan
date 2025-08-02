//! Scan Components Module
//!
//! This module contains all components related to scan configuration and results.
//! Each component handles a specific aspect of the scanning functionality.

use crate::tui::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

pub mod options;
pub mod ports;
pub mod results;
pub mod targets;

use options::OptionsComponent;
use ports::PortsComponent;
use results::ResultsComponent;
use targets::TargetsComponent;

/// Coordinator for all scan-related components
pub struct ScanComponents {
    targets: TargetsComponent,
    ports: PortsComponent,
    options: OptionsComponent,
    results: ResultsComponent,
}

impl ScanComponents {
    /// Create a new scan components coordinator
    pub fn new() -> Self {
        Self {
            targets: TargetsComponent::new(),
            ports: PortsComponent::new(),
            options: OptionsComponent::new(),
            results: ResultsComponent::new(),
        }
    }

    /// Render the scan configuration section with targets, ports, and options
    pub fn render_config_section(&self, f: &mut Frame, area: Rect, state: &AppState) {
        // Create section block
        let config_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Scan Configuration",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ))
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Reset));

        f.render_widget(config_block.clone(), area);

        let inner_area = config_block.inner(area);

        // Create layout for config items: targets (4), ports (3), options (4)
        let config_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Targets
                Constraint::Length(3), // Ports (one line smaller)
                Constraint::Length(3), // Options
            ])
            .split(inner_area)
            .to_vec();

        // Render targets input field
        self.targets.render(f, config_chunks[0], state);

        // Render ports input field
        self.ports.render(f, config_chunks[1], state);

        // Render options section
        self.options.render(f, config_chunks[2], state);
    }

    /// Render the scan results section
    pub fn render_results_section(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.results.render(f, area, state);
    }
}
