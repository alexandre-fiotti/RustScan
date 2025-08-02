//! Output Capture Module
//!
//! This module provides functionality to capture all application output
//! (tracing logs, external command output) and redirect it to the TUI display.

use std::io::{self, Write};
use tracing_subscriber::fmt::writer::MakeWriter;

use crate::tui_app::models::OutputBuffer;

/// Custom writer that captures output and sends it to the OutputBuffer
#[derive(Debug, Clone)]
pub struct TuiWriter {
    buffer: OutputBuffer,
}

impl TuiWriter {
    /// Create a new TUI writer with the given output buffer
    pub fn new(buffer: OutputBuffer) -> Self {
        Self { buffer }
    }
}

impl Write for TuiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Convert bytes to string and add to buffer
        let text = String::from_utf8_lossy(buf);

        // Split into lines and add each line separately
        for line in text.lines() {
            if !line.trim().is_empty() {
                self.buffer.push_line(line.to_string());
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // Nothing to flush since we write directly to buffer
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for TuiWriter {
    type Writer = TuiWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }

    fn make_writer_for(&'a self, _meta: &tracing::Metadata<'_>) -> Self::Writer {
        self.clone()
    }
}

/// Initialize tracing with TUI output capture
pub fn init_tracing_capture(buffer: OutputBuffer) -> Result<(), Box<dyn std::error::Error>> {
    let writer = TuiWriter::new(buffer);

    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_env_filter("trace,crossterm=warn,ratatui=warn") // Capture all levels but reduce noise from crossterm/ratatui
        .without_time() // We don't need timestamps in TUI output
        .with_target(false) // Keep output clean
        .with_level(true) // Show log levels
        .init();

    Ok(())
}

/// Capture external command output to the buffer
pub fn capture_command_output(
    buffer: &OutputBuffer,
    command_name: &str,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
) {
    // Add command header
    buffer.push_line(format!("=== {} Output ===", command_name));

    // Add stdout
    if !stdout.is_empty() {
        let stdout_text = String::from_utf8_lossy(&stdout);
        for line in stdout_text.lines() {
            if !line.trim().is_empty() {
                buffer.push_line(line.to_string());
            }
        }
    }

    // Capture stderr if any
    if !stderr.is_empty() {
        buffer.push_line("--- STDERR ---".to_string());
        let stderr_text = String::from_utf8_lossy(&stderr);
        for line in stderr_text.lines() {
            if !line.trim().is_empty() {
                buffer.push_line(format!("ERROR: {}", line));
            }
        }
    }

    // Add separator
    buffer.push_line("".to_string());
}
