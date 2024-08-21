use std::{
	io::{self, stdout},
	rc::Rc,
};

use crossterm::{
	cursor::EnableBlinking,
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use config::config::Config;
use editor::editor::*;

// Initialize the terminal
pub fn init(filename: String) -> io::Result<()> {
	// Put stdout into raw mode (turn off canonical mode)
	enable_raw_mode()?;
	// Set configuration
	let config = Config::default();
	// Switches the terminal to an alternate screen and changes the cursor
	execute!(
		stdout(),
		EnterAlternateScreen,
		EnableBlinking,
		config.cursor_style,
	)?;

	// Draw the terminal widgets
	match run(filename, config) {
		Ok(_) => (),
		Err(_) => {
			// Turn off raw mode for stdout (enable canonical mode)
			disable_raw_mode()?;
			// Exit the alternate screen
			execute!(stdout(), LeaveAlternateScreen,)?;
			panic!("An error has occurred");
		}
	};

	// Turn off raw mode for stdout (enable canonical mode)
	disable_raw_mode()?;
	// Exit the alternate screen
	execute!(stdout(), LeaveAlternateScreen,)?;

	Ok(())
}

// Main driver function
fn run(filename: String, config: Config) -> io::Result<()> {
	// Create a new terminal
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	// Struct to track the entire editing space
	let mut editor_space = EditorSpace::new(filename, config);

	// Loop while editing
	loop {
		// Draw the frame
		terminal.draw(|frame| {
			ui(frame, &mut editor_space);
			frame.set_cursor(
				(editor_space.cursor_position[0] + editor_space.width.0 + 1) as u16,
				(editor_space.cursor_position[1] + editor_space.height.0 + 1) as u16,
			);
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

// Build the layout for displaying the widgets
fn build_layout(frame: &mut Frame) -> Rc<[Rect]> {
	/* The width of the widget that displays the line numbers.
	This should be 2 greater than the number of digits
	to display (9 = 7 digits). */
	let line_nums_width = 9;
	// Create the layout
	Layout::new(
		Direction::Horizontal,
		[
			Constraint::Length(line_nums_width),
			Constraint::Length(frame.size().width - line_nums_width),
		],
	)
	// Split it over the entire frame
	.split(frame.size())
}

// Define the frame ui
fn ui(frame: &mut Frame, editor: &mut EditorSpace) {
	let layout = build_layout(frame);

	// Set the starting position for the cursor of the editor space if it hasn't been set
	if !editor.start_cursor_set {
		let _ = editor.init_editor(
			(layout[1].x as usize, layout[1].y as usize),
			layout[1].width as usize,
			layout[1].height as usize,
		);
	}
	// Main editor space
	if !editor.blocks.as_ref().unwrap().blocks_list.is_empty() {
		// Clone the config for the editor
		let config = editor.config.clone();
		frame.render_widget(
			editor.get_paragraph().block(
				Block::new()
					.fg(config.theme.app_fg)
					.bg(config.theme.app_bg)
					.borders(Borders::ALL),
			),
			layout[1],
		);
		// Render line numbers
		frame.render_widget(
			editor.get_line_numbers_paragraph().block(
				Block::new()
					.fg(config.theme.app_fg)
					.bg(config.theme.app_bg)
					.borders(Borders::all()),
			),
			layout[0],
		);
	} else {
		// If the file is empty, make an empty block
		frame.render_widget(
			Block::new()
				.fg(editor.config.theme.app_fg)
				.bg(editor.config.theme.app_bg)
				.borders(Borders::ALL),
			layout[1],
		);
		// If the file is empty, make an empty block
		frame.render_widget(
			Block::new()
				.fg(editor.config.theme.app_fg)
				.bg(editor.config.theme.app_bg)
				.borders(Borders::ALL),
			layout[0],
		);
	}
}
