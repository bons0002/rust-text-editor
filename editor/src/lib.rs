/// This module controls the editor space.
/// The editor space is where the bulk of text editing takes place.
pub mod editor {

	use std::{
		fs::{read_to_string, File, OpenOptions},
		io::Error,
		path::Path,
		rc::Rc,
		time::Duration,
	};

	use cli_clipboard::{ClipboardContext, ClipboardProvider};
	use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
	use ratatui::{
		layout::Rect,
		style::Style,
		text::{Line, Span, Text},
		widgets::{Block, BorderType, Borders, Paragraph},
		Frame,
	};
	use rayon::iter::{
		IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelExtend,
		ParallelIterator,
	};
	use unicode_segmentation::UnicodeSegmentation;

	use blocks::Blocks;
	use config::config::Config;
	use key_functions::{
		copy_paste, editing_keys,
		highlight_keys::{self, selection::Selection},
		navigation_keys, save_key,
	};
	use unredo_stack::{stack_choice::StackChoice, UnRedoStack, UnRedoState};

	/// Module containing the `Blocks` structure.
	/// This `Blocks` structure loads in multiple text blocks at once.
	mod blocks;
	/// Subroutines for the `handle_input` function.
	/// The `handle_input` function takes keyboard input and performs an action.
	mod input_handlers;
	/// Module containing all the logic of each key and key combination.
	mod key_functions;
	/// Module containing the `UnRedoStack` structure which handles
	/// both undo and redo states for the editor.
	mod unredo_stack;
	// Testing module found at crate/src/editor/tests.rs
	#[cfg(test)]
	mod tests;

	// 300 millisecond pollrate for reading terminal events
	const POLLRATE: u64 = 300;

	/// The struct for the editing space of the app.
	/// Each `EditorSpace` opens its own file and handles the IO for editing.
	pub struct EditorSpace {
		// Object containing multiple text blocks
		blocks: Option<Blocks>,
		// The clipboard to copy from and paste to
		clipboard: Option<ClipboardContext>,
		/// The config of the editor. Currently, it only sets the tab width.
		pub config: Config,
		// Position of cursor on the screen
		cursor_position: [usize; 2],
		// The file that is open
		file: File,
		// Name of file opened in current editor space
		filename: String,
		// The number of lines in the entire file
		file_length: usize,
		// The height of the widget
		height: usize,
		// Position used to access indices within graphemes vectors
		is_initialized: bool,
		// Used to scroll the text on screen (and calculate line number)
		scroll_offset: usize,
		// Structure keeping track of the highlighted selection of text
		selection: Selection,
		// Used to store the horizontal position in the text
		stored_position: usize,
		// Actual position on the current line of text
		text_position: usize,
		// Stack for storing undo/redo states
		unredo_stack: UnRedoStack,
		// Horizontal bounds of the editor block
		widget_horz_bounds: (usize, usize),
		// Vertical bounds of the editor widget
		widget_vert_bounds: (usize, usize),
		// The width of the widget
		width: usize,
	}

	impl EditorSpace {
		/// Create a new EditorSpace
		pub fn new(filename: String, config: Config) -> Self {
			// Open (and create if necessary) the given file
			let file = Self::open_file(&filename);
			// Create a clipboard
			let clipboard = match ClipboardContext::new() {
				Ok(clip) => Some(clip),
				Err(_) => None,
			};
			// Construct an EditorSpace
			EditorSpace {
				blocks: None,
				clipboard,
				config,
				cursor_position: [0, 0],
				file,
				filename,
				file_length: 0,
				height: 0,
				is_initialized: false,
				scroll_offset: 0,
				selection: Selection::new(),
				stored_position: 0,
				text_position: 0,
				unredo_stack: UnRedoStack::new(),
				widget_horz_bounds: (0, 0),
				widget_vert_bounds: (0, 0),
				width: 0,
			}
		}

		/// Get the key pressed and perform an action
		pub fn handle_input(&mut self, break_loop: &mut bool) {
			// Non-blocking read
			if event::poll(Duration::from_millis(POLLRATE)).unwrap() {
				// Read input
				if let Event::Key(KeyEvent {
					code,
					modifiers,
					kind: KeyEventKind::Press,
					..
				}) = event::read().unwrap()
				{
					// If no modifier key is pressed
					if modifiers.is_empty() {
						input_handlers::no_modifiers(self, code);
					// If the Shift modifier is pressed
					} else if modifiers == KeyModifiers::SHIFT {
						input_handlers::shift_modifier(self, code);
					// If the Control modifier is pressed
					} else if modifiers == KeyModifiers::CONTROL {
						input_handlers::control_modifier(self, code, break_loop);
					// If Control and Shift modifiers are both pressed
					} else if modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) {
						input_handlers::control_and_shift_modifiers(self, code);
					}
				}
			}
		}

		/// Render the widgets for the EditorSpace and its line numbers
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
				(self.cursor_position[0] + self.widget_horz_bounds.0 + 1) as u16,
				(self.cursor_position[1] + self.widget_vert_bounds.0 + 1) as u16,
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

		// Get the current line number for the given position
		fn get_line_num(&self, position: usize) -> usize {
			position + self.scroll_offset + self.blocks.as_ref().unwrap().starting_line_num
		}

		// Return the vector as a paragraph
		fn get_paragraph(&mut self) -> Paragraph {
			// Clone the blocks
			let mut blocks = self.blocks.as_ref().unwrap().clone();
			// Check the blocks are valid
			blocks.check_blocks(self);
			// Set the editor blocks to this new blocks
			self.blocks = Some(blocks.clone());
			// The current line number in the blocks
			let line_num = self.get_line_num(self.cursor_position[1]) - blocks.starting_line_num;
			// Get the lines of the currently loaded blocks as a vector
			let mut lines = self.get_lines_from_blocks(blocks, line_num);

			// Highlight the line that the cursor is on
			if let Some(line) = lines.get(line_num) {
				lines[line_num] = line.clone().style(
					Style::default()
						.fg(self.config.theme.line_highlight_fg_color)
						.bg(self.config.theme.line_highlight_bg_color),
				);
			}

			// Return a paragraph from the lines
			Paragraph::new(Text::from(lines)).scroll((self.scroll_offset as u16, 0))
		}

		// Return a Paragraph of the line numbers that are displayed
		fn get_line_numbers_paragraph(&self) -> Paragraph {
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

			// Clone the blocks
			let mut blocks = self.blocks.as_ref().unwrap().clone();
			// Create the remaining line after deleting the selection
			let remaining_line = Self::construct_remaining_line(&mut blocks, start, end);
			// Update the first line of the selection
			blocks.update_some_line(remaining_line, start.1).unwrap();

			// Loop to delete the selection
			for _i in start.1..end.1 {
				// Ensure that the blocks are valid
				blocks.check_blocks(self);
				// Delete the next line
				let _ = blocks.delete_line(start.1 + 1);
				// Reduce the file length
				self.file_length -= 1;
			}
			// Set the editor blocks
			self.blocks = Some(blocks);

			// Reset the position of the cursor (and the scroll offset)
			self.reset_cursor(end);
			// Clear the selection
			self.selection.is_empty = true;
		}

		// Set the starting Position of the editing space cursor
		fn init_starting_position(&mut self, start: (usize, usize), width: usize, height: usize) {
			// Set the bounds of the block
			self.widget_horz_bounds = (start.0, start.0 + width);
			self.widget_vert_bounds = (start.1, start.1 + height);
			/* Track the height of the widget (subtract three because there is a top and
			bottom boundary plus an extra line that isn't included in the height) */
			self.height = self.widget_vert_bounds.1 - self.widget_vert_bounds.0 - 3;
			/* Track the width of the widget (subtract 2 for the side borders of the
			widget) */
			self.width = self.widget_horz_bounds.1 - self.widget_horz_bounds.0 - 2;

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

			let default = String::from("\n");
			// If there is a blank final line, add one to the file length
			if lines.last().unwrap_or(&default).ends_with('\n') {
				self.file_length += 1;
			}
		}

		// Create the first block when the editor is opened
		fn init_first_block(&mut self) -> Result<usize, Error> {
			// Create a block at block number 0
			let blocks = Blocks::new(self, 0, 0)?;
			// Wrap this Blocks in an Option
			self.blocks = Some(blocks);
			// Return 0 to indicate success
			Ok(0)
		}

		// Render a blank ui if there are no TextBlocks in the editor Blocks
		fn render_empty_ui(&self, layout: Rc<[Rect]>, frame: &mut Frame) {
			// If the file is empty, render an empty line numbers widget
			frame.render_widget(Block::new().borders(Borders::ALL), layout[0]);
			// If the file is empty, render an empty EditorSpace widget
			frame.render_widget(Block::new().borders(Borders::ALL), layout[1]);
		}

		// Render the ui for the editor
		fn render_full_ui(&mut self, layout: Rc<[Rect]>, frame: &mut Frame) {
			// Render line numbers widget
			frame.render_widget(
				self.get_line_numbers_paragraph().block(
					Block::new()
						.borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
						.border_type(BorderType::Thick),
				),
				layout[0],
			);

			// Render the editor widget
			frame.render_widget(
				self.get_paragraph().block(
					Block::new()
						.borders(Borders::ALL)
						.border_type(BorderType::Thick),
				),
				layout[1],
			);
		}

		// Get the lines of text from the Blocks content
		fn get_lines_from_blocks(&self, blocks: Blocks, line_num: usize) -> Vec<Line> {
			// Convert the blocks into one text vector
			let mut text: Vec<String> = Vec::new();
			// Iterate through the blocks that are currently loaded in
			for block in blocks.blocks_list {
				// Add all of the lines in these blocks into the `text` vector
				text.par_extend(block.content);
			}

			// Create a vector of Lines from the text
			text.into_par_iter()
				.enumerate()
				.map(|(idx, line)| {
					// If the line is empty, return a blank line
					if line.is_empty() {
						// Blank space to add to put on the line (for line highlighting)
						let blank_space = String::from(&" ".repeat(self.width));
						// Return the blank line
						return Line::from(blank_space);
					// The line the cursor is on
					} else if idx == line_num {
						// Length of the text on the line
						let len = line.graphemes(true).count();
						// Only create blank space if there is room
						let blank_space = match len < self.width {
							true => &" ".repeat(self.width - len),
							false => &String::new(),
						};
						// Return the line
						return self.parse_line(idx, &(line + blank_space));
					}
					self.parse_line(idx, &line)
				})
				.collect()
		}

		// Create a Line struct from the given String line
		fn parse_line(&self, idx: usize, line: &str) -> Line {
			// Top line of the widget
			let top_line = self.scroll_offset;
			// The bottom line of the widget
			let bottom_line = self.height + self.scroll_offset;
			// Start tab with a vertical line
			let mut tab_char = String::from("\u{2502}");
			// Iterator to create a string of tab_width - 1 number of spaces
			tab_char.push_str(&" ".repeat(self.config.tab_width - 1));

			// Only highlight if selection isn't empty (and its within the widget's bounds)
			if !self.selection.is_empty && idx >= top_line && idx <= bottom_line {
				// Highlight characters
				return self.highlight_line(idx, line);
			}

			Line::from(String::from(line).replace('\t', &tab_char))
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
				.grapheme_indices(true)
				.map(|(loc, character)| {
					// Highlight the grapheme
					self.highlight_grapheme(idx, loc, character, &tab_char, start_line, end_line)
				})
				.collect();

			Line::from(graphemes)
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
				self.highlight_one_line(loc, String::from(character).replace('\t', tab_char))
			// If on first line (and there are multiple lines in selection)
			} else if idx == start_line {
				// Highlight character
				self.highlight_first_line(loc, String::from(character).replace('\t', tab_char))
			// If on last line (and there are multiple lines in selection)
			} else if idx == end_line {
				// Highlight character
				self.highlight_last_line(loc, String::from(character).replace('\t', tab_char))
			// If between first and last line in multine selection
			} else if idx > start_line && idx < end_line {
				Span::from(String::from(character).replace('\t', tab_char))
					.style(Style::default().bg(self.config.theme.selection_highlight))
			// If not in selection
			} else {
				Span::from(String::from(character).replace('\t', tab_char))
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

		// Return a Vector of the line numbers that are displayed
		fn get_line_numbers(&self) -> Vec<usize> {
			let blocks = self.blocks.as_ref().unwrap();
			(blocks.starting_line_num + self.scroll_offset + 1
				..blocks.len() + blocks.starting_line_num + self.scroll_offset + 1)
				.collect()
		}

		// Check if the editor is empty (no blocks loaded in)
		fn is_empty(&self) -> bool {
			self.blocks.as_ref().unwrap().blocks_list.is_empty()
		}

		// Create the remaining line after deleting a selection of text
		fn construct_remaining_line(
			blocks: &mut Blocks,
			start: (usize, usize),
			end: (usize, usize),
		) -> String {
			// Get the text on the first line of the selection before the cursor
			let before_selection = blocks
				.get_some_line(start.1)
				.unwrap()
				.grapheme_indices(true)
				.filter_map(|(idx, graph)| {
					if idx < start.0 {
						Some(String::from(graph))
					} else {
						None
					}
				})
				.collect::<String>();

			// Get the text on the last line of the selection after the cursor
			let after_selection = blocks
				.get_some_line(end.1)
				.unwrap()
				.grapheme_indices(true)
				.filter_map(|(idx, graph)| {
					if idx >= end.0 {
						Some(String::from(graph))
					} else {
						None
					}
				})
				.collect::<String>();

			before_selection + after_selection.as_str()
		}

		/*  Reset the position of the cursor after a selection deletion (if need be).
		Also, reset the scroll offset (if need be). */
		fn reset_cursor(&mut self, end: (usize, usize)) {
			// Only reset cursor and scroll offset if at the end of the selection
			if self.get_line_num(self.cursor_position[1]) == end.1 && self.text_position == end.0 {
				// Reset scroll offset
				self.scroll_offset = self.selection.original_scroll_offset;
				// Reset the Blocks tracked location
				self.blocks.as_mut().unwrap().curr_position =
					self.selection.original_tracked_location;
				// Reset the cursor's line number
				self.cursor_position[1] = self.selection.original_cursor_position.1;
				// Move to the beginning of the line
				navigation_keys::home_key(self, true);
				// Move to the correct horizontal position on the line
				while self.cursor_position[0] < self.selection.original_cursor_position.0
					&& key_functions::check_cursor_end_line(self)
				{
					navigation_keys::right_arrow(self, true);
				}
			}
		}

		// Get the current state of the editor (to be added to the unredo stack)
		fn get_unredo_state(&self) -> UnRedoState {
			(
				self.stored_position,
				self.text_position,
				self.cursor_position,
				self.scroll_offset,
				self.blocks.as_ref().unwrap().clone(),
				self.selection.clone(),
			)
		}
	}
}
