//! Shared primitives owned by the Model

pub mod button_mode;
pub mod output_buffer;
pub mod text_input;

pub use button_mode::ButtonMode;
pub use output_buffer::{OutputBuffer, ScrollInfo};
pub use text_input::TextInput;
