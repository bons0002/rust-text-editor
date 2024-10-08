// Subroutines for the right arrow functions

use super::{
	down_arrow, end_key, home_key, EditorSpace, GraphemeCursor, UnicodeSegmentation,
	UnicodeWidthStr,
};

// Logic for moving right in the text before reaching the end of the line
pub fn right_normally(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// The line of text
	let line = editor.blocks.as_ref().unwrap().get_current_line();
	// If not a tab character, move normally
	if line.chars().nth(editor.text_position) != Some('\t') {
		// Move right for non-tab chars
		right_not_tab(editor, line_num, &line, will_store_cursor);
	// Otherwise, move the number of tab spaces
	} else {
		right_tab(editor, will_store_cursor);
	}
}

pub fn right_end(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// Move right at the end of the line (move to next line)
	if line_num < editor.file_length - 1 {
		down_arrow(editor);
		home_key(editor, will_store_cursor);
	} else {
		end_key(editor, will_store_cursor);
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

fn right_tab(editor: &mut EditorSpace, will_store_cursor: bool) {
	editor.text_position += 1; // tabs are width 1
	editor.cursor_position[0] += editor.config.tab_width;
	// Store the cursor if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}
}
