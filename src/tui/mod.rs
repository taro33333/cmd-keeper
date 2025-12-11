//! TUI (Terminal User Interface) module
//!
//! This module provides an interactive terminal interface for cmd-keeper,
//! similar to lazygit.

mod app;
mod event;
mod ui;

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Write};
use std::process::Command;

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

        // Check if command execution is requested
        if app.execute_requested {
            app.execute_requested = false;

            if let Some(command) = app.selected_command() {
                // Execute the command
                let result = execute_command(terminal, &command);

                // Update status message based on result
                match result {
                    Ok(exit_code) => {
                        if exit_code == 0 {
                            app.status_message =
                                Some("✓ Command executed successfully".to_string());
                        } else {
                            app.status_message =
                                Some(format!("⚠ Command exited with code {}", exit_code));
                        }
                    }
                    Err(e) => {
                        app.status_message = Some(format!("✗ Error: {}", e));
                    }
                }
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Executes a shell command, temporarily suspending the TUI
///
/// This function:
/// 1. Leaves the alternate screen and disables raw mode
/// 2. Executes the command with inherited stdin/stdout/stderr
/// 3. Waits for user to press Enter
/// 4. Restores the TUI
fn execute_command(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    command: &str,
) -> Result<i32> {
    // Step 1: Leave alternate screen and disable raw mode
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    // Step 2: Parse the command using shell-words
    let args = shell_words::split(command).context("Failed to parse command")?;

    if args.is_empty() {
        // Re-enter TUI immediately if command is empty
        enable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        terminal.clear()?;
        anyhow::bail!("Empty command");
    }

    // Step 3: Print command being executed
    println!("\n\x1b[1;36m$ {}\x1b[0m\n", command);

    // Step 4: Execute the command
    let exit_code = match Command::new(&args[0])
        .args(&args[1..])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            let status = child.wait().context("Failed to wait for command")?;
            status.code().unwrap_or(-1)
        }
        Err(e) => {
            eprintln!("\n\x1b[1;31mError:\x1b[0m {}", e);
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Command not found: {}", args[0]);
            }
            -1
        }
    };

    // Step 5: Print exit status and wait for user input
    println!();
    if exit_code == 0 {
        println!("\x1b[1;32m✓ Command completed successfully\x1b[0m");
    } else {
        println!("\x1b[1;33m⚠ Command exited with code {}\x1b[0m", exit_code);
    }

    println!("\n\x1b[2mPress Enter to return to cmd-keeper...\x1b[0m");
    io::stdout().flush()?;

    // Wait for Enter key
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Step 6: Re-enter alternate screen and enable raw mode
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    terminal.clear()?;

    Ok(exit_code)
}
