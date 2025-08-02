//! Components Module
//!
//! This module contains all UI components related to scan configuration and results.

use crate::tui::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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

    /// Render the scan configuration section
    pub fn render_config_section(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let config_block = Block::default().borders(Borders::ALL).title(Span::styled(
            "Scan Configuration",
            Style::default()
                .fg(crate::tui::ui::layout::Layout::banner_green())
                .add_modifier(Modifier::BOLD),
        ));

        f.render_widget(config_block.clone(), area);

        let inner_area = config_block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Targets
                Constraint::Length(3), // Ports
                Constraint::Length(3), // Options
            ])
            .split(inner_area)
            .to_vec();

        self.targets.render(f, chunks[0], state);
        self.ports.render(f, chunks[1], state);
        self.options.render(f, chunks[2], state);
    }

    /// Render the scan results section
    pub fn render_results_section(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.results.render(f, area, state);
    }
}

impl Default for ScanComponents {
    fn default() -> Self {
        Self::new()
    }
}
