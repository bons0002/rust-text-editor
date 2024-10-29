use std::{
	env::consts,
	io::{self, stdout, Error},
	rc::Rc,
};

use crossterm::{
	cursor::EnableBlinking,
	event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
	layout::{Constraint, Direction, Layout, Rect},
	prelude::CrosstermBackend,
	terminal::Terminal,
	text::Text,
	widgets::{Block, BorderType, Borders, Paragraph},
	Frame,
};

use config::config::Config;
use editor::editor::EditorSpace;

// Main driver function
pub fn run(filename: String) -> io::Result<()> {
	// Initialize the config and terminal
	let (config, mut terminal) = init()?;
	// Struct to track the entire editing space
	let mut editor_space = EditorSpace::new(filename, config);

	// Flag to break the below loop (ending app execution)
	let mut break_loop = false;

	// Run the app
	loop {
		// Draw the frame in the terminal
		terminal.draw(|frame| {
			// Draw the ui
			ui(frame, &mut editor_space);
		})?;
		// Get input within the editor space
		editor_space.handle_input(&mut break_loop);
		// Check if user wants to quit the app
		if break_loop {
			break;
		}
	}

	// Reset variables when leaving the app
	end()?;

	Ok(())
}

// Initialize the terminal and the config
fn init() -> Result<(Config, Terminal<CrosstermBackend<io::Stdout>>), Error> {
	// Create a default config
	let config = Config::default();

	// Put stdout into raw mode (turn off canonical mode)
	enable_raw_mode()?;
	// Switches the terminal to an alternate screen and changes the cursor
	execute!(
		stdout(),
		EnterAlternateScreen,
		EnableBlinking,
		config.cursor_style,
	)?;
	// Only enable keyboard enhancments if not on windows
	if consts::OS != "windows" {
		execute!(
			stdout(),
			PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
		)?;
	}

	// Create a new terminal
	let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	// Return the config and terminal
	Ok((config, terminal))
}

// Define the frame ui
fn ui(frame: &mut Frame, editor: &mut EditorSpace) {
	// Create the layout for the line numbers and editor widgets
	let (editor_layout, keybinds_layout) = build_layout(frame);
	// Render the editor and line numbers ui
	editor.render_ui(frame, editor_layout);
	// Render the keybinds
	render_keybinds(frame, keybinds_layout, editor);
}

// Build the layout for displaying the widgets
fn build_layout(frame: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>) {
	/* The height of the widget displaying all keybindings.
	3 because 1 line of text and 1 line for the bottom border */
	let keybinds_height = 3;
	// The vertical split of the frame
	let outer_layout = Layout::new(
		Direction::Vertical,
		[
			Constraint::Length(frame.size().height - keybinds_height),
			Constraint::Length(keybinds_height),
		],
	)
	// Split over the entire frame
	.split(frame.size());
	/* The width of the widget that displays the line numbers.
	This should be 2 greater than the number of digits
	to display (9 = 7 digits). */
	let line_nums_width = 9;
	// Create the layout for the EditorSpace and the line numbers
	let editor_layout = Layout::new(
		Direction::Horizontal,
		[
			Constraint::Length(line_nums_width),
			Constraint::Length(frame.size().width - line_nums_width),
		],
	)
	// Split it over the top widget from the outer_layout
	.split(outer_layout[0]);

	(editor_layout, outer_layout)
}

// Render the widget displaying the keybinds
fn render_keybinds(frame: &mut Frame, layout: Rc<[Rect]>, editor: &mut EditorSpace) {
	// The keybinds that are displayed
	let keybinds = format!(
		"\'<^s> Save\' \t \'<^q> Quit\' \t \'<^c> Copy\' \t \'<^x> Cut\'\n\
         \'<^p> Paste\' {} \'<^z> Undo\' \t \'<^r> Redo\' \t \'<^Arrows> Jump\'",
		&" ".repeat(editor.config.tab_width - 1)
	)
	.replace('\t', &" ".repeat(editor.config.tab_width));

	// Render the keybinds widget
	frame.render_widget(
		Paragraph::new(Text::from(keybinds)).block(
			Block::new()
				.borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
				.border_type(BorderType::Thick),
		),
		layout[1],
	);
}

// Reset the terminal before exiting the app
fn end() -> io::Result<()> {
	// Turn off raw mode for stdout (enable canonical mode)
	disable_raw_mode()?;
	// Exit the alternate screen
	execute!(stdout(), LeaveAlternateScreen)?;

	// Keyboard enhancements were only enabled on non-windows platforms
	if consts::OS != "windows" {
		execute!(stdout(), PopKeyboardEnhancementFlags)?;
	}

	Ok(())
}
