// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`
// Contains the logic for all the keys pressed

use super::{Config, EditorSpace, File};
use std::io::Write;

mod cursor_line;
// Contains logic for all highlighting keys
pub mod highlight_selection;

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		//editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert the character into the correct line in the correct block
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, code);

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += 1;
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace, config: &Config) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		//editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert tab character into the line
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, '\t');

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += config.tab_width;
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		//editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert a new line and truncate the current one (after the cursor)
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_new_line(line_num, editor.text_position);

	// Reset cursor to beginning of line
	editor.text_position = 0;
	editor.cursor_position = [editor.width.0 + 1, editor.cursor_position[1] + 1];
	// Add a line to the overall file length
	editor.file_length += 1;
}

// Functionality of the backspace key
pub fn backspace(editor: &mut EditorSpace, config: &Config) {
	// If there is no highlighted selection, backspace normally
	if editor.selection.is_empty {
		// Remove empty line
		// If cursor at beginning of line, move to above line
		if editor.text_position == 0 {
			if editor.file_length > 1 {
				// Move up one line
				up_arrow(editor, config);
				end_key(editor, config);
				// Line number of current line in the block
				let line_num = editor.get_line_num();

				// Delete the previous line and append its text content to the current line
				editor.blocks.as_mut().unwrap().delete_line(line_num);

				// Reduce the file length
				editor.file_length -= 1;
			}
		// Otherwise, just move cursor left
		} else {
			// Move left
			left_arrow(editor, config);
			// Line number of current line in the block
			let line_num = editor.get_line_num();

			// Remove one character
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_char_in_line(line_num, editor.text_position);
		}
	} else {
		// Delete the selection
		//editor.delete_selection();
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If there is no highlighted selection, delete normally
	if editor.selection.is_empty {
		// Line number of current line in the block
		let line_num = editor.get_line_num();

		// If not at the end of the current line
		if editor.text_position < editor.blocks.as_ref().unwrap().get_line(line_num).len() {
			// Delete next char
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_char_in_line(line_num, editor.text_position);
		// If not at end of last line
		} else if line_num < editor.file_length - 1 {
			// Delete the below line and append its text content to the current line
			editor.blocks.as_mut().unwrap().delete_line(line_num);
			// Reduce the overall file length
			editor.file_length -= 1;
		}
	} else {
		// Delete the selection
		//editor.delete_selection();
	}
}

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
	// Position on the current line
	let line_position = editor.text_position[0];

	// If the x position is before the start of the line, return false
	if line_position == 0 {
		return false;
	}
	true
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace, config: &Config) {
	// Position on the current line
	let line_position = editor.text_position[0];
	// Line number of current line in the block
	let line_num = editor.text_position[1];

	// If the cursor doesn't move before the beginning of the editor block
	if check_cursor_begin_line(editor) {
		// If the next char isn't a tab, move normally
		if editor.block[line_num].chars().nth(line_position - 1) != Some('\t') {
			editor.text_position[0] -= 1;
			editor.cursor_position[0] -= 1;
		// Otherwise, move by the number of tab spaces
		} else {
			editor.text_position[0] -= 1;
			editor.cursor_position[0] -= config.tab_width;
		}
	} else {
		// Move to above line
		if line_num > 0 {
			up_arrow(editor, config);
			end_key(editor, config);
		} else {
			home_key(editor);
		}
	}
}

// Check the end of line cursor condition
fn check_cursor_end_line(editor: &mut EditorSpace, idx: usize) -> bool {
	// Position on the current line
	let line_position = editor.text_position[0];

	// If the x position is beyond the end of the line, return false
	if line_position >= editor.block[idx].chars().count() {
		return false;
	}
	true
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, config: &Config) {
	// Position on the current line
	let line_position = editor.text_position[0];
	// Line number of current line in the block
	let line_num = editor.text_position[1];

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// If not a tab character, move normally
		if editor.block[line_num].chars().nth(line_position) != Some('\t') {
			editor.text_position[0] += 1;
			editor.cursor_position[0] += 1;
		// Otherwise, move the number of tab spaces
		} else {
			editor.text_position[0] += 1;
			editor.cursor_position[0] += config.tab_width;
		}
	} else {
		// Move to next line
		if line_num < editor.file_length - 1 {
			down_arrow(editor, config);
			home_key(editor);
		} else {
			end_key(editor, config);
		}
	}
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace, config: &Config) {
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];

	// Ensure that the cursor doesn't move above the editor block
	if cursor_line_num > editor.height.0 + 1 {
		// Move the cursor to the previous line
		cursor_line::move_cursor_line(editor, config, cursor_line::Operation::Sub, 1, 1);
	// If the cursor moves beyond the bound, scroll up
	} else if editor.scroll_offset > 0 {
		// Scroll up
		editor.scroll_offset -= 1;

		// Move to the previous line in the text, but don't move the screen cursor
		cursor_line::move_cursor_line(editor, config, cursor_line::Operation::Sub, 1, 0);
	}
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace, config: &Config) {
	// Line number of current line in the block
	let line_num = editor.text_position[1];
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];

	// Make sure cursor doesn't move outside of text
	if line_num < editor.file_length - 1 {
		// Ensure that the cursor doesn't move below the editor block
		if cursor_line_num < ((editor.height.1 - editor.height.0) + 1) {
			// Move the cursor to the next line
			cursor_line::move_cursor_line(editor, config, cursor_line::Operation::Add, 1, 1);
		// If the cursor goes below the bound, scroll down
		} else if editor.scroll_offset < editor.file_length {
			// Scroll down
			editor.scroll_offset += 1;

			// Move the position in the text, but don't move the screen cursor
			cursor_line::move_cursor_line(editor, config, cursor_line::Operation::Add, 1, 0);
		}
	}
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace) {
	// Move to beginning of line
	editor.text_position = [0, editor.text_position[1]];
	editor.cursor_position = [editor.width.0 + 1, editor.cursor_position[1]];
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace, config: &Config) {
	// Line number of current line in the block
	let line_num = editor.text_position[1];
	// Count the number of tab characters
	let tab_chars = editor.block[line_num].matches('\t').count() * (config.tab_width - 1);

	// Move to end of line if not past the end of the widget
	if editor.block[line_num].len() < (editor.width.1 - editor.width.0) {
		// Set the cursor to the end of the visual line in the widget
		editor.text_position[0] = editor.block[line_num].len();
		// Set screen cursor to end of line
		editor.cursor_position[0] = editor.width.0 + editor.block[line_num].len() + 1 + tab_chars;
	// If line longer than width of widget, move to the end of the 'visible' line
	} else {
		// Set position in text
		editor.text_position[0] = (editor.width.1 - editor.width.0) - 1;
		// Set screen cursor to end of widget
		editor.cursor_position[0] =
			editor.width.0 + (editor.width.1 - editor.width.0) + tab_chars - 1;
	}
}

// Save key combo functionality
pub fn save_key_combo(editor: &mut EditorSpace) {
	// Create a blank file
	let mut file = match File::create(&editor.filename) {
		Ok(file) => file,
		Err(_) => panic!("Could not open file"),
	};
	// Write the block to the new file
	for line in &editor.block {
		writeln!(file, "{}", line).unwrap();
	}
}
