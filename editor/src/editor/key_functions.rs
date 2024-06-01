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
			// Remove the current line
			editor.content.remove(editor.pos.1);
			// Move up one line
			up_arrow(editor, config);
			end_key(editor, config);
			// Append the rest of the line to the previous line (where the cursor is moving to)
			editor.content[editor.pos.1].push_str(after_cursor);
		}
	} else {    // Move cursor left
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
	} else if editor.pos.1 < editor.content.len() - 1 { // If not at end of last line
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
		up_arrow(editor, config);
		end_key(editor, config);
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
		down_arrow(editor, config);
		home_key(editor);
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

// Move the cursor right and change the highlighted selection of text
pub fn highlight_right(editor: &mut EditorSpace, config: &Config) {
	// If there is currently no selection
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Set the starting selection point
		let begin = (editor.pos.0 as isize, editor.pos.1 as isize);
		editor.selection = (begin, begin);
		// Move right
		right_arrow(editor, config);
		// Set endpoint for selection
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		editor.selection = (editor.selection.0, end);
	} else {
		// Ensure doesn't try to select past end of line
		if editor.pos.0 < editor.content[editor.pos.1].len() && editor.pos.0 < (editor.width.1 - editor.width.0 - 1) {
			// Move right
			right_arrow(editor, config);
			// New location
			let update = (editor.pos.0 as isize, editor.pos.1 as isize);

			// Remove selection if deselecting last position
			if update == editor.selection.1 {
				editor.selection = ((-1, -1), (-1, -1));
			// If the cursor is before the selection, deselect characters
			} else if update.1 == editor.selection.0.1 && update.0 < editor.selection.1.0 {
				editor.selection = (update, editor.selection.1);
			} else if update.0 <= editor.content[editor.pos.1].len() as isize {	// Otherwise, continue selecting at the end
				// Update endpoint for selection
				editor.selection = (editor.selection.0, update);
			}
		}
	}
}

// Move the cursor left and change the highlighted selection of text
pub fn highlight_left(editor: &mut EditorSpace, config: &Config) {
	// If there is currently no selection
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Set the starting selection point
		let begin = (editor.pos.0 as isize, editor.pos.1 as isize);
		editor.selection = (begin, begin);
		// Move left
		left_arrow(editor, config);
		// Set startpoint for selection
		let begin = (editor.pos.0 as isize, editor.pos.1 as isize);
		editor.selection = (begin, editor.selection.1);
	} else {
		// Ensure doesn't highlight before line
		if editor.pos.0 > 0 {
			// Move left
			left_arrow(editor, config);
			// New location
			let update = (editor.pos.0 as isize, editor.pos.1 as isize);
			
			// Remove selection if deselecting last position
			if update == editor.selection.0 {
				editor.selection = ((-1, -1), (-1, -1));
			// If the cursor is after the end of the selection, deselect characters
			} else if update.1 == editor.selection.1.1 && update.0 > editor.selection.0.0 {
				editor.selection = (editor.selection.0, update);
			} else if update.0 >= 0 {	// Otherwise, continue selecting at the beginning
				// Update startpoint for selection
				editor.selection = (update, editor.selection.1);
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::editor::key_functions;
	use crate::editor::EditorSpace;
	use config::config::Config;

	// Test the highlight right when starting from no highlighted selection
	#[test]
	fn highlight_right_initial_selection() {
		// Construct a new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((0, 0), 100, 1);

		// Highlight 3 characters (123)
		for _i in 0..3 {
			key_functions::highlight_right(&mut editor, &config);
		}

		// Check that the endpoints were updated correctly
		assert_eq!(editor.selection, ((0, 0), (3, 0)));
		assert_ne!(editor.selection, ((3, 0), (6, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "123");
	}

	// Test highlight right when the highlighted selection gets reset and then a new highlighted selection is done
	#[test]
	fn highlight_right_reset_selection() {
		// Construct a new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((0, 0), 100, 1);
		// Highlight 3 character
		for _i in 0..3 {
			key_functions::highlight_right(&mut editor, &config);
		}
		// Reset selection
		editor.selection = ((-1, -1), (-1, -1));

		// Select next 3 (456)
		for _i in 0..3 {
			key_functions::highlight_right(&mut editor, &config);
		}

		// Check endpoints
		assert_ne!(editor.selection, ((0, 0), (3, 0)));
		assert_eq!(editor.selection, ((3, 0), (6, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "456");
		assert_ne!(selected_string, "45");
	}

	// Check the highlight right when the end of a line (or editor block) is reached
	#[test]
	fn highlight_right_overflow_check() {
		// Construct new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set to beginning of line
		editor.set_starting_pos((0, 0), 100, 1);
		// Highlight 3 characters
		for _i in 0..20 {
			key_functions::highlight_right(&mut editor, &config);
		}

		// Check overflow
		assert_eq!(editor.selection, ((0, 0), (10, 0)));
		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "1234567890");
	}

	// Test the highlight left when starting from no highlighted selection
	#[test]
	fn highlight_left_initial_selection() {
		// Construct new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set the starting position
		editor.pos = (5, 0);
		editor.cursor_pos = (6 + editor.width.0, 0);
		// Highlight 3 characters
		for _i in 0..3 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Check correct selection
		assert_eq!(editor.selection, ((2, 0), (5, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "345");
	}

	// Test the highlight left when the highlighted selection gets reset and then a new selection is performed
	#[test]
	fn highlight_left_reset_selection() {
		// Construct new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set the starting position
		editor.pos = (5, 0);
		editor.cursor_pos = (6 + editor.width.0, 0);
		// Highlight 3 characters
		for _i in 0..3 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Reset the selection
		editor.selection = ((-1, -1), (-1, -1));
		// Highlight 3
		for _i in 0..3 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Check correct selection
		assert_eq!(editor.selection, ((0, 0), (2, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "12");
	}

	// Test that highlight left doesn't highlight before the beginning of the line
	#[test]
	fn highlight_left_overflow_check() {
		// Construct new editor
		let config = Config::default();
		let filename = String::from("../editor/test_files/highlight_horizontal.txt");
		let mut editor = EditorSpace::new(filename, &config);

		// Set to beginning of line
		editor.pos = (0, 0);
		// Highlight 3 characters
		for _i in 0..3 {
			key_functions::highlight_left(&mut editor, &config);
		}

		// Check overflow
		assert_eq!(editor.selection, ((0, 0), (0, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
		assert_eq!(selected_string, "");
	}
}

