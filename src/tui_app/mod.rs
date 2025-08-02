//! Terminal User Interface Module
//!
//! This module contains all components for the TUI application,
//! including state management, event handling, UI components, and data models.

pub mod app;
pub mod events;
pub mod models;
pub mod state;
pub mod ui;

// Re-export commonly used items for convenience
pub use app::TuiApp;
pub use events::EventHandler;
pub use models::TextInput;
pub use state::{AppState, SelectedField};
