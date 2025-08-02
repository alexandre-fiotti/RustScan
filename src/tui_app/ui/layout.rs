//! Layout Component
//!
//! This component handles the overall layout structure of the TUI.

use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

use super::theme::layout;

/// Layout utilities for the TUI
pub struct Layout;

impl Layout {
    /// Create the main layout with banner, content, and footer areas
    pub fn main_layout(area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(layout::HEADER_HEIGHT), // Banner
                Constraint::Min(0),                        // Content
                Constraint::Length(layout::FOOTER_HEIGHT), // Footer
            ])
            .split(area)
            .to_vec()
    }

    /// Create a two-section layout (config top, results bottom)
    pub fn two_section_layout(area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(layout::SCAN_CONFIG_HEIGHT), // Config section
                Constraint::Min(0),                             // Results section
            ])
            .split(area)
            .to_vec()
    }
}
