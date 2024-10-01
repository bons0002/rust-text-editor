use super::{
	check_cursor_begin_line, check_cursor_end_line, highlight_keys, EditorSpace, GraphemeCursor,
	UnicodeSegmentation, UnicodeWidthStr,
};

// Subroutines for the left arrow functions
mod left_subroutines;
// Subroutines for the right arrow functions
mod right_subroutines;
// Subroutines for the up arrow functions
mod up_subroutines;
// Subroutines for the down arrow functions
mod down_subroutines;

/*
=============================================
			Regular movement keys
=============================================
*/

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't move before the beginning of the line
	if check_cursor_begin_line(editor) {
		// Move left if not at the beginning of the line (move normally)
		left_subroutines::left_normally(editor, line_num, will_store_cursor);
	} else {
		// Move left at the beginning of the line
		left_subroutines::left_beginning(editor, line_num, will_store_cursor);
	}
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// Move right normally
		right_subroutines::right_normally(editor, line_num, will_store_cursor);
	// If the cursor goes beyond the end of the line
	} else {
		right_subroutines::right_end(editor, line_num, will_store_cursor);
	}
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace) {
	// The current line number
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Ensure that the cursor doesn't move above the editor block
	if editor.cursor_position[1] > 0 {
		// Move up without scrolling
		up_subroutines::up_no_scroll(editor);
	// If the cursor moves beyond the bound
	} else if editor.scroll_offset > 0 {
		// Move up and scroll
		up_subroutines::up_with_scroll(editor);
	// If moving before the start of the block, insert a new head
	} else if line_num < editor.blocks.as_ref().unwrap().starting_line_num + 1 && line_num > 0 {
		// Move up and load blocks
		up_subroutines::up_load_blocks(editor);
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
		// Line number of the screen number
		let cursor_line_num = editor.cursor_position[1];
		// Ensure that the cursor doesn't move below the editor block (sub 3 because 2 lines of borders)
		if cursor_line_num < editor.height {
			// Move down without scrolling
			down_subroutines::down_no_scroll(editor);
		// If the cursor goes below the bound
		} else {
			// Move down and scroll
			down_subroutines::down_with_scroll(editor);
		}
	}
}

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
	// Move right until the end of the line
	while check_cursor_end_line(editor, line_num) {
		right_arrow(editor, will_store_cursor);
	}
}

/*
===============================================
			Jump movement functions
===============================================
*/

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
			highlight_keys::highlight_left(editor);
		// If set to not highlight
		} else {
			left_arrow(editor, true);
		}
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
			highlight_keys::highlight_right(editor);
		// If set to not highlight
		} else {
			right_arrow(editor, true);
		}
	}
}

// Move the cursor up 10 lines
pub fn jump_up(editor: &mut EditorSpace, will_highlight: bool) {
	// Move up 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_keys::highlight_up(editor);
		// If set to not highlight
		} else {
			up_arrow(editor);
		}
	}
}

// Move the cursor down 10 lines
pub fn jump_down(editor: &mut EditorSpace, will_highlight: bool) {
	// Move down 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_keys::highlight_down(editor);
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

/*
==============================
			Helper
==============================
*/

// Realign the cursor to the stored cursor position
fn realign_cursor(editor: &mut EditorSpace, line_num: usize) {
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
