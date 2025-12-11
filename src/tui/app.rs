//! Application state management (Model + Update in Elm Architecture)
//!
//! This module contains the application state and update logic for the TUI.

use anyhow::Result;
use tui_textarea::TextArea;

use crate::models::{CommandDatabase, CommandEntry};
use crate::storage::Storage;

/// Application mode (state machine)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode - browsing the command list
    Normal,
    /// Adding a new command
    Adding(AddingField),
    /// Confirming deletion
    ConfirmDelete,
}

/// Which field is being edited in Adding mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddingField {
    Command,
    Description,
    Tags,
}

/// Message type for state updates (like Redux actions)
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    MoveUp,
    MoveDown,
    MoveToTop,
    MoveToBottom,

    // Mode transitions
    StartAdding,
    CancelAdding,
    ConfirmAdd,
    NextField,
    PrevField,

    // Delete
    StartDelete,
    CancelDelete,
    ConfirmDelete,

    // Actions
    CopyToClipboard,

    // Exit
    Quit,
}

/// Application state (Model in Elm Architecture)
pub struct App<'a> {
    /// Current mode
    pub mode: Mode,

    /// Command database
    pub db: CommandDatabase,

    /// Storage handler
    storage: Storage,

    /// Currently selected index in the list
    pub selected_index: usize,

    /// Text areas for adding new command
    pub command_input: TextArea<'a>,
    pub description_input: TextArea<'a>,
    pub tags_input: TextArea<'a>,

    /// Status message to display
    pub status_message: Option<String>,

    /// Whether the app should quit
    pub should_quit: bool,
}

impl<'a> App<'a> {
    /// Creates a new App instance
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;
        let db = storage.load()?;

        let mut command_input = TextArea::default();
        command_input.set_placeholder_text("Enter command...");
        command_input.set_cursor_line_style(ratatui::style::Style::default());

        let mut description_input = TextArea::default();
        description_input.set_placeholder_text("Enter description...");
        description_input.set_cursor_line_style(ratatui::style::Style::default());

        let mut tags_input = TextArea::default();
        tags_input.set_placeholder_text("Enter tags (comma-separated)...");
        tags_input.set_cursor_line_style(ratatui::style::Style::default());

        Ok(Self {
            mode: Mode::Normal,
            db,
            storage,
            selected_index: 0,
            command_input,
            description_input,
            tags_input,
            status_message: None,
            should_quit: false,
        })
    }

    /// Returns the currently selected command entry
    pub fn selected_entry(&self) -> Option<&CommandEntry> {
        self.db.entries.get(self.selected_index)
    }

    /// Returns the number of entries
    pub fn entry_count(&self) -> usize {
        self.db.entries.len()
    }

    /// Update the application state based on a message (Update in Elm Architecture)
    pub fn update(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::MoveUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Message::MoveDown => {
                if self.selected_index + 1 < self.entry_count() {
                    self.selected_index += 1;
                }
            }
            Message::MoveToTop => {
                self.selected_index = 0;
            }
            Message::MoveToBottom => {
                if !self.db.entries.is_empty() {
                    self.selected_index = self.entry_count() - 1;
                }
            }
            Message::StartAdding => {
                self.mode = Mode::Adding(AddingField::Command);
                self.clear_inputs();
            }
            Message::CancelAdding => {
                self.mode = Mode::Normal;
                self.clear_inputs();
            }
            Message::NextField => {
                if let Mode::Adding(field) = &self.mode {
                    self.mode = Mode::Adding(match field {
                        AddingField::Command => AddingField::Description,
                        AddingField::Description => AddingField::Tags,
                        AddingField::Tags => AddingField::Command,
                    });
                }
            }
            Message::PrevField => {
                if let Mode::Adding(field) = &self.mode {
                    self.mode = Mode::Adding(match field {
                        AddingField::Command => AddingField::Tags,
                        AddingField::Description => AddingField::Command,
                        AddingField::Tags => AddingField::Description,
                    });
                }
            }
            Message::ConfirmAdd => {
                self.add_command()?;
                self.mode = Mode::Normal;
            }
            Message::StartDelete => {
                if !self.db.entries.is_empty() {
                    self.mode = Mode::ConfirmDelete;
                }
            }
            Message::CancelDelete => {
                self.mode = Mode::Normal;
            }
            Message::ConfirmDelete => {
                self.delete_selected()?;
                self.mode = Mode::Normal;
            }
            Message::CopyToClipboard => {
                self.copy_to_clipboard()?;
            }
            Message::Quit => {
                self.should_quit = true;
            }
        }
        Ok(())
    }

    /// Clears all input fields
    fn clear_inputs(&mut self) {
        self.command_input = TextArea::default();
        self.command_input.set_placeholder_text("Enter command...");
        self.command_input
            .set_cursor_line_style(ratatui::style::Style::default());

        self.description_input = TextArea::default();
        self.description_input
            .set_placeholder_text("Enter description...");
        self.description_input
            .set_cursor_line_style(ratatui::style::Style::default());

        self.tags_input = TextArea::default();
        self.tags_input
            .set_placeholder_text("Enter tags (comma-separated)...");
        self.tags_input
            .set_cursor_line_style(ratatui::style::Style::default());
    }

    /// Adds a new command from the input fields
    fn add_command(&mut self) -> Result<()> {
        let command = self.command_input.lines().join("\n").trim().to_string();
        let description = self.description_input.lines().join("\n").trim().to_string();
        let tags_str = self.tags_input.lines().join("\n").trim().to_string();

        if command.is_empty() {
            self.status_message = Some("Command cannot be empty".to_string());
            return Ok(());
        }

        let tags: Vec<String> = if tags_str.is_empty() {
            vec![]
        } else {
            tags_str.split(',').map(|s| s.trim().to_string()).collect()
        };

        let id = self.db.add(command, description, tags);
        self.storage.save(&self.db)?;
        self.status_message = Some(format!("✓ Command added (ID: {})", id));
        self.clear_inputs();

        // Select the newly added item
        self.selected_index = self.entry_count().saturating_sub(1);

        Ok(())
    }

    /// Deletes the currently selected command
    fn delete_selected(&mut self) -> Result<()> {
        if let Some(entry) = self.selected_entry() {
            let id = entry.id;
            self.db.remove_by_id(id);
            self.storage.save(&self.db)?;
            self.status_message = Some(format!("✓ Command {} deleted", id));

            // Adjust selection if needed
            if self.selected_index >= self.entry_count() && self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
        Ok(())
    }

    /// Copies the selected command to clipboard
    fn copy_to_clipboard(&mut self) -> Result<()> {
        if let Some(entry) = self.selected_entry() {
            let mut clipboard = arboard::Clipboard::new()?;
            clipboard.set_text(&entry.command)?;
            self.status_message = Some("✓ Copied to clipboard".to_string());
        }
        Ok(())
    }

    /// Returns the current text area based on the adding field
    pub fn current_textarea_mut(&mut self) -> Option<&mut TextArea<'a>> {
        match &self.mode {
            Mode::Adding(field) => Some(match field {
                AddingField::Command => &mut self.command_input,
                AddingField::Description => &mut self.description_input,
                AddingField::Tags => &mut self.tags_input,
            }),
            _ => None,
        }
    }

    /// Reloads the database from storage
    #[allow(dead_code)]
    pub fn reload(&mut self) -> Result<()> {
        self.db = self.storage.load()?;
        Ok(())
    }
}
