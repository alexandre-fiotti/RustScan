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

use crate::tui::{events::EventHandler, state::AppState, ui::UI};

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
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create app and run
        let mut app = TuiApp::new();
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
