//! Main TUI Application Module
//!
//! This module contains the core TUI application logic for RustScan.
//! It coordinates between all other TUI modules.

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

use crate::tui_app::{
    ui::components::scan_results::init_tui_output_capture, ui::UI, AppState, EventHandler,
};

/// Main TUI Application
pub struct TuiApp {
    /// Application state
    state: AppState,
    /// Event handler
    event_handler: EventHandler,
    /// UI renderer
    ui: UI,
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            event_handler: EventHandler::new(),
            ui: UI::default(),
        }
    }

    /// Run the TUI application
    pub fn run() -> io::Result<()> {
        // Create app
        let mut app = TuiApp::new();

        // Initialize TUI output capture
        init_tui_output_capture(app.state.output_buffer().clone());

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Add welcome message to output buffer
        app.state
            .output_buffer()
            .push_line("RustScan TUI - PTY-based command execution".to_string());
        app.state.output_buffer().push_line(
            "Enter targets and ports, then press Tab to navigate or Enter to confirm".to_string(),
        );
        app.state
            .output_buffer()
            .push_line("Use Shift+Up/Down, PageUp/PageDown, or mouse wheel to scroll".to_string());
        app.state.output_buffer().push_line("".to_string());

        // Run the app
        let res = run_app(&mut terminal, &mut app);

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

    /// Get reference to the app state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Get mutable reference to the app state
    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut TuiApp,
) -> io::Result<()> {
    loop {
        // Render current screen
        terminal.draw(|f| {
            app.ui.render(f, app.state());
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(250))? {
            let event = event::read()?;
            app.event_handler.handle_event(&mut app.state, event)?;
        }

        if app.state().should_quit() {
            break;
        }
    }
    Ok(())
}
