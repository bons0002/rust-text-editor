pub mod editor {

	use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
	use ratatui::{
		style::Style,
		text::{Line, Span, Text},
		widgets::Paragraph,
	};
	use std::{
		fs::{self, File},
		path::Path,
		time::Duration,
	};

	use config::config::Config;
	use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator};

	// Module containing all the functionality of each key. Called in handle_input
	mod key_functions;

	pub struct EditorSpace {
		// Name of file opened in current editor frame
		pub filename: String,
		// Text content of current frame
		pub content: Vec<String>,
		// Position of cursor on the screen
		pub cursor_pos: (usize, usize),
		// Position within the text (content vector)
		pub pos: (usize, usize),
		// Track if the starting cursor position has already been set
		pub start_cursor_set: bool,
		// TEMP bool to break the main loop
		pub break_loop: bool,
		// The starting and ending positions of the highlighted selection of text
		// (Negative values indicate no selection)
		pub selection: ((isize, isize), (isize, isize)),

		// Horizontal bounds of the editor block
		width: (usize, usize),
		// Vertical bounds of the editor block
		height: (usize, usize),
		// Sets the amount to scroll the text
		scroll_offset: (u16, u16),
	}

	impl EditorSpace {
		pub fn new(filename: String, config: &Config) -> Self {
			// Check if a file exists, if not create it
			if !Path::new(&filename).exists() {
				File::create(&filename).unwrap();
			}
			EditorSpace {
				// Read in the contents of the file
				content: Self::parse_file(&filename, config),
				filename,
				cursor_pos: (0, 0),
				pos: (0, 0),
				start_cursor_set: false,
				break_loop: false,
				selection: ((-1, -1), (-1, -1)),
				width: (0, 0),
				height: (0, 0),
				scroll_offset: (0, 0),
			}
		}

		// Parse the specified file to a vector of strings (each element representing a line) as a string for the raw data
		fn parse_file(filename: &String, config: &Config) -> Vec<String> {
			// Read the file to a string
			let content = fs::read_to_string(&filename).expect("Couldn't read file");

			// Vector containing text lines
			let mut result: Vec<String> = Vec::new();
			// String of spaces of length tab_width. Used to replace space indentation with tab indentation
			let tab_spaces = " ".repeat(config.tab_width);

			if !content.is_empty() {
				let line_split: Vec<&str> = content.split('\n').collect();
				// Split the string into lines
				let mut lines: Vec<String> = Vec::new();
				// Convert this split lines into a vector of strings
				lines.par_extend(
					line_split
						.into_par_iter() // Parallel iterator
						.map(|line| {
							// Operation
							String::from(line)
						}),
				);

				// Add each line (with space indentation replaced with tabs) to the vector of strings
				result.par_extend(
					lines
						.into_par_iter()
						.map(|line| line.replace(&tab_spaces, "\t")),
				);
				// Return the vector and raw string
				return result;
			}
			// If there is no text in the file being opened, push an empty line to the vector
			result.push(String::from(""));
			result
		}

		// Set the starting position of the editing space cursor
		pub fn set_starting_pos(&mut self, start: (usize, usize), width: usize, height: usize) {
			// Set the bounds of the block
			self.width = (start.0, start.0 + width);
			self.height = (start.1, start.1 + height);

			// Set the cursor to the beginning of the block
			self.pos = (0, 0);
			self.cursor_pos = (self.width.0 + 1, self.height.0 + 1);

			// Flag that cursor has been initialized
			self.start_cursor_set = true;
		}

		// Return the vector as a paragraph
		pub fn get_paragraph(&self, config: &Config) -> Paragraph {
			// Vector to store the lines
			let mut lines: Vec<Line> = Vec::new();
			let content = &self.content;

			// Create a vector of Lines (in parallel)
			lines.par_extend(content.into_par_iter().enumerate().map(|(idx, part)| {
				self.parse_line(config, idx, part)
			}));

			// Highlight the line that the cursor is on
			lines[self.pos.1] = lines[self.pos.1].clone().style(
				Style::default()
					.fg(config.theme.line_highlight_fg_color)
					.bg(config.theme.line_highlight_bg_color),
			);

			// Return a paragraph from the lines
			Paragraph::new(Text::from(lines)).scroll(self.scroll_offset)
		}

		fn parse_line(&self, config: &Config, idx:usize, line: &String) -> Line {
			// Split the line into individual words
			let characters: Vec<char> = line.chars().collect();
			let mut spans: Vec<Span> = Vec::new();
			// Iterate through each character on the line
			spans.par_extend(characters.into_par_iter().enumerate().map(|(loc, character)| {
				match character {
					'\t' => {
						// Start tab with a vertical line
						let mut tab_char = String::from("\u{023D0}");
						// Iterator to create a string of tab_width - 1 number of spaces
						tab_char.push_str(&" ".repeat(config.tab_width - 1));

						self.highlight_char(config, idx, loc, tab_char)
					}
					_ => {
						self.highlight_char(config, idx, loc, String::from(character))
					}
				}
			}));

			Line::from(spans)
		}

		// Highlight a specific character on the line within the highlighting selection
		fn highlight_char(&self, config: &Config, idx: usize, loc: usize, character: String) -> Span {
			// If only one line
			if (idx as isize) == self.selection.0.1 && self.selection.0.1 == self.selection.1.1 {
				// If within selection, highlight character
				if (loc as isize) >= self.selection.0.0 && (loc as isize) < self.selection.1.0 {
					Span::from(character).style(Style::default().bg(config.theme.selection_highlight))
				} else {
					Span::from(character)
				}
			// If on first line (and there are multiple lines in selection)
			} else if (idx as isize) == self.selection.0.1 {
				// Highlight all characters on the line after the cursor
				if (loc as isize) >= self.selection.0.0 {
					Span::from(character).style(Style::default().bg(config.theme.selection_highlight))
				} else {
					Span::from(character)
				}
			// If on last line (and there are multiple lines in selection)
			} else if (idx as isize) == self.selection.1.1 {
				// Highlight all characters on the line before the cursor
				if (loc as isize) < self.selection.1.0 {
					Span::from(character).style(Style::default().bg(config.theme.selection_highlight))
				} else {
					Span::from(character)
				}
			// If between first and last line in multine selection
			} else if (idx as isize) > self.selection.0.1 && (idx as isize) < self.selection.1.1 {
				Span::from(character).style(Style::default().bg(config.theme.selection_highlight))
			// If not in selection
			} else {
				Span::from(character)
			}
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
							KeyCode::Char(code) => {
								key_functions::char_key(self, code);
							}
							// If Enter was pressed, insert newline
							KeyCode::Enter => {
								key_functions::enter_key(self);
							}
							// If tab was pressed, insert tab character
							KeyCode::Tab => {
								key_functions::tab_key(self, config);
							}
							// If backspace was pressed, remove the previous character
							KeyCode::Backspace => {
								key_functions::backspace(self, config);
							}
							// If delete was pressed, remove the next character
							KeyCode::Delete => {
								key_functions::delete_key(self);
							}
							// Left arrow moves cursor left
							KeyCode::Left => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
								// Left arrow functionality
								key_functions::left_arrow(self, config);
							}
							// Right arrow moves cursor right
							KeyCode::Right => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
								// Right arrow functionality
								key_functions::right_arrow(self, config);
							}
							// Up arrow move cursor up one line
							KeyCode::Up => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
								// Up arrow functionality
								key_functions::up_arrow(self, config);
							}
							// Down arrow move cursor down one line
							KeyCode::Down => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
								// Down arrow functionality
								key_functions::down_arrow(self, config);
							}
							// Home button moves to beginning of line
							KeyCode::Home => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
								// Home key functionality
								key_functions::home_key(self);
							}
							// End button move to end of line
							KeyCode::End => {
								// Clear the highlighted selection of text
								self.selection = ((-1, -1), (-1, -1));
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
								key_functions::char_key(self, code.to_ascii_uppercase());
							}
							// Right arrow highlight text
							KeyCode::Right => {
								key_functions::highlight_right(self, config);
							}
							// Left arrow highlight text
							KeyCode::Left => {
								key_functions::highlight_left(self, config);
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
							KeyCode::Char('s') => {
								key_functions::save_key_combo(self);
							}
							// Break the loop to end the program
							KeyCode::Char('c') => {
								self.break_loop = true;
							}
							_ => (),
						}
					}

					_ => (),
				}
			}
		}
	}
}

