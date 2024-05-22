use std::io::{self, stdout};
use std::rc::Rc;

use crossterm::cursor::{EnableBlinking, SetCursorStyle};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    execute,
};

use ratatui::{
    prelude::*,
    widgets::*,
};

use editor::Editor::*;
use config;

// Initialize the terminal
pub fn init(filename: String) -> io::Result<()> {
    // Put stdout into raw mode (turn off canonical mode)
    enable_raw_mode()?;
    // Set configuration
    let config = config::Config::new(SetCursorStyle::BlinkingBar, 4);
    // Switches the terminal to an alternate screen and changes the cursor
    execute!(
        stdout(),
        EnterAlternateScreen,
        EnableBlinking,
        config.cursor_style,
    )?;

    // Draw the terminal widgets
    // Temporarily not handling errors
    run(filename, config)?;

    // Turn off raw mode for stdout (enable canonical mode)
    disable_raw_mode()?;
    // Exit the alternate screen
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// Main driver function
fn run(filename: String, config: config::Config) -> io::Result<()> {
    // Create a new terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Struct to track the entire editing space
    let mut editor_space = EditorSpace::new(filename);

    // Loop while editing
    loop {
        terminal.draw(|frame| {
            ui(frame, &mut editor_space, &config);
            frame.set_cursor(editor_space.raw_pos.0 as u16, editor_space.raw_pos.1 as u16);
        })?;
        // Get input and add to the string
        editor_space.handle_input(&config);
        // Check if break loop
        if editor_space.break_loop {
            break;
        }
    }

    Ok(())
}

// Define the frame ui
fn ui(frame: &mut Frame, editor_space: &mut EditorSpace, config:&config::Config) {
    let layouts = create_layouts(frame);
    let tabs_layout = &layouts[0];
    let main_layout = &layouts[1];

    let tab_name = editor_space.filename.clone();
    frame.render_widget(
        Tabs::new(vec![tab_name, String::from("Tab 2"), String::from("Tab 3"), String::from("Tab 4")])
            .block(Block::bordered())
            .style(Style::default().white())
            .highlight_style(Style::default().white().on_blue().underline_color(Color::White).add_modifier(Modifier::BOLD))
            .select(0)
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
            Paragraph::new(editor_space.get_paragraph(config.tab_width)).block(Block::new().borders(Borders::ALL)),
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
        editor_space.set_starting_pos(
            (main_layout[1].x as usize,
                main_layout[1].y as usize),
                main_layout[1].width as usize,
                main_layout[1].height as usize
            );
    }
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