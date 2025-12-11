//! Event handling for the TUI
//!
//! This module handles keyboard and other events.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use super::app::{AddingField, App, Message, Mode};

/// Handles keyboard input and returns appropriate messages
pub fn handle_event(app: &mut App) -> Result<Option<Message>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            return Ok(handle_key_event(app, key));
        }
    }
    Ok(None)
}

/// Handles a single key event based on the current mode
fn handle_key_event(app: &mut App, key: KeyEvent) -> Option<Message> {
    match &app.mode {
        Mode::Normal => handle_normal_mode(key),
        Mode::Adding(field) => handle_adding_mode(app, key, field.clone()),
        Mode::ConfirmDelete => handle_confirm_delete_mode(key),
    }
}

/// Handles key events in Normal mode
fn handle_normal_mode(key: KeyEvent) -> Option<Message> {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),

        // Navigation (Vim-like)
        KeyCode::Char('j') | KeyCode::Down => Some(Message::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::MoveUp),
        KeyCode::Char('g') => Some(Message::MoveToTop),
        KeyCode::Char('G') => Some(Message::MoveToBottom),
        KeyCode::Home => Some(Message::MoveToTop),
        KeyCode::End => Some(Message::MoveToBottom),

        // Actions
        KeyCode::Char('a') => Some(Message::StartAdding),
        KeyCode::Char('d') => Some(Message::StartDelete),
        KeyCode::Char('y') => Some(Message::CopyToClipboard),
        KeyCode::Char('x') | KeyCode::Enter => Some(Message::ExecuteCommand),

        _ => None,
    }
}

/// Handles key events in Adding mode
fn handle_adding_mode(app: &mut App, key: KeyEvent, field: AddingField) -> Option<Message> {
    match key.code {
        // Cancel
        KeyCode::Esc => Some(Message::CancelAdding),

        // Save (Ctrl+S)
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Message::ConfirmAdd)
        }

        // Next field (Tab)
        KeyCode::Tab => Some(Message::NextField),

        // Previous field (Shift+Tab or BackTab)
        KeyCode::BackTab => Some(Message::PrevField),

        // Enter in Tags field saves the command
        KeyCode::Enter if field == AddingField::Tags => Some(Message::ConfirmAdd),

        // Enter in other fields moves to next field
        KeyCode::Enter => Some(Message::NextField),

        // Pass other keys to the text area
        _ => {
            if let Some(textarea) = app.current_textarea_mut() {
                textarea.input(key);
            }
            None
        }
    }
}

/// Handles key events in ConfirmDelete mode
fn handle_confirm_delete_mode(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => Some(Message::ConfirmDelete),
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => Some(Message::CancelDelete),
        _ => None,
    }
}
