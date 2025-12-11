//! UI rendering logic (View in Elm Architecture)
//!
//! This module handles all the rendering of the TUI.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::app::{AddingField, App, Mode};

/// Main render function (View in Elm Architecture)
pub fn render(frame: &mut Frame, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Status bar
            Constraint::Length(1), // Help bar
        ])
        .split(frame.area());

    // Split main content into list and detail
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Render components
    render_list(frame, app, main_chunks[0]);
    render_detail(frame, app, main_chunks[1]);
    render_status_bar(frame, app, chunks[1]);
    render_help_bar(frame, app, chunks[2]);

    // Render popups if needed
    match &app.mode {
        Mode::Adding(_) => render_add_popup(frame, app),
        Mode::ConfirmDelete => render_delete_confirm(frame, app),
        _ => {}
    }
}

/// Renders the command list
fn render_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .db
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = format!(" {:>3} │ {}", entry.id, truncate_str(&entry.command, 40));
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Commands ")
                .title_style(Style::default().fg(Color::Cyan).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black));

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}

/// Renders the detail panel for the selected command
fn render_detail(frame: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(entry) = app.selected_entry() {
        let lines = vec![
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::DarkGray)),
                Span::styled(entry.id.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Command:",
                Style::default().fg(Color::Cyan).bold(),
            )]),
            Line::from(vec![Span::styled(
                &entry.command,
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description:",
                Style::default().fg(Color::Cyan).bold(),
            )]),
            Line::from(vec![Span::styled(
                &entry.description,
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Tags:",
                Style::default().fg(Color::Cyan).bold(),
            )]),
            Line::from(vec![Span::styled(
                entry.tags_display(),
                Style::default().fg(Color::Green),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    entry.created_at.format("%Y-%m-%d %H:%M").to_string(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ];

        Text::from(lines)
    } else {
        Text::from(vec![Line::from(vec![Span::styled(
            "No commands yet. Press 'a' to add one.",
            Style::default().fg(Color::DarkGray).italic(),
        )])])
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(" Details ")
                .title_style(Style::default().fg(Color::Cyan).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Renders the status bar
fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(msg) = &app.status_message {
        msg.clone()
    } else {
        format!(
            " {} command(s) │ Selected: {}/{}",
            app.entry_count(),
            if app.entry_count() > 0 {
                app.selected_index + 1
            } else {
                0
            },
            app.entry_count()
        )
    };

    let paragraph = Paragraph::new(status)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(paragraph, area);
}

/// Renders the help bar at the bottom
fn render_help_bar(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = match &app.mode {
        Mode::Normal => {
            " q: Quit │ a: Add │ d: Delete │ Enter/y: Copy │ j/↓: Down │ k/↑: Up │ g: Top │ G: Bottom "
        }
        Mode::Adding(_) => " Tab: Next Field │ Shift+Tab: Prev │ Ctrl+S: Save │ Esc: Cancel ",
        Mode::ConfirmDelete => " y: Confirm Delete │ n/Esc: Cancel ",
    };

    let paragraph =
        Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray).bg(Color::Black));

    frame.render_widget(paragraph, area);
}

/// Renders the add command popup
fn render_add_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 50, frame.area());

    // Clear the area first
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Add New Command ")
        .title_style(Style::default().fg(Color::Green).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    frame.render_widget(block, area);

    // Inner area for form fields
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Command
            Constraint::Length(3), // Description
            Constraint::Length(3), // Tags
            Constraint::Min(1),    // Spacer
        ])
        .split(area);

    // Determine which field is active
    let active_field = match &app.mode {
        Mode::Adding(field) => field.clone(),
        _ => AddingField::Command,
    };

    // Command input
    let command_block = Block::default()
        .title(" Command ")
        .borders(Borders::ALL)
        .border_style(if active_field == AddingField::Command {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    app.command_input.set_block(command_block);
    frame.render_widget(&app.command_input, inner[0]);

    // Description input
    let desc_block = Block::default()
        .title(" Description ")
        .borders(Borders::ALL)
        .border_style(if active_field == AddingField::Description {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    app.description_input.set_block(desc_block);
    frame.render_widget(&app.description_input, inner[1]);

    // Tags input
    let tags_block = Block::default()
        .title(" Tags (comma-separated) ")
        .borders(Borders::ALL)
        .border_style(if active_field == AddingField::Tags {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    app.tags_input.set_block(tags_block);
    frame.render_widget(&app.tags_input, inner[2]);
}

/// Renders the delete confirmation dialog
fn render_delete_confirm(frame: &mut Frame, app: &App) {
    let area = centered_rect(40, 20, frame.area());

    frame.render_widget(Clear, area);

    let text = if let Some(entry) = app.selected_entry() {
        format!(
            "Delete command #{}?\n\n\"{}\"\n\n[y] Yes  [n] No",
            entry.id,
            truncate_str(&entry.command, 30)
        )
    } else {
        "No command selected".to_string()
    };

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(" Confirm Delete ")
                .title_style(Style::default().fg(Color::Red).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Truncates a string to a maximum length
fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
