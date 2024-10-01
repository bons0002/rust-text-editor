// Subroutines for the left arrow functions

use super::{
	end_key, home_key, up_arrow, EditorSpace, GraphemeCursor, UnicodeSegmentation, UnicodeWidthStr,
};

// Logic for moving left if not at the beginning of the line
pub fn left_normally(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
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
		left_tab(editor, will_store_cursor);
	}
}

// Moving left at the beginning of the line
pub fn left_beginning(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// Move to above line
	if line_num > 0 {
		up_arrow(editor);
		end_key(editor, will_store_cursor);
	} else {
		home_key(editor, will_store_cursor);
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

// Moving left at a tab character
fn left_tab(editor: &mut EditorSpace, will_store_cursor: bool) {
	editor.text_position -= 1;
	editor.cursor_position[0] -= editor.config.tab_width;
	// Store the cursor position if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}
}
