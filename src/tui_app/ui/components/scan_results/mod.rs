//! Results Module
//!
//! This module handles all aspects of capturing, processing, and managing
//! output from the entire RustScan repository - including scans, scripts,
//! logs, and any other command output. It also provides the UI component
//! for displaying all this captured output.

pub mod display;
pub mod output_capture;

// Re-export the main functionality
pub use output_capture::{
    capture_output_line, capture_output_lines, execute_command_with_pty_capture,
    execute_shell_command_for_tui, init_tui_output_capture, is_tui_mode,
};

// Re-export the display component
pub use display::ResultsComponent;
