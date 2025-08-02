//! Theme Module
//!
//! This module defines all colors, styles, and common UI constants used throughout the TUI.
//! It provides a centralized place for visual consistency across components.

use ratatui::style::{Color, Modifier, Style};

// === Core Brand Colors ===

/// Primary green color - used for highlights and active elements
pub const fn primary_green() -> Color {
    Color::Rgb(0, 255, 0)
}

/// Primary blue color - used for accents and links
pub const fn primary_blue() -> Color {
    Color::Rgb(0, 150, 255)
}

// === Text Colors ===

/// Primary text color for normal content
pub const fn text_primary() -> Color {
    Color::White
}

/// Placeholder text color for empty fields
pub const fn text_placeholder() -> Color {
    Color::Gray
}

// === Border Colors ===

/// Active border color for selected elements
pub const fn border_active() -> Color {
    primary_green()
}

/// Normal border color for unselected elements
pub const fn border_normal() -> Color {
    Color::White
}

// === Common Styles ===

/// Style for main section titles (white and bold)
/// Used for major sections like "Scan Configuration" and "Scan Results"
pub fn section_title_style() -> Style {
    Style::default()
        .fg(text_primary())
        .add_modifier(Modifier::BOLD)
}

/// Style for selected component titles (green and bold)
/// Used for individual components like "Targets", "Ports", "Options" when selected
pub fn title_selected_style() -> Style {
    Style::default()
        .fg(primary_green())
        .add_modifier(Modifier::BOLD)
}

/// Style for unselected component titles (white and bold)
/// Used for individual components like "Targets", "Ports", "Options" when not selected
pub fn title_unselected_style() -> Style {
    Style::default()
        .fg(text_primary())
        .add_modifier(Modifier::BOLD)
}

/// Style for component titles when hovered (blue and bold with underline)
/// Provides visual feedback that the component can be clicked
pub fn title_hovered_style() -> Style {
    Style::default()
        .fg(primary_blue())
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

/// Style for component borders when hovered (blue border)
/// Provides visual feedback that the component can be clicked
pub fn border_hovered_style() -> Style {
    Style::default().fg(primary_blue())
}

/// Style for active/selected elements
pub fn active_style() -> Style {
    Style::default().fg(border_active())
}

/// Style for normal text
pub fn normal_text_style() -> Style {
    Style::default().fg(text_primary())
}

/// Style for placeholder text
pub fn placeholder_style() -> Style {
    Style::default().fg(text_placeholder())
}

/// Style for links and clickable elements
pub fn link_style() -> Style {
    Style::default().fg(primary_blue())
}

/// Layout constants used throughout the TUI
pub mod layout {
    // === Section Heights ===

    /// Height of the header/banner section when expanded
    pub const HEADER_HEIGHT: u16 = 5;

    /// Height of the header/banner section when collapsed
    pub const HEADER_HEIGHT_COLLAPSED: u16 = 1;

    /// Height of the footer section
    pub const FOOTER_HEIGHT: u16 = 1;

    /// Height of the scan configuration section
    pub const SCAN_CONFIG_HEIGHT: u16 = 14;

    // === Component Heights ===

    /// Height of individual input components (targets, ports, options)
    pub const INPUT_COMPONENT_HEIGHT: u16 = 3;

    // === Margins and Padding ===

    /// Standard margin for internal layouts
    pub const STANDARD_MARGIN: u16 = 1;
}

/// Text constants used throughout the TUI
pub mod text {
    // === Banner Content ===

    /// ASCII art lines for the banner
    pub const ASCII_LINES: [&str; 4] = [
        ".----. .-. .-. .----..---.  .----. .---.   .--.  .-. .-.",
        "| {}  }| { } |{ {__ {_   _}{ {__  /  ___} / {} \\ |  `| |",
        "| .-. \\| {_} |.-._} } | |  .-._} }\\     }/  /\\  \\| |\\  |",
        "`-' `-'`-----'`----'  `-'  `----'  `---' `-'  `-'`-' `-'",
    ];

    /// Banner subtitle
    pub const BANNER_SUBTITLE: &str = "The Modern Day Port Scanner";

    /// Collapsed banner text
    pub const COLLAPSED_BANNER: &str = "RUSTSCAN";

    // === Section Titles ===

    /// Title for the scan configuration section
    pub const SCAN_CONFIG_TITLE: &str = "Scan Configuration";

    /// Title for the scan results section
    pub const SCAN_RESULTS_TITLE: &str = "Scan Results";

    // === Component Titles ===

    /// Title for the targets input component
    pub const TARGETS_TITLE: &str = "Targets";

    /// Title for the ports input component
    pub const PORTS_TITLE: &str = "Ports";

    /// Title for the options component
    pub const OPTIONS_TITLE: &str = "Options";

    // === Placeholder Text ===

    /// Placeholder text for targets input
    pub const TARGETS_PLACEHOLDER: &str =
        "Enter targets (e.g., 192.168.1.1, example.com, 10.0.0.0/24)";

    /// Placeholder text for ports input
    pub const PORTS_PLACEHOLDER: &str =
        "All ports (1-65535) - Enter custom ports (e.g., 80,443,22 or 1-1000)";

    // === Footer Links ===

    /// GitHub link text
    pub const GITHUB_LINK: &str = "https://github.com/RustScan/RustScan";

    /// Discord link text
    pub const DISCORD_LINK: &str = "http://discord.skerritt.blog";

    // === Help Text ===

    /// Navigation instruction text
    pub const NAVIGATION_HELP: &str = "[Tab] to navigate | [Enter] to start scan | [Q] to quit";
}
