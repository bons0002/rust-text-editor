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
		IndexedParallelIterator, IntoParallelIterator, ParallelBridge, ParallelExtend,
		ParallelIterator,
	};

	use config::config::Config;

	mod blocks;
	use blocks::Blocks;

	// Module containing all the functionality of each key. Called in handle_input
	mod key_functions;
	use key_functions::highlight_selection::Selection;
	use unicode_segmentation::UnicodeSegmentation;

	pub struct EditorSpace {
		// Text block of current frame
		pub blocks: Option<Blocks>,
		// Flag for whether to break rendering loop in main app
		pub break_loop: bool,
		// Position on the current line of text
		pub text_position: usize,
		// Position of cursor on the screen (and in the text)
		pub cursor_position: [usize; 2],
		// Name of file opened in current editor frame
		pub filename: String,
		// The file that is open
		pub file: File,
		// The length of the entire file that is being openned
		pub file_length: usize,
		// Vertical bounds of the editor block
		pub height: (usize, usize),
		// Horizontal bounds of the editor block
		pub width: (usize, usize),
		// Sets the amount to scroll the text
		scroll_offset: usize,
		// Structure keeping track of the highlighted selection of text
		selection: Selection,
		// Track if the starting cursor position has already been set
		pub start_cursor_set: bool,
	}

	impl EditorSpace {
		pub fn new(filename: String) -> Self {
			// Check if a file exists, if not create it
			if !Path::new(&filename).exists() {
				File::create(&filename).unwrap();
			}
			// Open the file in read-write mode
			let file = match OpenOptions::new().read(true).write(true).open(&filename) {
				Ok(file) => file,
				Err(err) => panic!("{}", err),
			};
			// Construct an EditorSpace
			EditorSpace {
				blocks: None,
				break_loop: false,
				text_position: 0,
				cursor_position: [0, 0],
				filename,
				file,
				file_length: 0,
				height: (0, 0),
				width: (0, 0),
				scroll_offset: 0,
				selection: Selection::new(),
				start_cursor_set: false,
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
			// Return the file length
			Ok(self.file_length)
		}

		// Create the first block when the editor is opened
		pub fn init_first_block(&mut self) -> Result<usize, Error> {
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
			// Initialize the file length
			self.init_file_length()?;
			// Create the first block of text in Blocks
			self.init_first_block()?;
			// Return the string "Success" (arbitrary)
			Ok("Success")
		}

		// Highlight a specific character on the line within the highlighting selection
		fn highlight_char(
			&self,
			config: &Config,
			idx: usize,
			loc: usize,
			character: String,
		) -> Span {
			if !self.selection.is_empty {
				// If only one line
				if idx == self.selection.start[1]
					&& self.selection.start[1] == self.selection.end[1]
				{
					// If within selection, highlight character
					if loc >= self.selection.start[0] && loc < self.selection.end[0] {
						Span::from(character)
							.style(Style::default().bg(config.theme.selection_highlight))
					} else {
						Span::from(character)
					}
				// If on first line (and there are multiple lines in selection)
				} else if idx == self.selection.start[1] {
					// Highlight all characters on the line after the cursor
					if loc >= self.selection.start[0] {
						Span::from(character)
							.style(Style::default().bg(config.theme.selection_highlight))
					} else {
						Span::from(character)
					}
				// If on last line (and there are multiple lines in selection)
				} else if idx == self.selection.end[1] {
					// Highlight all characters on the line before the cursor
					if loc < self.selection.end[0] {
						Span::from(character)
							.style(Style::default().bg(config.theme.selection_highlight))
					} else {
						Span::from(character)
					}
				// If between first and last line in multine selection
				} else if idx > self.selection.start[1] && idx < self.selection.end[1] {
					Span::from(character)
						.style(Style::default().bg(config.theme.selection_highlight))
				// If not in selection
				} else {
					Span::from(character)
				}
			} else {
				Span::from(character)
			}
		}

		// Create a Line struct from the given String line
		fn parse_line(&self, config: &Config, idx: usize, line: &str) -> Line {
			// Split the line into individual words
			let characters: Vec<&str> = line.graphemes(true).collect();
			let mut spans: Vec<Span> = Vec::new();
			// Iterate through each character on the line
			spans.par_extend(
				characters
					.into_par_iter()
					.enumerate()
					.map(|(loc, character)| {
						match character {
							"\t" => {
								// Start tab with a vertical line
								let mut tab_char = String::from("\u{023D0}");
								// Iterator to create a string of tab_width - 1 number of spaces
								tab_char.push_str(&" ".repeat(config.tab_width - 1));
								// Highlight this spaces representation of a tab
								self.highlight_char(config, idx, loc, tab_char)
							}
							_ => {
								// Highlight this (non-tab) character
								self.highlight_char(config, idx, loc, String::from(character))
							}
						}
					}),
			);
			spans.push(Span::from("\n"));

			// Return the line
			Line::from(spans)
		}

		// Return the vector as a paragraph
		pub fn get_paragraph(&self, config: &Config) -> Paragraph {
			// Clone the blocks of text
			let blocks = self.blocks.clone();

			// Convert the blocks into one text vector
			let mut text: Vec<String> = Vec::new();

			// Iterate through the blocks that are currently loaded in
			for block in blocks.unwrap().blocks_list {
				// Add all of the lines in these blocks into the `text` vector
				block.content.into_par_iter().collect_into_vec(&mut text);
			}

			// Create a vector of Lines from the text
			let mut lines: Vec<Line> = text
				.into_par_iter()
				.enumerate()
				.map(|(idx, line)| self.parse_line(config, idx, &line))
				.collect();

			// The current line number in the text
			let line_num = self.get_line_num() - self.blocks.as_ref().unwrap().starting_line_num;

			// Highlight the line that the cursor is on
			lines[line_num] = lines[line_num].clone().style(
				Style::default()
					.fg(config.theme.line_highlight_fg_color)
					.bg(config.theme.line_highlight_bg_color),
			);

			// Return a paragraph from the lines
			Paragraph::new(Text::from(lines)).scroll((self.scroll_offset as u16, 0))
		}

		// TODO: UPDATE FILE LENGTH WHEN DELETING MULTILINE SELECTION
		// Delete the highlighted selection of text
		/*
		fn delete_selection(&mut self) {
			// Get everything before the selected text on the beginning line
			let mut before_selection =
				String::from(&self.block[self.selection.start[1]][..self.selection.start[0]]);
			// Get everything after the selected text on the ending line
			let after_selection =
				String::from(&self.block[self.selection.end[1]][self.selection.end[0]..]);

			let idx = self.selection.start[1] + 1;
			// Remove the middle lines of the selection
			for _i in (self.selection.start[1] + 1)..(self.selection.end[1] + 1) {
				self.block.remove(idx);
			}

			// Concat the block after the selection to before the selection
			before_selection.push_str(after_selection.as_str());
			// Set the line to the new string
			self.block[self.selection.start[1]] = before_selection;

			// Reset the selection
			self.selection.is_empty = true;

			// Move cursor back to original Position
			if self.text_position == self.selection.end {
				self.text_position = [
					self.selection.original_text_position.0,
					self.selection.original_text_position.1,
				];
				self.cursor_position = [
					self.selection.original_cursor_position.0,
					self.selection.original_cursor_position.1,
				];
			}
		}
		*/

		// Get the current line number
		pub fn get_line_num(&self) -> usize {
			self.cursor_position[1] + self.scroll_offset
		}

		// Get the key pressed
		pub fn handle_input(&mut self, config: &Config) {
			// Non-blocking read
			if event::poll(Duration::from_millis(50)).unwrap() {
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
							KeyCode::Enter => key_functions::enter_key(self, config),
							// If tab was pressed, insert tab character
							KeyCode::Tab => key_functions::tab_key(self, config),
							// If backspace was pressed, remove the previous character
							KeyCode::Backspace => key_functions::backspace(self, config),
							// If delete was pressed, remove the next character
							KeyCode::Delete => key_functions::delete_key(self),
							// Left arrow moves cursor left
							KeyCode::Left => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Left arrow functionality
								key_functions::left_arrow(self, config);
							}
							// Right arrow moves cursor right
							KeyCode::Right => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Right arrow functionality
								key_functions::right_arrow(self, config);
							}
							// Up arrow move cursor up one line
							KeyCode::Up => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Up arrow functionality
								key_functions::up_arrow(self, config);
							}
							// Down arrow move cursor down one line
							KeyCode::Down => {
								// Clear the highlighted selection of text
								self.selection.is_empty = true;
								// Down arrow functionality
								key_functions::down_arrow(self, config);
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
								key_functions::end_key(self, config);
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
								key_functions::highlight_selection::highlight_right(self, config)
							}
							// Left arrow highlight text to the left
							KeyCode::Left => {
								key_functions::highlight_selection::highlight_left(self, config)
							}
							// Up arrow highlights text upwards
							KeyCode::Up => {
								key_functions::highlight_selection::highlight_up(self, config)
							}
							// Down arrow highlights text downwards
							KeyCode::Down => {
								key_functions::highlight_selection::highlight_down(self, config)
							}
							// End key highlights to end of line
							KeyCode::End => {
								key_functions::highlight_selection::highlight_end(self, config)
							}
							// Home key highlights to beginning of line
							KeyCode::Home => {
								key_functions::highlight_selection::highlight_home(self, config)
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
							KeyCode::Char('s') => key_functions::save_key_combo(),
							// Break the loop to end the program
							KeyCode::Char('c') => self.break_loop = true,
							_ => (),
						}
					}

					_ => (),
				}
			}
		}
	}
}
