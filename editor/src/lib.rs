pub mod editor {

	use std::{
		fs::{File, OpenOptions},
		io::{self, BufRead, Error},
		path::Path,
		time::Duration,
	};

	use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
	use ratatui::{
		style::Style,
		text::{Line, Span, Text},
		widgets::Paragraph,
	};
	use rayon::iter::{
		IndexedParallelIterator, IntoParallelIterator, ParallelBridge, ParallelIterator,
	};

	use config::config::Config;

	mod blocks;
	use blocks::Blocks;

	// Module containing all the functionality of each key. Called in handle_input
	mod key_functions;
	use key_functions::highlight_selection::Selection;
	use unicode_segmentation::UnicodeSegmentation;

	// Testing module found at crate/src/editor/tests.rs
	#[cfg(test)]
	mod tests;

	pub struct EditorSpace {
		// Text block of current frame
		pub blocks: Option<Blocks>,
		// Flag for whether to break rendering loop in main app
		pub break_loop: bool,
		// The config of the editor
		pub config: Config,
		// Position of cursor on the screen (and in the text)
		pub cursor_position: [usize; 2],
		// Name of file opened in current editor frame
		pub filename: String,
		// The file that is open
		file: File,
		// The number of lines in the entire file
		pub file_length: usize,
		// Flag for if the file has been initialize
		pub has_file: bool,
		// Vertical bounds of the editor block
		pub height: (usize, usize),
		// Position used to access indices within graphemes vector
		index_position: usize,
		// Horizontal bounds of the editor block
		pub width: (usize, usize),
		// Sets the amount to scroll the text
		scroll_offset: usize,
		// Structure keeping track of the highlighted selection of text
		selection: Selection,
		// Track if the starting cursor position has already been set
		pub start_cursor_set: bool,
		// Position on the current line of text
		text_position: usize,
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
				filename,
				file,
				file_length: 0,
				has_file: false,
				height: (0, 0),
				index_position: 0,
				width: (0, 0),
				scroll_offset: 0,
				selection: Selection::new(),
				start_cursor_set: false,
				text_position: 0,
			}
		}

		// Set the starting Position of the editing space cursor
		fn init_starting_position(&mut self, start: (usize, usize), width: usize, height: usize) {
			// Set the bounds of the block
			self.width = (start.0, start.0 + width);
			self.height = (start.1, start.1 + height);

			// Set the cursor to the beginning of the block
			self.cursor_position = [0, 0];

			// Flag that cursor has been initialized
			self.start_cursor_set = true;
		}

		// Initialize the file length variable
		fn init_file_length(&mut self) -> Result<usize, Error> {
			// Open the file
			let file = File::open(&self.filename)?;
			// Count the lines of the file (in parallel)
			self.file_length = io::BufReader::new(file).lines().par_bridge().count();
			// The file has been initialized
			self.has_file = true;
			// Return the file length
			Ok(self.file_length)
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
		pub fn init_editor(
			&mut self,
			start: (usize, usize),
			width: usize,
			height: usize,
		) -> Result<&str, Error> {
			// Initialize the starting position of the screen cursor
			self.init_starting_position(start, width, height);
			// Initialize the length of the file
			self.init_file_length()?;
			// Create the first block of text in Blocks
			self.init_first_block()?;
			// Return the string "Success" (arbitrary)
			Ok("Success")
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
		pub fn get_paragraph(&mut self) -> Paragraph {
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
				self.text_position,
				self.get_line_num(self.cursor_position[1]),
			) == start
			{
				// Move to the last line of the selection
				while self.get_line_num(self.cursor_position[1]) != end.1 {
					key_functions::down_arrow(self);
				}
				// Move the horizontal position to the end horizontal position
				key_functions::home_key(self);
				while self.text_position < end.0 {
					key_functions::right_arrow(self);
				}
			}
			// Backspace until at the beginning of the selection
			while self.text_position != start.0
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
				match event::read().unwrap() {
					// Return the character if only a key (without moodifier key) is pressed
					Event::Key(KeyEvent {
						code,
						modifiers: KeyModifiers::NONE,
						..
					}) => {
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
								key_functions::left_arrow(self);
							}
							// Right arrow moves cursor right
							KeyCode::Right => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Right arrow functionality
								key_functions::right_arrow(self);
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
								key_functions::home_key(self);
							}
							// End button move to end of line
							KeyCode::End => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// End key functionality
								key_functions::end_key(self);
							}
							_ => (),
						}
					}

					// Shift modifier key
					Event::Key(KeyEvent {
						code,
						modifiers: KeyModifiers::SHIFT,
						..
					}) => {
						match code {
							// Uppercase characters
							KeyCode::Char(code) => {
								key_functions::char_key(self, code.to_ascii_uppercase())
							}
							// Right arrow highlight text to the right
							KeyCode::Right => {
								key_functions::highlight_selection::highlight_right(self)
							}
							// Left arrow highlight text to the left
							KeyCode::Left => {
								key_functions::highlight_selection::highlight_left(self)
							}
							// Up arrow highlights text upwards
							KeyCode::Up => key_functions::highlight_selection::highlight_up(self),
							// Down arrow highlights text downwards
							KeyCode::Down => {
								key_functions::highlight_selection::highlight_down(self)
							}
							// End key highlights to end of line
							KeyCode::End => key_functions::highlight_selection::highlight_end(self),
							// Home key highlights to beginning of line
							KeyCode::Home => {
								key_functions::highlight_selection::highlight_home(self)
							}
							_ => (),
						}
					}

					// Control modified keys
					Event::Key(KeyEvent {
						code,
						modifiers: KeyModifiers::CONTROL,
						..
					}) => {
						match code {
							// Save the frame to the file
							KeyCode::Char('s') => key_functions::save_key_combo(self, false, ""),
							// Break the loop to end the program
							KeyCode::Char('q') => self.break_loop = true,
							_ => (),
						}
					}

					_ => (),
				}
			}
		}
	}
}
