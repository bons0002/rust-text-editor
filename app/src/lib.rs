use std::io::{self, stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::*,
};

use editor;

// Main driver function
pub fn run(filename: String) -> io::Result<()> {
    // Put stdout into raw mode (turn off canonical mode)
    enable_raw_mode()?;
    // Switches the terminal to an alternate screen
    stdout().execute(EnterAlternateScreen)?;

    // Draw the terminal widgets
    // Temporarily not handling errors
    let _ = draw_terminal(filename);

    // Turn off raw mode for stdout (enable canonical mode)
    disable_raw_mode()?;
    // Exit the alternate screen
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// Create the terminal and draw the ui
fn draw_terminal(filename: String) -> io::Result<()> {
    // Create a new terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Text that will be printed to the editor
    let mut editor_space = editor::Editor::new(filename);
    for i in 0..50 {
        terminal.draw(|frame| {
            ui(frame, &editor_space.filename, &editor_space.content);
            frame.set_cursor(0, 0);
        })?;
        // Get input and add to the string
        editor::handle_input(&mut editor_space);
    }

    Ok(())
}

// Define the frame ui
fn ui(frame: &mut Frame, _filename: &str, text: &str) {
    // Create tabs (TEMP)
    let tabs_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(5), Constraint::Percentage(95)],
    )
    .split(frame.size());
    frame.render_widget(
        Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .block(Block::bordered())
            .style(Style::default().white())
            .highlight_style(Style::default().green())
            .select(2)
            .divider(symbols::DOT)
            .padding(" ", " "),
    tabs_layout[0]
    );
    // Create the rest of the frame
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(10), Constraint::Percentage(90)],
    )
    .split(tabs_layout[1]);
    // File explorer
    frame.render_widget(
        Block::new().title("Explorer").borders(Borders::ALL),
        main_layout[0],
    );
    // Main editor space
    frame.render_widget(
        Paragraph::new(text).block(Block::new().borders(Borders::ALL)),
        main_layout[1],
    );
}