//! Scan Configuration Layout
//!
//! This module handles the layout structure for the scan configuration section.

use ratatui::{
    layout::{Constraint, Direction, Layout as RatatuiLayout, Rect},
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::tui_app::ui::theme::{layout, text, title_style};

/// Layout utilities for the scan configuration section
pub struct ScanConfigLayout;

impl ScanConfigLayout {
    /// Create the scan configuration section with proper border and inner layout
    pub fn render_section_frame(f: &mut Frame, area: Rect) -> Rect {
        let config_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(text::SCAN_CONFIG_TITLE, title_style()));

        f.render_widget(config_block.clone(), area);
        config_block.inner(area)
    }

    /// Create the internal layout for targets, ports, and options
    pub fn internal_layout(area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .margin(layout::STANDARD_MARGIN)
            .constraints([
                Constraint::Length(layout::INPUT_COMPONENT_HEIGHT), // Targets
                Constraint::Length(layout::INPUT_COMPONENT_HEIGHT), // Ports
                Constraint::Length(layout::INPUT_COMPONENT_HEIGHT), // Options
            ])
            .split(area)
            .to_vec()
    }
}
