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

// Define the frame ui
fn ui(frame: &mut Frame, editor: &mut EditorSpace) {
	// Construct the layout of the frame
	let layouts = create_layouts(frame);
	// The layout for the tabs bar
	let tabs_layout = &layouts[0];
	// The layout for the editor space and the explorer
	let main_layout = &layouts[1];

	let tab_name = editor.filename.clone();
	// Render tabs
	frame.render_widget(
		Tabs::new(vec![
			tab_name,
			String::from("Tab 2"),
			String::from("Tab 3"),
			String::from("Tab 4"),
		])
		.block(
			Block::new()
				.fg(editor.config.theme.app_fg)
				.bg(editor.config.theme.app_bg)
				.borders(Borders::ALL),
		)
		.style(Style::default().white())
		.highlight_style(
			Style::default()
				.fg(editor.config.theme.tab_fg)
				.bg(editor.config.theme.tab_bg)
				.underline_color(editor.config.theme.tab_fg)
				.add_modifier(Modifier::BOLD),
		)
		.select(0)
		.divider(symbols::DOT)
		.padding(" ", " "),
		tabs_layout[0],
	);
	// File explorer
	frame.render_widget(
		Block::new()
			.title("Explorer")
			.fg(editor.config.theme.app_fg)
			.bg(editor.config.theme.app_bg)
			.borders(Borders::ALL),
		main_layout[0],
	);
	// Set the starting position for the cursor of the editor space if it hasn't been set
	if !editor.start_cursor_set {
		let _ = editor.init_editor(
			(main_layout[1].x as usize, main_layout[1].y as usize),
			main_layout[1].width as usize,
			main_layout[1].height as usize,
		);
	}
	// Main editor space
	if !editor.blocks.as_ref().unwrap().blocks_list.is_empty() {
		frame.render_widget(
			editor.get_paragraph().block(
				Block::new()
					.fg(editor.config.theme.app_fg)
					.bg(editor.config.theme.app_bg)
					.borders(Borders::ALL),
			),
			main_layout[1],
		);
	} else {
		// If the file is empty, make an empty block
		frame.render_widget(
			Block::new()
				.fg(editor.config.theme.app_fg)
				.bg(editor.config.theme.app_bg)
				.borders(Borders::ALL),
			main_layout[1],
		);
	}
}

// Create the ui layouts for the frame
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
		[Constraint::Percentage(14), Constraint::Percentage(86)],
	)
	.split(tabs_layout[1]);

	let layouts = vec![tabs_layout, main_layout];

	layouts
}
