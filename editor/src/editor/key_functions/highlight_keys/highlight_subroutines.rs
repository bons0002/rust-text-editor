use std::cmp::Ordering;

use super::{
	down_arrow, end_key, home_key, left_arrow, movement::Movement, right_arrow, up_arrow,
	EditorSpace,
};

// Initialize selection if there currently is no selection
pub fn init_selection(editor: &mut EditorSpace, movement: Movement) {
	// Initialize the selections original position fields
	init_subroutines::init_original_positions(editor);

	// Initialize highlighting forward
	if movement == Movement::Right
		|| movement == Movement::Down
		|| movement == Movement::End
		|| movement == Movement::PageDown
	{
		init_subroutines::init_foward(editor, movement);
	// Initialize highlighting backwards
	} else {
		init_subroutines::init_backwards(editor, movement);
	}

	// Flag selection as being not empty
	editor.selection.is_empty = false;
}

/* Subroutine for the highlight_left function.
Controls all of the actual highlight_left logic */
pub fn left_subroutine(editor: &mut EditorSpace) {
	// Move left
	left_arrow(editor, true);
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

/* Subroutine for the highlight_right function.
Controls all of the actual highlight_right logic */
pub fn right_subroutine(editor: &mut EditorSpace) {
	// Move right
	right_arrow(editor, true);
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

// Subroutine for controlling highlighting down boundary updates
pub fn down_subroutine(editor: &mut EditorSpace) {
	// Move down
	down_arrow(editor);
	// Get the new location after moving
	let update = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// Check the line number that the cursor has moved onto
	match update[1].cmp(&editor.selection.end[1]) {
		// If moving at the beginning of the selection
		Ordering::Less => {
			// Deselect from the beginning
			editor.selection.start = update;
		}
		// If the cursor has moved onto the last line
		Ordering::Equal => {
			down_subroutines::down_equal_branch(editor, update);
		}
		// If moving at the end of the selection
		Ordering::Greater => {
			down_subroutines::down_greater_branch(editor, update);
		}
	}
}

// Subroutine for controlling highlighting up boundary updates
pub fn up_subroutine(editor: &mut EditorSpace) {
	// Move up
	up_arrow(editor);
	// Get the new location after moving
	let update = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// Check the line number that the cursor has moved onto
	match update[1].cmp(&editor.selection.start[1]) {
		// If moving at the beginning of the selection
		Ordering::Less => {
			up_subroutines::up_less_branch(editor, update);
		}
		// If the cursor has moved onto the starting line
		Ordering::Equal => {
			up_subroutines::up_equal_branch(editor, update);
		}
		// If moving at the end of the selection
		Ordering::Greater => {
			// Deselect from the end
			editor.selection.end = update;
		}
	}
}

// highlight_home subroutine for highlighting text until the beginning of the line
pub fn home_subroutine(editor: &mut EditorSpace) {
	// Store the current location
	let prior = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// Move to beginning of line
	home_key(editor, true);
	// Get the new location after moving
	let update = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// If only one line
	if editor.selection.start[1] == editor.selection.end[1]
		&& update[1] == editor.selection.start[1]
	{
		home_end_subroutines::home_one_line(editor, prior, update);
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

// Highlight_end subroutine for highlighting text until the end of the line
pub fn end_subroutine(editor: &mut EditorSpace) {
	// Store the current location
	let prior = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// Move to end of line
	end_key(editor, true);
	// Get new location after moving
	let update = [
		editor.text_position,
		editor.get_line_num(editor.cursor_position[1]),
	];
	// If only one line
	if editor.selection.start[1] == editor.selection.end[1]
		&& update[1] == editor.selection.start[1]
	{
		home_end_subroutines::end_one_line(editor, prior, update);
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

/*
===================================================================
			Subroutines for the init_selection function
===================================================================
*/
mod init_subroutines {
	use super::{EditorSpace, Movement};

	/* Initialize the three original position fields (original_cursor_position,
	original_text_position, original_scroll_offset) for the selection. */
	pub fn init_original_positions(editor: &mut EditorSpace) {
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
	}

	// Initialize highlighting Right, Down, or End
	pub fn init_foward(editor: &mut EditorSpace, movement: Movement) {
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
	}

	// Initialize highlighting Left, Up, or Home
	pub fn init_backwards(editor: &mut EditorSpace, movement: Movement) {
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
}

/*
====================================================================
			Subroutines for the down_subroutine function
====================================================================
*/
mod down_subroutines {
	use super::{EditorSpace, Ordering};

	// Subroutine for highlighting down ONTO the last line of the selection
	pub fn down_equal_branch(editor: &mut EditorSpace, update: [usize; 2]) {
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

	// Subroutine for highlighting down after the last line of the selection
	pub fn down_greater_branch(editor: &mut EditorSpace, update: [usize; 2]) {
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

mod up_subroutines {
	use super::{EditorSpace, Ordering};

	// Subroutine for highlighting up at the beginning of the selection
	pub fn up_less_branch(editor: &mut EditorSpace, update: [usize; 2]) {
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

	// Subroutine for highlighting up ONTO the first line of the selection
	pub fn up_equal_branch(editor: &mut EditorSpace, update: [usize; 2]) {
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
}

/*
========================================================================
			Subroutines for the highlight_home/end functions
========================================================================
*/
mod home_end_subroutines {
	use super::EditorSpace;

	// Subroutine for the highlight_home if performed on only one line of text
	pub fn home_one_line(editor: &mut EditorSpace, prior: [usize; 2], update: [usize; 2]) {
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
	}

	// Subroutine for the highlight_home if performed on only one line of text
	pub fn end_one_line(editor: &mut EditorSpace, prior: [usize; 2], update: [usize; 2]) {
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
	}
}
