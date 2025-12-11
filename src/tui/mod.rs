//! TUI (Terminal User Interface) module
//!
//! This module provides an interactive terminal interface for cmd-keeper,
//! similar to lazygit.

mod app;
mod event;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub use app::App;

/// Runs the TUI application
pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application loop
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle events
        if let Some(msg) = event::handle_event(app)? {
            app.update(msg)?;
        }

        // Clear status message after a while (simple approach)
        // In a more complex app, you'd use a timer
        if app.status_message.is_some() {
            // Keep message visible for one more frame, then it will be shown
            // and cleared on the next action
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
