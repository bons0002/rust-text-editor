use super::{
	check_cursor_begin_line, check_cursor_end_line, highlight_selection, EditorSpace,
	GraphemeCursor, UnicodeSegmentation, UnicodeWidthStr,
};

// Home key functionality
pub fn home_key(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Move to beginning of line
	editor.text_position = 0;
	editor.cursor_position[0] = 0;
	// Set the stored cursor to the beginning of the line if the flag is set
	if will_store_cursor {
		editor.stored_position = 0;
	}
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	while check_cursor_end_line(editor, line_num) {
		right_arrow(editor, will_store_cursor);
	}
}

// Logic for moving left for a non-tab char
fn left_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str, will_store_cursor: bool) {
	// Create a cursor to navigate the grapheme cluster
	let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
	// Get the previous location in the text
	let loc = cursor.prev_boundary(line, 0);
	// Set the text position
	let loc = match loc {
		Ok(num) => match num {
			Some(num) => num,
			None => panic!(
				/* Return the source file name, line number error occurred in this source file,
				and line_num that this grapheme boundary exists on. */
				"{}::left_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
				file!(),
				line!(),
				line_num
			),
		},
		Err(_) => 0,
	};

	// Update editor text position
	editor.text_position = loc;

	// Get the current location on the line
	let pos = editor.text_position;
	// Get the previous grapheme
	let character = line
		.grapheme_indices(true)
		.filter_map(|(loc, graph)| if loc == pos { Some(graph) } else { None })
		.last()
		.unwrap_or_default();

	// Get the width of the current grapheme
	let char_width = UnicodeWidthStr::width(character);
	// Move the screen cursor
	editor.cursor_position[0] -= char_width;
	// Store the cursor position if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}
}

// Logic for moving left if not at the beginning of the line
fn left_not_beginning_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If the previous char isn't a tab, move normally
	if line.chars().nth(editor.text_position - 1) != Some('\t') {
		left_not_tab(editor, line_num, &line, will_store_cursor);
	// Otherwise, move by the number of tab spaces
	} else {
		editor.text_position -= 1;
		editor.cursor_position[0] -= editor.config.tab_width;
		// Store the cursor position if the flag is set
		if will_store_cursor {
			editor.stored_position = editor.cursor_position[0];
		}
	}
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't move before the beginning of the line
	if check_cursor_begin_line(editor) {
		// Move left if not at the beginning of the line (move normally)
		left_not_beginning_of_line(editor, line_num, will_store_cursor);
	} else {
		// Move to above line
		if line_num > 0 {
			up_arrow(editor);
			end_key(editor, will_store_cursor);
		} else {
			home_key(editor, will_store_cursor);
		}
	}
}

// Jump one unicode word to the left
pub fn jump_left(editor: &mut EditorSpace, will_highlight: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Line number of current line in the text
	let line = editor.blocks.as_ref().unwrap().get_line(line_num).unwrap();

	// Get the index of the previous word
	let index = line
		.unicode_word_indices()
		.filter(|(idx, _)| *idx < editor.text_position)
		.last()
		.unwrap_or((0, ""))
		.0;

	// Move to the beginning of the previous word
	while editor.text_position > index {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_left(editor);
		// If set to not highlight
		} else {
			left_arrow(editor, true);
		}
	}
}

// Logic for moving right in the text for a non-tab char
fn right_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str, will_store_cursor: bool) {
	// Create a cursor to navigate the grapheme cluster
	let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
	// Get the next location in the text
	let loc = cursor.next_boundary(line, 0);
	// Set the text position
	let loc = match loc {
		Ok(num) => match num {
			Some(num) => num,
			None => panic!(
				/* Return the source file name, line number error occurred in this source file,
				and line_num that this grapheme boundary exists on. */
				"{}::right_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
				file!(),
				line!(),
				line_num
			),
		},
		Err(_) => line.len(),
	};

	// Get the position on the line
	let pos = editor.text_position;
	// Get the current grapheme
	let character = line
		.grapheme_indices(true)
		.filter_map(|(loc, graph)| if loc == pos { Some(graph) } else { None })
		.last()
		.unwrap_or_default();

	// Update editor text position
	editor.text_position = loc;

	// Get the width of the current grapheme
	let char_width = UnicodeWidthStr::width(character);
	// Move the screen cursor
	editor.cursor_position[0] += char_width;
	// Set the stored cursor position if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}
}

// Logic for moving right in the text before reaching the end of the line
fn right_not_end_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If not a tab character, move normally
	if line.chars().nth(editor.text_position) != Some('\t') {
		// Move right for non-tab chars
		right_not_tab(editor, line_num, &line, will_store_cursor);
	// Otherwise, move the number of tab spaces
	} else {
		editor.text_position += 1; // tabs are width 1
		editor.cursor_position[0] += editor.config.tab_width;
		// Store the cursor if the flag is set
		if will_store_cursor {
			editor.stored_position = editor.cursor_position[0];
		}
	}
}

// Logic for moving right in the text when at the end of the line (move to next line)
fn right_end_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// Last line that the cursor can move to
	let file_length = editor.file_length - 1;

	// Move to next line
	if line_num < file_length {
		down_arrow(editor);
		home_key(editor, will_store_cursor);
	} else {
		end_key(editor, will_store_cursor);
	}
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// Move right normally
		right_not_end_of_line(editor, line_num, will_store_cursor);
	// If the cursor goes beyond the end of the line
	} else {
		// Move right at the end of the line (move to next line)
		right_end_of_line(editor, line_num, will_store_cursor);
	}
}

// Jump one unicode word to the right
pub fn jump_right(editor: &mut EditorSpace, will_highlight: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Line number of current line in the text
	let line = editor.blocks.as_ref().unwrap().get_line(line_num).unwrap();

	// Get the index of the next word
	let index = line
		.unicode_word_indices()
		.find(|(idx, _)| *idx > editor.text_position)
		.unwrap_or((line.len(), ""))
		.0;

	// Move to the beginning of the next word
	while editor.text_position < index {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_right(editor);
		// If set to not highlight
		} else {
			right_arrow(editor, true);
		}
	}
}

// Logic for moving up without scrolling
fn up_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the prev line
	editor.cursor_position[1] -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// Logic for moving up while scrolling
fn up_with_scroll(editor: &mut EditorSpace) {
	// Scroll up
	editor.scroll_offset -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// Logic for loading new blocks while moving up
fn up_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the head
	blocks.as_mut().unwrap().push_head(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;

	// Update scroll offset
	editor.scroll_offset += editor.blocks.as_ref().unwrap().get_head().len - 1;
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace) {
	// The current line number
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Ensure that the cursor doesn't move above the editor block
	if editor.cursor_position[1] > 0 {
		// Move up without scrolling
		up_no_scroll(editor);
	// If the cursor moves beyond the bound
	} else if editor.scroll_offset > 0 {
		// Move up and scroll
		up_with_scroll(editor);
	// If moving before the start of the block, insert a new head
	} else if line_num < editor.blocks.as_ref().unwrap().starting_line_num + 1 && line_num > 0 {
		// Move up and load blocks
		up_load_blocks(editor);
	}
}

// Move the cursor up 10 lines
pub fn jump_up(editor: &mut EditorSpace, will_highlight: bool) {
	// Move up 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_up(editor);
		// If set to not highlight
		} else {
			up_arrow(editor);
		}
	}
}

// Logic for moving down without scrolling
fn down_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the next line
	editor.cursor_position[1] += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// Logic for loading blocks when moving down
fn down_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the tail (and remove head if necessary)
	blocks.as_mut().unwrap().push_tail(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;
}

// Logic for moving down while scrolling
fn down_with_scroll(editor: &mut EditorSpace) {
	// Scroll down
	editor.scroll_offset += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// If moving after the end of the block, insert a new tail
	if line_num
		>= editor.blocks.as_ref().unwrap().starting_line_num + editor.blocks.as_ref().unwrap().len()
		&& line_num < editor.file_length - 1
	{
		// Move down and load new blocks
		down_load_blocks(editor);
	}
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// The main control flow of the down arrow key
fn down_conditions(editor: &mut EditorSpace) {
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];
	// Ensure that the cursor doesn't move below the editor block (sub 3 because 2 lines of borders)
	if cursor_line_num < editor.height {
		// Move down without scrolling
		down_no_scroll(editor);
	// If the cursor goes below the bound
	} else {
		// Move down and scroll
		down_with_scroll(editor);
	}
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Last line that the cursor can move to
	let file_length = editor.file_length - 1;

	// Ensure that the cursor doesn't move beyond the end of the file
	if line_num < file_length {
		// Following proper control flow for moving down
		down_conditions(editor);
	}
}

// Move the cursor down 10 lines
pub fn jump_down(editor: &mut EditorSpace, will_highlight: bool) {
	// Move down 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_down(editor);
		// If set to not highlight
		} else {
			down_arrow(editor);
		}
	}
}

// Move up one page
pub fn page_up(editor: &mut EditorSpace) {
	// Move up one page.
	for _i in 0..editor.height + 1 {
		up_arrow(editor);
	}
}

// Move down one page
pub fn page_down(editor: &mut EditorSpace) {
	// Move down one page.
	for _i in 0..editor.height + 1 {
		down_arrow(editor);
	}
}
