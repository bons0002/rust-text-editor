// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`

use super::{Config, EditorSpace, File};
use std::io::Write;

mod cursor_line;

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// Insert the character
	editor.content[editor.pos.1].insert(editor.pos.0, code);
	// Move cursor
	editor.pos = (editor.pos.0 + 1, editor.pos.1);
	editor.cursor_pos = (editor.cursor_pos.0 + 1, editor.cursor_pos.1);
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
	// Get the current cursor position
	let loc = (editor.pos.0, editor.pos.1);
	let line = &editor.content[loc.1];

	// Get the rest of the line after the cursor
	let after_cursor = get_after_cursor(line, loc.0);

	// Insert new row
	editor.content.insert(loc.1 + 1, String::from(after_cursor));
	// Remove the rest of the old row after the enter
	editor.content[loc.1].truncate(loc.0);

	// Reset cursor to beginning of line
	editor.pos = (0, loc.1 + 1);
	editor.cursor_pos = (editor.width.0 + 1, editor.cursor_pos.1 + 1);
}

// Functionality of the backspace key
pub fn backspace(editor: &mut EditorSpace, config: &Config) {
	let line = editor.content[editor.pos.1].clone();
	// Remove empty line
	if editor.pos.0 <= 0 {  // If cursor at beginning of line, move to above line
		if editor.content.len() > 1 {
			// Get the text from the rest of the line after the cursor
			let after_cursor = get_after_cursor(&line, editor.pos.0);
			// Move up one line
			up_arrow(editor, config);
			end_key(editor, config);
			// Remove the current line
			editor.content.remove(editor.pos.1 + 1);
			// Append the rest of the line to the previous line (where the cursor is moving to)
			editor.content[editor.pos.1].push_str(after_cursor);
		}
	// Move cursor left
	} else {
		left_arrow(editor, config);
		// Remove one character
		editor.content[editor.pos.1].remove(editor.pos.0);
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If not at the end of the current line
	if editor.pos.0 < editor.content[editor.pos.1].len() {
		// Delete next char
		editor.content[editor.pos.1].remove(editor.pos.0);
	// If not at end of last line
	} else if editor.pos.1 < editor.content.len() - 1 {
		// Get entire next line
		let appending_line = editor.content[editor.pos.1 + 1].clone();
		// Append the next line to the current line
		editor.content[editor.pos.1].push_str(appending_line.as_str());
		// Remove the next line
		editor.content.remove(editor.pos.1 + 1);
	}
}

// Get the rest of the text on the line after the cursor
fn get_after_cursor(line: &String, loc: usize) -> &str {
	// Get the rest of the line and store
	&line[loc..]
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace, config: &Config) {
	// Insert tab character
	editor.content[editor.pos.1].insert(editor.pos.0, '\t');
	// Move cursor
	editor.pos = (editor.pos.0 + 1, editor.pos.1);
	editor.cursor_pos = (editor.cursor_pos.0 + config.tab_width, editor.cursor_pos.1);
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace, config: &Config) {
	// If the cursor doesn't move before the beginning of the editor block
	if check_cursor_begin_line(editor) {
		// If the next char isn't a tab, move normally
		if editor.content[editor.pos.1].chars().nth(editor.pos.0 - 1) != Some('\t') {
			editor.pos = (editor.pos.0 - 1, editor.pos.1);
			editor.cursor_pos = (editor.cursor_pos.0 - 1, editor.cursor_pos.1);
		} else {	// Otherwise, move by the number of tab spaces
			editor.pos = (editor.pos.0 - 1, editor.pos.1);
			editor.cursor_pos = (editor.cursor_pos.0 - config.tab_width, editor.cursor_pos.1);
		}
	} else {
		// Move to above line
		if editor.pos.1 > 0 {
			up_arrow(editor, config);
			end_key(editor, config);
		} else {
			home_key(editor);
		}
	}
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, config: &Config) {
	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, editor.pos.1) {
		// If not a tab character, move normally
		if editor.content[editor.pos.1].chars().nth(editor.pos.0) != Some('\t') {
			editor.pos = (editor.pos.0 + 1, editor.pos.1);
			editor.cursor_pos = (editor.cursor_pos.0 + 1, editor.cursor_pos.1);
		} else {	// Otherwise, move the number of tab spaces
			editor.pos = (editor.pos.0 + 1, editor.pos.1);
			editor.cursor_pos = (editor.cursor_pos.0 + config.tab_width, editor.cursor_pos.1);
		}
	} else {
		// Move to next line
		if editor.pos.1 < editor.content.len() - 1 {
			down_arrow(editor, config);
			home_key(editor);
		} else {
			end_key(editor, config);
		}
	}
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace, config: &Config) {
	// Ensure that the cursor doesn't move above the editor block
	if editor.cursor_pos.1 > editor.height.0 + 1 {
		// Move the cursor to the previous line
		cursor_line::move_cursor_line(editor, config, cursor_line::Operation::SUB, 1, 1);
	} else if editor.scroll_offset.0 > 0 {	// If the cursor moves beyond the bound, scroll up
		// Scroll up
		editor.scroll_offset = (editor.scroll_offset.0 - 1, editor.scroll_offset.1);

		// Move to the previous line in the text, but don't move the screen cursor
		cursor_line::move_cursor_line(editor, config, cursor_line::Operation::SUB, 1, 0);
	}
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace, config: &Config) {
	// Make sure cursor doesn't move outside of text
	if editor.pos.1 < editor.content.len() - 1 {
		// Ensure that the cursor doesn't move below the editor block
		if editor.cursor_pos.1 < ((editor.height.1 - editor.height.0) + 1) {
			// Move the cursor to the next line
			cursor_line::move_cursor_line(editor, config, cursor_line::Operation::ADD, 1, 1);
		} else if editor.scroll_offset.0 < editor.content.len() as u16 {
			// If the cursor goes below the bound, scroll down
			// Scroll down
			editor.scroll_offset = (editor.scroll_offset.0 + 1, editor.scroll_offset.1);

			// Move the position in the text, but don't move the screen cursor
			cursor_line::move_cursor_line(editor, config, cursor_line::Operation::ADD, 1, 0);
		}
	}
}

// Check the end of line cursor condition
fn check_cursor_end_line(editor: &mut EditorSpace, idx: usize) -> bool {
	// If the x position is beyond the end of the line, return false
	if editor.pos.0 >= editor.content[idx].chars().count() {
		return false;
	}
	true
}

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
	// If the x position is before the start of the line, return false
	if editor.pos.0 <= 0 {
		return false;
	}
	true
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace) {
	// Move to beginning of line
	editor.pos = (0, editor.pos.1);
	editor.cursor_pos = (editor.width.0 + 1, editor.cursor_pos.1)
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace, config: &Config) {
	// Count the number of tab characters
	let tab_chars = editor.content[editor.pos.1].matches('\t').count() * (config.tab_width - 1);

	// Move to end of line if not past the end of the block
	if editor.content[editor.pos.1].len() < (editor.width.1 - editor.width.0) {
		// Set the cursor to the end of the visual line in the block
		editor.pos = (editor.content[editor.pos.1].len(), editor.pos.1);
		editor.cursor_pos = (
			editor.width.0 + editor.content[editor.pos.1].len() + 1 + tab_chars,
			editor.cursor_pos.1,
		);
	} else {	// If line longer than width of block, move to the end of the 'visible' line
		editor.pos = ((editor.width.1 - editor.width.0) - 1, editor.pos.1);
		editor.cursor_pos = (
			editor.width.0 + (editor.width.1 - editor.width.0) + tab_chars - 1,
			editor.cursor_pos.1,
		);
	}
}

// Save key combo functionality
pub fn save_key_combo(editor: &mut EditorSpace) {
	// Create a blank file
	let mut file = match File::create(&editor.filename) {
		Ok(file) => file,
		Err(_) => panic!("Could not open file"),
	};
	// Write the content to the new file
	for line in &editor.content {
		writeln!(file, "{}", line).unwrap();
	}
}

// Shift + Right_Arrow highlights (or un-highlights) a selection of text as moving right
pub fn highlight_right(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get the start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move right
		right_arrow(editor, config);
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Move right and get location
		right_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If last char
		if update == editor.selection.1 {
			editor.selection = ((-1, -1), (-1, -1));
		// If after on last line
		} else if update.1 >= editor.selection.1.1 {
			// If before end of selection on last line
			if update.0 < editor.selection.1.0 && update.1 == editor.selection.1.1 {
				// Deselect
				editor.selection = (update, editor.selection.1);
			// Otherwise, add to the front of the selection
			} else {
				editor.selection = (editor.selection.0, update);
			}
		// If not on last line
		} else {
			editor.selection = (update, editor.selection.1);
		}
	}
}

// Shift + Left_Arrow highlights (or un-highlights) a selection of text as moving left
pub fn highlight_left(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move left
		left_arrow(editor, config);
		// Get start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Move left and get location
		left_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If last char
		if update == editor.selection.0 {
			editor.selection = ((-1, -1), (-1, -1));
		// If before or on first line
		} else if update.1 <= editor.selection.0.1 {
			// If after beginning of selection on first line
			if update.0 > editor.selection.0.0 && update.1 == editor.selection.0.1 {
				// Deselect
				editor.selection = (editor.selection.0, update);
			// Otherwise, add to the front of the selection
			} else {
				editor.selection = (update, editor.selection.1);
			}
		// If not on first line
		} else {
			editor.selection = (editor.selection.0, update);
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::editor::key_functions;
	use crate::editor::EditorSpace;
	use config::config::Config;

	// Test highlighting 3 characters to the right
	#[test]
	fn highlight_right_3_chars() {
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((0, 0), 100, 100);
		editor.pos = (0, 1);

		// Select 3 characters
		for _i in 0..3 {
			key_functions::highlight_right(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection, ((0, 1), (3, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "abc");
	}

	// Test whether highlight_right correctly wraps to the second line
	#[test]
	fn highlight_right_wrap() {
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((100, 100), 100, 100);
		editor.pos = (4, 0);

		// Select 10 characters
		for _i in 0..10 {
			key_functions::highlight_right(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection, ((4, 0), (4, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let selected_string_2 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
		let mut selected_string = String::from(selected_string_1);
		selected_string.push_str(selected_string_2);

		assert_eq!(selected_string, "56789abcd");
	}

	// Test highlighting 3 characters to the left
	#[test]
	fn highlight_left_3_chars() {
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((100, 100), 100, 100);
		editor.pos = (4, 0);

		// Select 3 characters
		for _i in 0..3 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection, ((1, 0), (4, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "234");
	}

	// Test whether highlight_left correctly wraps to the first line
	#[test]
	fn highlight_left_wrap() {
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((1, 1), 10, 2);
		editor.cursor_pos = (4, 4);	// Arbitrary, just needs to satisfy `up_arrow` wrapping condition
		editor.pos = (4, 1);

		// Select 10 characters
		for _i in 0..10 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection, ((4, 0), (4, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let selected_string_2 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
		let mut selected_string = String::from(selected_string_1);
		selected_string.push_str(selected_string_2);
		
		assert_eq!(selected_string, "56789abcd");
	}
}

