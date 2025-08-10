//! Repository Output Capture Module
//!
//! This module provides functionality to capture and redirect ALL output
//! from the entire RustScan repository to the TUI interface. This includes:
//! - Scan results from the core scanning engine
//! - Script execution output (nmap, custom scripts)
//! - Log messages from throughout the codebase  
//! - Command-line tool output
//! - Error messages and diagnostics
//!
//! The goal is to ensure that when running in TUI mode, ALL output that would
//! normally go to the terminal is instead captured and displayed in the
//! TUI results area, providing a unified output experience.

use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{self, Read};
use std::process::Output;
use std::sync::{Mutex, OnceLock};
use std::thread;

use crate::tui_app::shared::OutputBuffer;

/// Thread-safe global output buffer for TUI mode
/// Uses OnceLock for safer initialization compared to static mut
static TUI_OUTPUT_BUFFER: OnceLock<Mutex<Option<OutputBuffer>>> = OnceLock::new();

/// Standard PTY size for terminal emulation
const DEFAULT_PTY_SIZE: PtySize = PtySize {
    rows: 24,
    cols: 80,
    pixel_width: 0,
    pixel_height: 0,
};

/// Initialize repository-wide output capture for TUI mode
///
/// This should be called once when the TUI starts to enable output interception
/// for ALL output from the entire RustScan repository. Once initialized, any
/// function that checks `is_tui_mode()` can redirect its output to the TUI.
/// Subsequent calls will be ignored.
pub fn init_tui_output_capture(buffer: OutputBuffer) {
    let _ = TUI_OUTPUT_BUFFER.set(Mutex::new(Some(buffer)));
}

/// Check if repository-wide TUI output capture is enabled
///
/// Functions throughout the RustScan codebase should check this before
/// deciding whether to output to stdout/stderr or redirect to the TUI.
pub fn is_tui_mode() -> bool {
    TUI_OUTPUT_BUFFER
        .get()
        .and_then(|mutex| mutex.lock().ok())
        .map(|guard| guard.is_some())
        .unwrap_or(false)
}

/// Get a clone of the TUI output buffer if available
fn get_tui_buffer() -> Option<OutputBuffer> {
    TUI_OUTPUT_BUFFER.get()?.lock().ok()?.as_ref().cloned()
}

/// Log a command execution to the TUI buffer
fn log_command_execution(command: &str, args: &[&str]) {
    if let Some(buffer) = get_tui_buffer() {
        let cmd_line = if args.is_empty() {
            command.to_string()
        } else {
            format!("{} {}", command, args.join(" "))
        };
        buffer.push_line(format!("$ {}", cmd_line));
    }
}

/// Capture output from a PTY reader and stream to TUI buffer
fn capture_pty_output(
    mut reader: Box<dyn Read + Send>,
    tui_buffer: Option<OutputBuffer>,
) -> Vec<u8> {
    let mut captured_output = Vec::new();
    let mut buffer = [0u8; 1024];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                let chunk = &buffer[..n];
                captured_output.extend_from_slice(chunk);

                // Stream to TUI buffer if available
                if let Some(ref buffer) = tui_buffer {
                    let text = String::from_utf8_lossy(chunk);
                    for line in text.lines() {
                        if !line.trim().is_empty() {
                            buffer.push_line(line.to_string());
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }

    captured_output
}

/// Create a mock ExitStatus for compatibility with std::process::Output
///
/// This is a workaround since ExitStatus cannot be constructed directly
fn create_exit_status(success: bool) -> io::Result<std::process::ExitStatus> {
    let command = if success { "true" } else { "false" };
    std::process::Command::new(command)
        .output()
        .map(|output| output.status)
}

/// Execute a command with PTY output capture for TUI mode
///
/// This function can be used as a drop-in replacement for `std::process::Command::output()`
/// anywhere in the RustScan repository when TUI output capture is desired. It captures
/// ALL command output including ANSI colors and formatting.
pub fn execute_command_with_pty_capture(command: &str, args: &[&str]) -> anyhow::Result<Output> {
    let pty_system = native_pty_system();

    // Create PTY pair
    let pair = pty_system.openpty(DEFAULT_PTY_SIZE)?;

    // Build command
    let mut cmd = CommandBuilder::new(command);
    for arg in args {
        cmd.arg(arg);
    }

    // Log command execution
    log_command_execution(command, args);

    // Spawn command in PTY
    let mut child = pair.slave.spawn_command(cmd)?;

    // Get TUI buffer for the capture thread
    let tui_buffer = get_tui_buffer();

    // Capture output in separate thread
    let reader = pair.master.try_clone_reader()?;

    let capture_handle = thread::spawn(move || capture_pty_output(reader, tui_buffer));

    // Wait for command completion
    let exit_status = child.wait()?;

    // Collect captured output
    let stdout = capture_handle
        .join()
        .map_err(|_| anyhow::anyhow!("Output capture thread panicked"))?;

    // Create compatible Output structure
    let status = create_exit_status(exit_status.success())?;

    Ok(Output {
        status,
        stdout,
        stderr: Vec::new(), // PTY combines stdout/stderr
    })
}

/// Execute a shell command using the same pattern as the existing scripts module
///
/// Returns the command output as a string, or an error if the command fails.
pub fn execute_shell_command_for_tui(script: &str) -> anyhow::Result<String> {
    let (shell, shell_arg) = if cfg!(unix) {
        ("sh", "-c")
    } else {
        ("cmd.exe", "/c")
    };

    let output = execute_command_with_pty_capture(shell, &[shell_arg, script])?;

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Command failed with exit code: {}", code));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Directly capture text output to the TUI
///
/// This function allows any part of the RustScan repository to send output
/// directly to the TUI results area. Use this for:
/// - Log messages that should be visible in TUI
/// - Scan progress updates
/// - Error messages and warnings
/// - Any other text that should appear in the results
pub fn capture_output_line(line: String) {
    if let Some(buffer) = get_tui_buffer() {
        buffer.push_line(line);
    }
}

/// Capture multiple lines of output to the TUI
///
/// Convenience function for capturing multiple lines at once.
pub fn capture_output_lines(lines: Vec<String>) {
    if let Some(buffer) = get_tui_buffer() {
        for line in lines {
            buffer.push_line(line);
        }
    }
}
