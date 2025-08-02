//! TUI Application Module
//!
//! This module contains all components for the Text User Interface (TUI)
//! implementation of RustScan using ratatui.

pub mod app;
pub mod events;
pub mod models;
pub mod state;
pub mod ui;

pub use app::TuiApp;
pub use events::EventHandler;
pub use models::{OutputBuffer, TextInput};
pub use state::{AppState, HoveredField, SelectedField};
pub use ui::components::scan_results::{
    execute_shell_command_for_tui, init_tui_output_capture, is_tui_mode,
};
