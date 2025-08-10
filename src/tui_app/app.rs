//! Main TUI Application Entrypoint
//!
//! Provides a pure function `run_tui()` that owns setup, loop, and teardown.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::tui_app::events::handle_event;
use crate::tui_app::{
    ui::components::scan_results::init_tui_output_capture, update::update, view::view, Model,
};

/// Run the TUI application
pub fn run_tui() -> io::Result<()> {
    // Create model
    let mut model = Model::new();

    // Initialize TUI output capture
    init_tui_output_capture(model.output_buffer().clone());

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Add welcome message to output buffer
    model
        .output_buffer()
        .push_line("RustScan TUI - PTY-based command execution".to_string());
    model.output_buffer().push_line(
        "Enter targets and ports, then press Tab to navigate or Enter to confirm".to_string(),
    );
    model
        .output_buffer()
        .push_line("Use Shift+Up/Down, PageUp/PageDown, or mouse wheel to scroll".to_string());
    model.output_buffer().push_line("".to_string());

    // Run the loop
    let res = run_loop(&mut terminal, &mut model);

    // Restore terminal - more thorough cleanup
    disable_raw_mode()?;

    // Clear any pending input/mouse events before disabling mouse capture
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?; // Drain the event queue
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Ensure all terminal commands are flushed
    terminal.backend_mut().flush()?;

    // Small delay to let terminal process the disable commands
    thread::sleep(Duration::from_millis(50));

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

/// Run the main application loop
fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    model: &mut Model,
) -> io::Result<()> {
    loop {
        // Render current screen
        terminal.draw(|f| view(model, f))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(50))? {
            let event = event::read()?;
            if let Some(mut msg) = handle_event(model, event)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e:?}")))?
            {
                // Cascade updates as long as update returns a follow-up message
                loop {
                    if let Some(next) = update(model, msg) {
                        msg = next;
                        continue;
                    }
                    break;
                }
            }
        }

        // If a short-lived activation is in progress, finish it when due and then start scan
        if model.maybe_finish_button_activation() {
            if let Some(next) = update(model, crate::tui_app::message::AppMsg::StartScan.into()) {
                // Handle any cascaded follow-ups
                let mut msg = next;
                loop {
                    if let Some(next2) = update(model, msg) {
                        msg = next2;
                        continue;
                    }
                    break;
                }
            }
        }

        if model.should_quit() {
            break;
        }
    }
    Ok(())
}
