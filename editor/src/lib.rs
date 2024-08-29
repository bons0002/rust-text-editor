pub mod editor {

	use std::{
		fs::{read_to_string, File, OpenOptions},
		io::Error,
		path::Path,
		rc::Rc,
		time::Duration,
	};

	use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
	use ratatui::{
		layout::Rect,
		style::{Style, Stylize},
		text::{Line, Span, Text},
		widgets::{Block, Borders, Paragraph},
		Frame,
	};
	use rayon::iter::{
		IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
	};
	use unicode_segmentation::UnicodeSegmentation;

	use config::config::Config;

	mod blocks;
	use blocks::Blocks;

	// Module containing all the functionality of each key. Called in handle_input
	mod key_functions;
	use key_functions::highlight_selection::{self, Selection};

	// Testing module found at crate/src/editor/tests.rs
	#[cfg(test)]
	mod tests;

	pub struct EditorSpace {
		// Object containing multiple text blocks
		blocks: Option<Blocks>,
		// Flag for whether to break rendering loop in main app
		pub break_loop: bool,
		// The config of the editor
		config: Config,
		// Position of cursor on the screen
		cursor_position: [usize; 2],
		// The file that is open
		file: File,
		// Name of file opened in current editor space
		filename: String,
		// The number of lines in the entire file
		file_length: usize,
		// Vertical bounds of the editor widget
		height: (usize, usize),
		// Position used to access indices within graphemes vectors
		index_position: usize,
		// Used to scroll the text on screen (and calculate line number)
		scroll_offset: usize,
		// Structure keeping track of the highlighted selection of text
		selection: Selection,
		// Used to store the horizontal position in the text
		stored_position: usize,
		// Track if the editor has been intialized
		is_initialized: bool,
		// Actual position on the current line of text
		text_position: usize,
		// Horizontal bounds of the editor block
		width: (usize, usize),
	}

	impl EditorSpace {
		// Open (and create if necessary) the given file
		fn open_file(filename: &str) -> File {
			// Check if a file exists, if not create it
			if !Path::new(filename).exists() {
				File::create(filename).unwrap();
			}
			// Open the file in read-write mode
			let file = match OpenOptions::new().read(true).write(true).open(filename) {
				Ok(file) => file,
				Err(err) => panic!("{}", err),
			};
			file
		}

		pub fn new(filename: String, config: Config) -> Self {
			// Open (and create if necessary) the given file
			let file = Self::open_file(&filename);
			// Construct an EditorSpace
			EditorSpace {
				blocks: None,
				break_loop: false,
				config,
				cursor_position: [0, 0],
				file,
				filename,
				file_length: 0,
				height: (0, 0),
				index_position: 0,
				scroll_offset: 0,
				selection: Selection::new(),
				stored_position: 0,
				is_initialized: false,
				text_position: 0,
				width: (0, 0),
			}
		}

		// Calculate the start and end indices for highlighting characters in the paragraph
		fn calc_highlight_indices(&self) -> (usize, usize) {
			if self.selection.start[1] < self.blocks.as_ref().unwrap().starting_line_num {
				// The starting line number of the
				let line = self.blocks.as_ref().unwrap().starting_line_num;
				(0, self.selection.end[1] - line)
			} else {
				let line = self.blocks.as_ref().unwrap().starting_line_num;
				(self.selection.start[1] - line, self.selection.end[1] - line)
			}
		}

		// Highlight character on one line and return them as a Span
		fn highlight_one_line(&self, loc: usize, character: String) -> Span {
			// If within selection, highlight character
			if loc >= self.selection.start[0] && loc < self.selection.end[0] {
				Span::from(character)
					.style(Style::default().bg(self.config.theme.selection_highlight))
			} else {
				Span::from(character)
			}
		}

		// Highlight character if on the first line of a multiline selection
		fn highlight_first_line(&self, loc: usize, character: String) -> Span {
			// Highlight all characters on the line after the cursor
			if loc >= self.selection.start[0] {
				Span::from(character)
					.style(Style::default().bg(self.config.theme.selection_highlight))
			} else {
				Span::from(character)
			}
		}

		// Highlight character if on the last line of a multiline selection
		fn highlight_last_line(&self, loc: usize, character: String) -> Span {
			// Highlight all characters on the line before the cursor
			if loc < self.selection.end[0] {
				Span::from(character)
					.style(Style::default().bg(self.config.theme.selection_highlight))
			} else {
				Span::from(character)
			}
		}

		// Highlight an individual grapheme
		fn highlight_grapheme(
			&self,
			idx: usize,
			loc: usize,
			character: &str,
			tab_char: &str,
			start_line: usize,
			end_line: usize,
		) -> Span {
			if idx == start_line && start_line == end_line {
				self.highlight_one_line(loc, String::from(character).replace("\t", tab_char))
			// If on first line (and there are multiple lines in selection)
			} else if idx == start_line {
				// Highlight character
				self.highlight_first_line(loc, String::from(character).replace("\t", tab_char))
			// If on last line (and there are multiple lines in selection)
			} else if idx == end_line {
				// Highlight character
				self.highlight_last_line(loc, String::from(character).replace("\t", tab_char))
			// If between first and last line in multine selection
			} else if idx > start_line && idx < end_line {
				Span::from(String::from(character).replace("\t", tab_char))
					.style(Style::default().bg(self.config.theme.selection_highlight))
			// If not in selection
			} else {
				Span::from(String::from(character).replace("\t", tab_char))
			}
		}

		// Highlight a line of text
		fn highlight_line(&self, idx: usize, line: &str) -> Line {
			// Indices for highlighting within the paragraph
			let (start_line, end_line) = self.calc_highlight_indices();
			// Start tab with a vertical line
			let mut tab_char = String::from("\u{2502}");
			// Iterator to create a string of tab_width - 1 number of spaces
			tab_char.push_str(&" ".repeat(self.config.tab_width - 1));
			// A vector of the graphemes as stylized spans
			let graphemes: Vec<Span> = line
				.graphemes(true)
				.collect::<Vec<&str>>()
				.into_par_iter()
				.enumerate()
				.map(|(loc, character)| {
					// Highlight the grapheme
					self.highlight_grapheme(idx, loc, character, &tab_char, start_line, end_line)
				})
				.collect();

			Line::from(graphemes)
		}

		// Create a Line struct from the given String line
		fn parse_line(&self, idx: usize, line: &str) -> Line {
			// Top line of the widget
			let top_line = self.scroll_offset;
			// The bottom line of the widget
			let bottom_line = self.height.1 - self.height.0 - 3 + self.scroll_offset;
			// Start tab with a vertical line
			let mut tab_char = String::from("\u{2502}");
			// Iterator to create a string of tab_width - 1 number of	 spaces
			tab_char.push_str(&" ".repeat(self.config.tab_width - 1));

			// Only highlight if selection isn't empty (and its within the widget's bounds)
			if !self.selection.is_empty && idx >= top_line && idx <= bottom_line {
				// Highlight characters
				return self.highlight_line(idx, line);
			}

			Line::from(String::from(line).replace("\t", &tab_char))
		}

		// Get the current line number for the given position
		fn get_line_num(&self, position: usize) -> usize {
			position + self.scroll_offset + self.blocks.as_ref().unwrap().starting_line_num
		}

		// Check that there are enough blocks loaded in, and return the blocks
		fn check_blocks(&mut self) -> Blocks {
			// Clone the blocks of text
			let mut blocks = self.blocks.as_ref().unwrap().clone();
			// Height of widget
			let height = self.height.1 - self.height.0;

			/* If the Blocks is too short, but there is more text to be shown,
			add a new TextBlock to the tail. */
			if blocks.len() < height + self.scroll_offset
				&& self.file_length > height
				&& blocks.tail_block < blocks.max_blocks - 1
			{
				// Add new tail block
				blocks.push_tail(self, true).unwrap();
			}
			// Return the blocks
			blocks
		}

		fn get_lines_from_blocks(&self, blocks: Blocks) -> Vec<Line> {
			// Convert the blocks into one text vector
			let mut text: Vec<String> = Vec::new();
			// Iterate through the blocks that are currently loaded in
			for block in blocks.blocks_list {
				// Add all of the lines in these blocks into the `text` vector
				text.extend(block.content);
			}

			// Create a vector of Lines from the text
			text.into_par_iter()
				.enumerate()
				.map(|(idx, line)| {
					// If the line is empty, return a blank line
					if line.is_empty() {
						return Line::from(String::new());
					}
					self.parse_line(idx, &line)
				})
				.collect()
		}

		// Return the vector as a paragraph
		fn get_paragraph(&mut self) -> Paragraph {
			// Check that there are enough blocks loaded
			let blocks = self.check_blocks();
			// Set the editor blocks to this new blocks
			self.blocks = Some(blocks.clone());
			// The current line number in the blocks
			let line_num = self.get_line_num(self.cursor_position[1]) - blocks.starting_line_num;
			// Get the lines of the currently loaded blocks as a vector
			let mut lines = self.get_lines_from_blocks(blocks);

			// Highlight the line that the cursor is on
			lines[line_num] = lines[line_num].clone().style(
				Style::default()
					.fg(self.config.theme.line_highlight_fg_color)
					.bg(self.config.theme.line_highlight_bg_color),
			);

			// Return a paragraph from the lines
			Paragraph::new(Text::from(lines)).scroll((self.scroll_offset as u16, 0))
		}

		// Return a Vector of the line numbers that are displayed
		fn get_line_numbers(&self) -> Vec<usize> {
			let blocks = self.blocks.as_ref().unwrap();
			(blocks.starting_line_num + self.scroll_offset + 1
				..blocks.len() + blocks.starting_line_num + self.scroll_offset + 1)
				.collect()
		}

		// Return a Paragraph of the line numbers that are displayed
		pub fn get_line_numbers_paragraph(&self) -> Paragraph {
			// Construct a vector of line numbers
			let line_nums: Vec<Line> = self
				.get_line_numbers()
				.into_par_iter()
				.map(|num| Line::from(format!("{}", num)))
				.collect();

			Paragraph::new(Text::from(line_nums))
		}

		// Set the starting Position of the editing space cursor
		fn init_starting_position(&mut self, start: (usize, usize), width: usize, height: usize) {
			// Set the bounds of the block
			self.width = (start.0, start.0 + width);
			self.height = (start.1, start.1 + height);

			// Set the cursor to the beginning of the block
			self.cursor_position = [0, 0];

			// Flag that cursor has been initialized
			self.is_initialized = true;
		}

		// Initialize the file length variable
		fn init_file_length(&mut self) {
			// Get the lines of the file (with their newline chars)
			let lines: Vec<String> = read_to_string(&self.filename)
				.unwrap()
				.split_inclusive('\n')
				.map(String::from)
				.collect();
			// Count the number of lines in the file
			self.file_length = lines.par_iter().count();
			// If there is a blank final line, add one to the file length
			if lines[lines.len() - 1].ends_with('\n') {
				self.file_length += 1;
			}
		}

		// Create the first block when the editor is opened
		fn init_first_block(&mut self) -> Result<usize, Error> {
			// Create a block at block number 0
			let blocks = Blocks::new(self, 0)?;
			// Wrap this Blocks in an Option
			self.blocks = Some(blocks);
			// Return 0 to indicate success
			Ok(0)
		}

		// Initialize the editor
		fn init_editor(
			&mut self,
			start: (usize, usize),
			width: usize,
			height: usize,
		) -> Result<&str, Error> {
			// Initialize the starting position of the screen cursor
			self.init_starting_position(start, width, height);
			// Initialize the length of the file
			self.init_file_length();
			// Create the first block of text in Blocks
			self.init_first_block()?;
			// Return the string "Success" (arbitrary)
			Ok("Success")
		}

		// Render a blank ui if there are no TextBlocks in the editor Blocks
		fn render_empty_ui(&self, layout: Rc<[Rect]>, frame: &mut Frame) {
			// If the file is empty, render an empty line numbers widget
			frame.render_widget(
				Block::new()
					.fg(self.config.theme.app_fg)
					.bg(self.config.theme.app_bg)
					.borders(Borders::ALL),
				layout[0],
			);
			// If the file is empty, render an empty EditorSpace widget
			frame.render_widget(
				Block::new()
					.fg(self.config.theme.app_fg)
					.bg(self.config.theme.app_bg)
					.borders(Borders::ALL),
				layout[1],
			);
		}

		// Render the ui for the editor
		fn render_full_ui(&mut self, layout: Rc<[Rect]>, frame: &mut Frame) {
			// Clone the config for the editor
			let config = self.config.clone();

			// Render line numbers widget
			frame.render_widget(
				self.get_line_numbers_paragraph().block(
					Block::new()
						.fg(self.config.theme.app_fg)
						.bg(self.config.theme.app_bg)
						.borders(Borders::ALL),
				),
				layout[0],
			);

			// Render the editor widget
			frame.render_widget(
				self.get_paragraph().block(
					Block::new()
						.fg(config.theme.app_fg)
						.bg(config.theme.app_bg)
						.borders(Borders::ALL),
				),
				layout[1],
			);
		}

		// Render the widgets for the EditorSpace and its line numbers
		pub fn render_ui(&mut self, frame: &mut Frame, layout: Rc<[Rect]>) {
			// Only initialize this if it hasn't been already
			if !self.is_initialized {
				// Initialize the editor's cursor, file length, and first text block
				let _ = self.init_editor(
					(layout[1].x as usize, layout[1].y as usize),
					layout[1].width as usize,
					layout[1].height as usize,
				);
			}

			// Set the cursor position on screen
			frame.set_cursor(
				(self.cursor_position[0] + self.width.0 + 1) as u16,
				(self.cursor_position[1] + self.height.0 + 1) as u16,
			);

			// If the editor is empty, render an empty widget
			if self.is_empty() {
				// Render an empty
				self.render_empty_ui(layout, frame);
			// Otherwise, render the text blocks in a widget
			} else {
				// Render the editor widget
				self.render_full_ui(layout, frame);
			}
		}

		// Check if the editor is empty (no blocks loaded in)
		pub fn is_empty(&self) -> bool {
			self.blocks.as_ref().unwrap().blocks_list.is_empty()
		}

		// Delete a highlighted selection of text
		fn delete_selection(&mut self) {
			// Start point of the selection (as an immutable tuple)
			let start = (self.selection.start[0], self.selection.start[1]);
			// End point of the selection (as an immutable tuple)
			let end = (self.selection.end[0], self.selection.end[1]);

			// Clear the selection
			self.selection.is_empty = true;

			// If the cursor is at the beginning of the selection
			if (
				self.index_position,
				self.get_line_num(self.cursor_position[1]),
			) == start
			{
				// Move to the last line of the selection
				while self.get_line_num(self.cursor_position[1]) < end.1 {
					key_functions::down_arrow(self);
				}
				// Move the horizontal position to the end horizontal position
				key_functions::home_key(self, true);
				while self.index_position < end.0 {
					key_functions::right_arrow(self, true);
				}
			}
			// Backspace until at the beginning of the selection
			while self.index_position != start.0
				|| self.get_line_num(self.cursor_position[1]) != start.1
			{
				key_functions::backspace(self);
			}
		}

		// Get the key pressed
		pub fn handle_input(&mut self) {
			// Non-blocking read
			if event::poll(Duration::from_millis(300)).unwrap() {
				// Read input
				if let Event::Key(KeyEvent {
					code, modifiers, ..
				}) = event::read().unwrap()
				{
					// If no modifier key is pressed
					if modifiers.is_empty() {
						// Return the key
						match code {
							// If normal character, insert that character
							KeyCode::Char(code) => key_functions::char_key(self, code),
							// If Enter was pressed, insert newline
							KeyCode::Enter => key_functions::enter_key(self),
							// If tab was pressed, insert tab character
							KeyCode::Tab => key_functions::tab_key(self),
							// If backspace was pressed, remove the previous character
							KeyCode::Backspace => key_functions::backspace(self),
							// If delete was pressed, remove the next character
							KeyCode::Delete => key_functions::delete_key(self),
							// Left arrow moves cursor left
							KeyCode::Left => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Left arrow functionality
								key_functions::left_arrow(self, true);
							}
							// Right arrow moves cursor right
							KeyCode::Right => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Right arrow functionality
								key_functions::right_arrow(self, true);
							}
							// Up arrow move cursor up one line
							KeyCode::Up => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Up arrow functionality
								key_functions::up_arrow(self);
							}
							// Down arrow move cursor down one line
							KeyCode::Down => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Down arrow functionality
								key_functions::down_arrow(self);
							}
							// Home button moves to beginning of line
							KeyCode::Home => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Home key functionality
								key_functions::home_key(self, true);
							}
							// End button move to end of line
							KeyCode::End => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// End key functionality
								key_functions::end_key(self, true);
							}
							// The Page Up key moves up one `height` of the editor widget
							KeyCode::PageUp => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Move one page up
								key_functions::page_up(self);
							}
							// The Page Down key moves down one `height` of the editor widget
							KeyCode::PageDown => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Move one page down
								key_functions::page_down(self);
							}
							_ => (),
						}
					// If the Shift modifier is pressed
					} else if modifiers == KeyModifiers::SHIFT {
						match code {
							// Uppercase characters
							KeyCode::Char(code) => {
								key_functions::char_key(self, code.to_ascii_uppercase())
							}
							// Right arrow highlight text to the right
							KeyCode::Right => highlight_selection::highlight_right(self),
							// Left arrow highlight text to the left
							KeyCode::Left => highlight_selection::highlight_left(self),
							// Up arrow highlights text upwards
							KeyCode::Up => highlight_selection::highlight_up(self),
							// Down arrow highlights text downwards
							KeyCode::Down => highlight_selection::highlight_down(self),
							// End key highlights to end of line
							KeyCode::End => highlight_selection::highlight_end(self),
							// Home key highlights to beginning of line
							KeyCode::Home => highlight_selection::highlight_home(self),
							// Highlight one page up
							KeyCode::PageUp => highlight_selection::highlight_page_up(self),
							// Highlight one page down
							KeyCode::PageDown => highlight_selection::highlight_page_down(self),
							_ => (),
						}
					// If the Control modifier is pressed
					} else if modifiers == KeyModifiers::CONTROL {
						match code {
							// Save the frame to the file
							KeyCode::Char('s') => key_functions::save_key_combo(self, false, ""),
							// Break the loop to end the program
							KeyCode::Char('q') => self.break_loop = true,
							// Jump to the next word
							KeyCode::Right => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Jump to the next word
								key_functions::jump_right(self, false);
							}
							// Jump to the previous word
							KeyCode::Left => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Jump to the previous word
								key_functions::jump_left(self, false);
							}
							// Jump up 10 lines
							KeyCode::Up => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Jump up 10 lines
								key_functions::jump_up(self, false);
							}
							// Jump down 10 lines
							KeyCode::Down => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Jump down 10 lines
								key_functions::jump_down(self, false);
							}
							_ => (),
						}
					// If Control and Shift modifiers are both pressed
					} else if modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) {
						match code {
							// Highlight the entire unicode word to the right
							KeyCode::Right => key_functions::jump_right(self, true),
							// Highlight the entire unicode word to the left
							KeyCode::Left => key_functions::jump_left(self, true),
							// Highlight upwards 10 lines
							KeyCode::Up => key_functions::jump_up(self, true),
							// Highlight down 10 lines
							KeyCode::Down => key_functions::jump_down(self, true),
							_ => (),
						}
					}
				}
			}
		}
	}
}
