// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`
// Contains the logic for all the keys pressed

use super::EditorSpace;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

// Contains logic for all highlighting keys
pub mod highlight_selection;

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert the character into the correct line in the correct block
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, code)
		.unwrap_or_else(|err| panic!("Couldn't insert char on line {} | {}", line_num, err));

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += 1;
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert tab character into the line
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, '\t')
		.unwrap_or_else(|err| panic!("Couldn't insert char on line {} | {}", line_num, err));

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += editor.config.tab_width;
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Insert a new line and truncate the current one (after the cursor)
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_new_line(line_num, editor.text_position)
		.unwrap_or_else(|err| panic!("Couldn't insert new line {} | {}", line_num, err));

	// Add a line to the overall file length
	editor.file_length += 1;

	// Reset cursor to beginning of line
	down_arrow(editor);
	home_key(editor);
}

// Functionality of the backspace key
pub fn backspace(editor: &mut EditorSpace) {
	// If there is no highlighted selection, backspace normally
	if editor.selection.is_empty {
		// Remove empty line
		// If cursor at beginning of line, move to above line
		if editor.text_position == 0 {
			if editor.file_length > 1 {
				// Move up one line
				up_arrow(editor);
				end_key(editor);
				// Line number of current line in the text
				let line_num = editor.get_line_num();

				// Delete the previous line and append its text content to the current line
				editor
					.blocks
					.as_mut()
					.unwrap()
					.delete_and_append_line(line_num)
					.unwrap_or_else(|err| {
						panic!("Couldn't delete line {} | {}", line_num + 1, err)
					});

				// Reduce the file length
				editor.file_length -= 1;
			}
		// Otherwise, just move cursor left
		} else {
			// Move left
			left_arrow(editor);
			// Line number of current line in the text
			let line_num = editor.get_line_num();

			// Remove one character
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_char_in_line(line_num, editor.text_position)
				.unwrap_or_else(|err| {
					panic!("Couldn't delete char on line {} | {}", line_num, err)
				});
		}
	} else {
		// Delete the selection
		editor.delete_selection();
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If there is no highlighted selection, delete normally
	if editor.selection.is_empty {
		// Line number of current line in the text
		let line_num = editor.get_line_num();

		let length = match editor.blocks.as_ref().unwrap().get_line_length(line_num) {
			Ok(len) => len,
			Err(err) => panic!("Couldn't get length of line {} | {}", line_num, err),
		};

		// If not at the end of the current line
		if editor.text_position < length {
			// Delete next char
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_char_in_line(line_num, editor.text_position)
				.unwrap_or_else(|err| {
					panic!("Couldn't delete char on line {} | {}", line_num, err)
				});
		// If not at end of last line
		} else if line_num < editor.file_length - 1 {
			// Delete the below line and append its text content to the current line
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_and_append_line(line_num)
				.unwrap_or_else(|err| panic!("Couldn't delete line {} | {}", line_num + 1, err));
			// Reduce the overall file length
			editor.file_length -= 1;
		}
	} else {
		// Delete the selection
		editor.delete_selection();
	}
}

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
	// If the x position is before the start of the line, return false
	if editor.text_position == 0 {
		return false;
	}
	true
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// If the cursor doesn't move before the beginning of the line
	if check_cursor_begin_line(editor) {
		// The line of text
		let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
			Ok(line) => line,
			Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
		};
		// If the next char isn't a tab, move normally
		if line.graphemes(true).nth(editor.text_position - 1) != Some("\t") {
			// Create a cursor to navigate the grapheme cluster
			let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
			// Get the previous location in the text
			let loc = cursor.prev_boundary(&line, 0);
			// Set the text position
			let loc = match loc {
				Ok(num) => match num {
					Some(num) => num,
					None => panic!(
						"{}::left_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
						file!(),
						line!(),
						line_num
					),
				},
				Err(_) => 0,
			};
			// Get the difference in the positions
			let diff = editor.text_position - loc;
			// Update editor text position
			editor.text_position -= diff;
			// Move the screen cursor
			match diff > 1 {
				// If there is a non ascii character there, the screen cursor needs to move two spaces
				true => editor.cursor_position[0] -= 2,
				// Otherwise, move one space
				false => editor.cursor_position[0] -= 1,
			}
		// Otherwise, move by the number of tab spaces
		} else {
			editor.text_position -= 1;
			editor.cursor_position[0] -= editor.config.tab_width;
		}
	} else {
		// Move to above line
		if line_num > 0 {
			up_arrow(editor);
			end_key(editor);
		} else {
			home_key(editor);
		}
	}
}

// Check the end of line cursor condition
fn check_cursor_end_line(editor: &mut EditorSpace, line_num: usize) -> bool {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If the x position is beyond the end of the line, return false
	if editor.text_position >= line.len() {
		return false;
	}
	true
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// The line of text
		let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
			Ok(line) => line,
			Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
		};
		// If not a tab character, move normally
		if line.graphemes(true).nth(editor.text_position) != Some("\t") {
			// Create a cursor to navigate the grapheme cluster
			let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
			// Get the next location in the text
			let loc = cursor.next_boundary(&line, 0);
			// Set the text position
			let loc = match loc {
				Ok(num) => match num {
					Some(num) => num,
					None => panic!(
						"{}::right_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
						file!(),
						line!(),
						line_num
					),
				},
				Err(_) => line.len(),
			};
			// Get the difference in the positions
			let diff = loc - editor.text_position;
			// Update editor text position
			editor.text_position += diff;
			// Move the screen cursor
			match diff > 1 {
				// If there is a non ascii character there, the screen cursor needs to move two spaces
				true => editor.cursor_position[0] += 2,
				// Otherwise, move one space
				false => editor.cursor_position[0] += 1,
			}
		// Otherwise, move the number of tab spaces
		} else {
			editor.text_position += 1;
			editor.cursor_position[0] += editor.config.tab_width;
		}
	} else {
		// Move to next line
		if line_num < editor.file_length - 1 {
			down_arrow(editor);
			home_key(editor);
		} else {
			end_key(editor);
		}
	}
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace) {
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];

	// Ensure that the cursor doesn't move above the editor block
	if cursor_line_num > 0 {
		// Move the cursor to the prev line
		editor.cursor_position[1] -= 1;
		// Line number of current line in the text
		let line_num = editor.get_line_num();
		// Save current position
		let position = editor.cursor_position[0];
		// Move cursor to beginning of line
		home_key(editor);
		// Loop until in correct position
		while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
			// Move right
			right_arrow(editor);
		}
	// If the cursor moves beyond the bound, scroll up
	} else if editor.scroll_offset > 0 {
		// Scroll up
		editor.scroll_offset -= 1;
		// Line number of current line in the text
		let line_num = editor.get_line_num();
		// Save current position
		let position = editor.cursor_position[0];
		// Move cursor to beginning of line
		home_key(editor);
		// Loop until in correct position
		while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
			// Move right
			right_arrow(editor);
		}
	// If moving before the start of the block, insert a new head
	} else if editor.get_line_num() < editor.blocks.as_ref().unwrap().starting_line_num + 1
		&& editor.get_line_num() > 0
	{
		// Clone the blocks
		let mut blocks = editor.blocks.clone();
		// Insert a new block at the head
		blocks.as_mut().unwrap().push_head(editor).unwrap();
		// Set this blocks to the editor
		editor.blocks = blocks;

		// Update scroll offset
		editor.scroll_offset += editor.blocks.as_ref().unwrap().get_head().len() - 1;
	}
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num();

	// Make sure cursor doesn't move outside of text
	if line_num < editor.file_length - 1 {
		// Line number of the screen number
		let cursor_line_num = editor.cursor_position[1];
		// Ensure that the cursor doesn't move below the editor block
		if cursor_line_num < (editor.height.1 - editor.height.0) - 3 {
			// Move the cursor to the next line
			editor.cursor_position[1] += 1;
			// Line number of current line in the text
			let line_num = editor.get_line_num();
			// Save current position
			let position = editor.cursor_position[0];
			// Move cursor to beginning of line
			home_key(editor);
			// Loop until in correct position
			while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
				// Move right
				right_arrow(editor);
			}
		// If the cursor goes below the bound, scroll down
		} else {
			// Scroll down
			editor.scroll_offset += 1;

			// Line number of current line in the text
			let line_num = editor.get_line_num();
			// If moving after the end of the block, insert a new tail
			if line_num
				>= editor.blocks.as_ref().unwrap().starting_line_num
					+ editor.blocks.as_ref().unwrap().len()
				&& line_num < editor.file_length - 1
			{
				// Clone the blocks
				let mut blocks = editor.blocks.clone();
				// Insert a new block at the tail (and remove head if necessary)
				blocks.as_mut().unwrap().push_tail(editor).unwrap();
				// Set this blocks to the editor
				editor.blocks = blocks;
			}
			// Save current position
			let position = editor.cursor_position[0];
			// Move cursor to beginning of line
			home_key(editor);
			// Loop until in correct position
			while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
				// Move right
				right_arrow(editor);
			}
		}
	}
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace) {
	// Move to beginning of line
	editor.text_position = 0;
	editor.cursor_position[0] = 0;
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num();
	while check_cursor_end_line(editor, line_num) {
		right_arrow(editor);
	}
}

// Save key combo functionality
pub fn save_key_combo() {}
