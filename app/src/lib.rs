use std::io::{self, stdout};
use std::rc::Rc;

use crossterm::cursor::{EnableBlinking, SetCursorStyle};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::*,
};

use editor;

// Initialize the terminal
pub fn init(filename: String) -> io::Result<()> {
    // Put stdout into raw mode (turn off canonical mode)
    enable_raw_mode()?;
    // Switches the terminal to an alternate screen
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableBlinking)?;
    stdout().execute(SetCursorStyle::BlinkingBar)?;

    // Draw the terminal widgets
    // Temporarily not handling errors
    let _ = run(filename);

    // Turn off raw mode for stdout (enable canonical mode)
    disable_raw_mode()?;
    // Exit the alternate screen
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// Main driver function
fn run(filename: String) -> io::Result<()> {
    // Create a new terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Struct to track the entire editing space
    let mut editor_space = editor::Editor::new(filename);
    // Loop while editing
    loop {
        terminal.draw(|frame| {
            ui(frame, &mut editor_space);
            frame.set_cursor(editor_space.raw_pos.0, editor_space.raw_pos.1);
        })?;
        // Get input and add to the string
        editor_space.handle_input();
        // Check if break loop
        if editor_space.break_loop {
            break;
        }
    }

    Ok(())
}

fn create_layouts(frame: &mut Frame) -> Vec<Rc<[Rect]>> {
    // Create tabs (TEMP)
    let tabs_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(5), Constraint::Percentage(95)],
    )
    .split(frame.size());

    // Create the rest of the frame
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(10), Constraint::Percentage(90)],
    )
    .split(tabs_layout[1]);

    let layouts = vec![tabs_layout, main_layout];

    return layouts;
}

// Define the frame ui
fn ui(frame: &mut Frame, editor_space: &mut editor::Editor) {
    let layouts = create_layouts(frame);
    let tabs_layout = &layouts[0];
    let main_layout = &layouts[1];
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
    // File explorer
    frame.render_widget(
        Block::new().title("Explorer").borders(Borders::ALL),
        main_layout[0],
    );
    // Main editor space
    if !editor_space.content.is_empty() {
        frame.render_widget(
            Paragraph::new(editor_space.get_paragraph()).block(Block::new().borders(Borders::ALL)),
            main_layout[1],
        );
    } else {
        frame.render_widget(
            Block::new().borders(Borders::ALL),
            main_layout[1],
        );
    }
    // Set the starting position for the cursor of the editor space if it hasn't been set
    if editor_space.start_cursor_set == false {
        editor_space.set_starting_pos((main_layout[1].x, main_layout[1].y), main_layout[1].width, main_layout[1].height);
    }
}