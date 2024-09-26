// Defines the logic of the movement keys that highlight selections of text.
// Also defines the Selection struct for track the highlighted selection.

use std::cmp::Ordering;

use super::{
	navigation_keys::{down_arrow, end_key, home_key, left_arrow, right_arrow, up_arrow},
	EditorSpace,
};

// Module containing direction keys to track movement
mod movement;
use movement::Movement;

// Structure that keeps track of the highlighted selection of text
#[derive(Clone, Debug)]
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

impl PartialEq for Selection {
	// Check that two selections are equal
	fn eq(&self, other: &Self) -> bool {
		self.start == other.start
			&& self.end == other.end
			&& self.is_empty == other.is_empty
			&& self.original_cursor_position == other.original_cursor_position
			&& self.original_text_position == other.original_text_position
			&& self.original_scroll_offset == other.original_scroll_offset
	}
}

// Initialize selection if there currently is no selection
fn init_selection(editor: &mut EditorSpace, movement: Movement) {
	// Store the starting position of the cursor
	editor.selection.original_cursor_position =
		(editor.cursor_position[0], editor.cursor_position[1]);
	// Store the original starting position in the text
	editor.selection.original_text_position = (
		editor.index_position,
		editor.get_line_num(editor.cursor_position[1]),
	);
	// Store the original scroll offset of the text
	editor.selection.original_scroll_offset = editor.scroll_offset;

	// Initialize highlighting forward
	if movement == Movement::Right
		|| movement == Movement::Down
		|| movement == Movement::End
		|| movement == Movement::PageDown
	{
		// Set the starting point of the selection
		editor.selection.start = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Use the corresponding movement key
		movement.take_movement(editor);
		// Set the endpoint of the selection
		editor.selection.end = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
	// Initialize highlighting backwards
	} else {
		// Set the endpoint of the selection
		editor.selection.end = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Use the corresponding movement key
		movement.take_movement(editor);
		// Set the starting point of the selection
		editor.selection.start = [
			editor.index_position,
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
		right_arrow(editor, true);
		// Get the new location after the move
		let update = [
			editor.index_position,
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
		left_arrow(editor, true);
		// Get the new location after the move
		let update = [
			editor.index_position,
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

// Subroutine for controlling highlighting up boundary updates
fn up_subroutine(editor: &mut EditorSpace, update: [usize; 2]) {
	// Check the line number that the cursor has moved onto
	match update[1].cmp(&editor.selection.start[1]) {
		// If moving at the beginning of the selection
		Ordering::Less => {
			// If only one line above the last line
			if editor.selection.start[1] - update[1] == 1 {
				// Make sure the end doesn't shift around
				editor.selection.end = [
					editor.selection.original_text_position.0,
					editor.selection.original_text_position.1,
				];
			}
			// Update the beginning of the selection
			editor.selection.start = update;
		}
		// If the cursor has moved onto the starting line
		Ordering::Equal => {
			// If the cursor is before the start
			match update[0].cmp(&editor.selection.start[0]) {
				Ordering::Less => {
					// Set the new end of the selection to the old start
					editor.selection.end = editor.selection.start;
					// Set the new start to the new cursor position
					editor.selection.start = update;
				}
				Ordering::Greater => {
					// Update the end of the selection
					editor.selection.end = update;
				}
				_ => (),
			}
		}
		// If moving at the end of the selection
		Ordering::Greater => {
			// Deselect from the end
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
		// Move up
		up_arrow(editor);
		// Get the new location after moving
		let update = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Update the selection boundaries
		up_subroutine(editor, update);
	}
}

// Subroutine for controlling highlighting down boundary updates
fn down_subroutine(editor: &mut EditorSpace, update: [usize; 2]) {
	// Check the line number that the cursor has moved onto
	match update[1].cmp(&editor.selection.end[1]) {
		// If moving at the beginning of the selection
		Ordering::Less => {
			// Deselect from the beginning
			editor.selection.start = update;
		}
		Ordering::Equal => {
			// If the cursor is before the end
			match update[0].cmp(&editor.selection.end[0]) {
				Ordering::Less => {
					// Set the new start to the new cursor position
					editor.selection.start = update;
				}
				Ordering::Greater => {
					editor.selection.start = editor.selection.end;
					// Update the end of the selection
					editor.selection.end = update;
				}
				_ => (),
			}
		}
		// If moving at the end of the selection
		Ordering::Greater => {
			// If on the second line
			if update[1] - editor.selection.end[1] == 1 {
				// Make sure the start doesn't shift around
				editor.selection.start = [
					editor.selection.original_text_position.0,
					editor.selection.original_text_position.1,
				];
			}
			// Update the end of the selection
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
		// Move down
		down_arrow(editor);
		// Get the new location after moving
		let update = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Update the selection boundaries
		down_subroutine(editor, update);
	}
}

// Highlight_end subroutine for highlighting text until the end of the line
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
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move to end of line
		end_key(editor, true);
		// Get new location after moving
		let update = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];

		// Highlight to the end of the line
		end_subroutine(editor, update, prior);
	}
}

// highlight_home subroutine for highlighting text until the beginning of the line
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
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];
		// Move to beginning of line
		home_key(editor, true);
		// Get the new location after moving
		let update = [
			editor.index_position,
			editor.get_line_num(editor.cursor_position[1]),
		];

		// Highlight to the beginning of the line
		home_subroutine(editor, update, prior);
	}
}

// Highlight (or un-highlight) one page up
pub fn highlight_page_up(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::PageUp);
	} else {
		// If file short than the widget, only move up for the number of lines in the file
		if editor.file_length < editor.height {
			for _i in 0..editor.file_length {
				highlight_up(editor);
			}
		// Otherwise, move up one widget
		} else {
			for _i in 0..editor.height + 1 {
				highlight_up(editor);
			}
		}
	}
}

// Highlight (or un-highlight) one page down
pub fn highlight_page_down(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, Movement::PageDown);
	} else {
		// If file short than the widget, only move down for the number of lines in the file
		if editor.file_length < editor.height {
			for _i in 0..editor.file_length {
				highlight_down(editor);
			}
		// Otherwise, move down one widget
		} else {
			for _i in 0..editor.height + 1 {
				highlight_down(editor);
			}
		}
	}
}
