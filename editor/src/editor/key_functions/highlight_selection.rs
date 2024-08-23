// Defines the logic of the movement keys that highlight selections of text.
// Also defines the Selection struct for track the highlighted selection.

use super::{down_arrow, end_key, home_key, left_arrow, right_arrow, up_arrow, EditorSpace};

// Module containing direction keys to track movement
mod movement;
use movement::Movement;

// Structure that keeps track of the highlighted selection of text
pub struct Selection {
	// The start point of the selection
	pub start: [usize; 2],
	// The endpoint of the selection
	pub end: [usize; 2],
	// Flag to track if selection is empty or not
	pub is_empty: bool,
	// Store the original position of the cursor before highlighting
	pub original_cursor_position: (usize, usize),
	// Store the original position in the text before highlighting
	original_text_position: (usize, usize),
	// Store the original scroll offset of the text
	pub original_scroll_offset: usize,
}

impl Selection {
	// Create a new Selection struct
	pub fn new() -> Self {
		Selection {
			start: [0, 0],
			end: [0, 0],
			is_empty: true,
			original_cursor_position: (0, 0),
			original_text_position: (0, 0),
			original_scroll_offset: 0,
		}
	}
}

// Initialize selection if there currently is no selection
fn init_selection(editor: &mut EditorSpace, movement: Movement) {
	// Store the starting position of the cursor
	editor.selection.original_cursor_position =
		(editor.cursor_position[0], editor.cursor_position[1]);
	// Store the original starting position in the text
	editor.selection.original_text_position = (
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	);
	// Store the original scroll offset of the text
	editor.selection.original_scroll_offset = editor.scroll_offset;

	// Initialize highlighting forward
	if movement == Movement::Right || movement == Movement::Down || movement == Movement::End {
		// Set the starting point of the selection
		editor.selection.start = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Use the corresponding movement key
		movement.take_movement(editor);
		// Set the endpoint of the selection
		editor.selection.end = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
	// Initialize highlighting backwards
	} else {
		// Set the endpoint of the selection
		editor.selection.end = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Use the corresponding movement key
		movement.take_movement(editor);
		// Set the starting point of the selection
		editor.selection.start = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
	}

	// Flag selection as being not empty
	editor.selection.is_empty = false;
}

// Highlight (or un-highlight) to the right of the cursor
pub fn highlight_right(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::Right);
	// Otherwise, add to the existing selection
	} else {
		// Move right
		right_arrow(editor);
		// Get the new location after the move
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// If the last character of the selection has been deselected
		if update == editor.selection.end {
			// Reset selection
			editor.selection.is_empty = true;
		// If the new location is on the last line of the selection
		} else if update[1] >= editor.selection.end[1] {
			// If before end of selection on last line
			if update[0] < editor.selection.end[0] && update[1] == editor.selection.end[1] {
				// Deselect
				editor.selection.start = update;
			// Otherwise, add to the end of the selection
			} else {
				editor.selection.end = update;
			}
		// If not on last line
		} else {
			// Deselect from the front of the selection
			editor.selection.start = update;
		}
	}
}

// Highlight (or un-highlight) to the left of the cursor
pub fn highlight_left(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::Left);
	// Otherwise, add to the existing selection
	} else {
		// Move left
		left_arrow(editor);
		// Get the new location after the move
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// If the new location is at the start of the selection (de-selected last character)
		if update == editor.selection.start {
			// Reset selection
			editor.selection.is_empty = true;
		// If before or on first line
		} else if update[1] <= editor.selection.start[1] {
			// If after beginning of selection on first line
			if update[0] > editor.selection.start[0] && update[1] == editor.selection.start[1] {
				// Deselect
				editor.selection.end = update;
			// Otherwise, add to the front of the selection
			} else {
				editor.selection.start = update;
			}
		// If not on first line
		} else {
			editor.selection.end = update;
		}
	}
}

// Highlight (or un-highlight) up to the cursor position on the above line
pub fn highlight_up(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::Up);
	// Otherwise, add to the existing selection
	} else {
		// Store the current location
		let prior = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move up
		up_arrow(editor);
		// Get the new location after moving
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// If the selection is now empty (but not on first line)
		if update == editor.selection.start && update[1] > 0 {
			// Reset selection
			editor.selection.is_empty = true;
		// If moving at the beginning of the selection
		} else if prior[1] <= editor.selection.start[1] {
			// Update the beginning of the selection
			editor.selection.start = update;
		// If moving at the end of the selection
		} else {
			// Deselect from the end
			editor.selection.end = update;
		}
	}
}

// Highlight (or un-highlight) up to the cursor position on the below line
pub fn highlight_down(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::Down);
	// Otherwise, add to the existing selection
	} else {
		// Store the current location
		let prior = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move down
		down_arrow(editor);
		// Get the new location after moving
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// If the selection is now empty (but not on last line)
		if update == editor.selection.end && update[1] < editor.file_length - 1 {
			// Reset selection
			editor.selection.is_empty = true;
		// If moving at the end of the selection
		} else if prior[1] >= editor.selection.end[1] {
			// Update the end of the selection
			editor.selection.end = update;
		// If moving at the beginning of the selection
		} else {
			// Deselect from the beginning
			editor.selection.start = update;
		}
	}
}

/* highlight_end subroutine for highlighting text until the end of the line,
provided that the selection is not empty now. */
fn end_subroutine(editor: &mut EditorSpace, update: [usize; 2], prior: [usize; 2]) {
	// If only one line
	if editor.selection.start[1] == editor.selection.end[1]
		&& update[1] == editor.selection.start[1]
	{
		// If cursor before selection
		if prior[0] <= editor.selection.start[0] {
			// Set start to original end
			editor.selection.start = [
				editor.selection.original_text_position.0,
				editor.selection.original_text_position.1,
			];
			// Set end to end of line
			editor.selection.end = update;
		// Otherwise, update endpoint
		} else {
			editor.selection.end = update;
		}
	// If at last line
	} else if prior[1] >= editor.selection.end[1] {
		// Update end
		editor.selection.end = update;
	// If at first line
	} else {
		// Deselect start
		editor.selection.start = update;
	}
}

// Highlight (or un-highlight) to the end of the line
pub fn highlight_end(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::End);
	// Otherwise, add to the existing selection
	} else {
		// Store the current location
		let prior = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move to end of line
		end_key(editor);
		// Get new location after moving
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];

		// If selection is now empty
		if update == editor.selection.end {
			// Reset selection
			editor.selection.is_empty = true;
		// If selection is not empty
		} else {
			// Highlight to the end of the line
			end_subroutine(editor, update, prior);
		}
	}
}

/* highlight_home subroutine for highlighting text until the beginning of the line,
provided that the selection is not empty now. */
fn home_subroutine(editor: &mut EditorSpace, update: [usize; 2], prior: [usize; 2]) {
	// If only one line
	if editor.selection.start[1] == editor.selection.end[1]
		&& update[1] == editor.selection.start[1]
	{
		// If cursor after selection
		if prior[0] >= editor.selection.start[0] {
			// Set end to original start
			editor.selection.end = [
				editor.selection.original_text_position.0,
				editor.selection.original_text_position.1,
			];
			// Set start to beginning of line
			editor.selection.start = update;
		// Otherwise, update beginning
		} else {
			editor.selection.start = update;
		}
	// If at first line
	} else if prior[1] <= editor.selection.start[1] {
		// Update end
		editor.selection.start = update;
	// If at last line
	} else {
		// Deselect start
		editor.selection.end = update;
	}
}

// Highlight (or un-highlight) to the beginning of the line
pub fn highlight_home(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::Home);
	// Otherwise, add to the existing selection
	} else {
		// Store the current location
		let prior = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move to beginning of line
		home_key(editor);
		// Get the new location after moving
		let update = [
			editor.text_position,
			editor.get_line_num(editor.cursor_position[1]),
		];

		// If selection is now empty
		if update == editor.selection.end {
			// Reset selection
			editor.selection.is_empty = true;
		// If not empty
		} else {
			// Highlight to the beginning of the line
			home_subroutine(editor, update, prior);
		}
	}
}
