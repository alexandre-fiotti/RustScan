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
use std::io;

use crate::tui_app::{output_capture, ui::UI, AppState, EventHandler};

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
        // Create app first to get access to output buffer
        let mut app = TuiApp::new();

        // Initialize tracing to capture all output to TUI buffer
        // This replaces the env_logger::init() from main.rs when in TUI mode
        if let Err(e) = output_capture::init_tracing_capture(app.state.output_buffer().clone()) {
            eprintln!("Failed to initialize tracing capture: {}", e);
        }

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Add welcome message to output buffer
        app.state
            .output_buffer()
            .push_line("RustScan TUI - All scan output and logs will appear here".to_string());
        app.state
            .output_buffer()
            .push_line("Use Shift+Up/Down or PageUp/PageDown to scroll through output".to_string());
        app.state.output_buffer().push_line("".to_string());

        // Run the app
        let res = run_app(&mut terminal, &mut app);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

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
