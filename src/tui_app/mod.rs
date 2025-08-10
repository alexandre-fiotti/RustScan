//! TUI Application Module
//!
//! This module contains all components for the Text User Interface (TUI)
//! implementation of RustScan using ratatui.

pub mod app;
pub mod events;
pub mod message;
pub mod model;
pub mod scan_config;
pub mod shared;
pub mod ui;
pub mod update;
pub mod view;

pub use app::run_tui;
pub use message::Message;
pub use model::Model;
pub use scan_config::{ScanConfig, SelectedField};
pub use shared::{OutputBuffer, TextInput};
pub use ui::components::scan_results::{
    execute_shell_command_for_tui, init_tui_output_capture, is_tui_mode,
};
