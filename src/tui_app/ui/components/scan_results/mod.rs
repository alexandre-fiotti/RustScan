//! Results Module
//!
//! This module handles all aspects of capturing, processing, and managing
//! output from the entire RustScan repository - including scans, scripts,
//! logs, and any other command output. It also provides the UI component
//! for displaying all this captured output.

pub mod display;

// Re-export the display component
pub use display::ResultsComponent;
