//! Models Module
//!
//! This module contains data models and state containers used throughout
//! the TUI application.
//!
//! Models are self-contained data structures that encapsulate specific
//! state and provide methods to manipulate that state, such as text inputs,
//! configuration objects, and other data containers.

pub mod text_input;

pub use text_input::TextInput;
