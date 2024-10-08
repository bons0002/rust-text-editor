use super::{
	navigation_keys::{down_arrow, end_key, home_key, left_arrow, up_arrow},
	EditorSpace,
};

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Add an undo state and delete the selection
		delete_subroutines::selection_delete(editor);
	// Update progress toward a new undo state if the current code is a space
	} else {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);
	}

	// Insert the character into the correct line in the correct block
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(editor.text_position, code);

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += 1;
	editor.stored_position = editor.cursor_position[0];
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Add an undo state and delete the selection
		delete_subroutines::selection_delete(editor);
	} else {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);
	}

	// Insert tab character into the line
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(editor.text_position, '\t');

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += editor.config.tab_width;
	editor.stored_position = editor.cursor_position[0];
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
	// Get the current editor state
	let state = editor.get_unredo_state();
	// Add a new undo state
	editor.unredo_stack.auto_update(state, true);

	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete selection
		editor.delete_selection();
	}

	// Insert a new line and truncate the current one (after the cursor)
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_new_line(editor.text_position);

	// Add a line to the overall file length
	editor.file_length += 1;

	// Reset cursor to beginning of line
	down_arrow(editor);
	home_key(editor, true);
}

// Functionality of the backspace key
pub fn backspace(editor: &mut EditorSpace) {
	// If there is no highlighted selection, backspace normally
	if editor.selection.is_empty {
		// The current line number
		let line_num = editor.get_line_num(editor.cursor_position[1]);
		// Remove empty line
		// If cursor at beginning of line, move to above line
		if editor.text_position == 0 && line_num != 0 {
			// Backspace at beginning of the line
			delete_subroutines::backspace_beginning_of_line(editor);
		// Otherwise, just move cursor left
		} else if editor.text_position != 0 {
			// Backspace normally, deleting one char
			delete_subroutines::backspace_normally(editor);
		}
	} else {
		// Add a new undo state and delete the selection
		delete_subroutines::selection_delete(editor);
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If there is no highlighted selection, delete normally
	if editor.selection.is_empty {
		// Delete character
		delete_subroutines::no_selection_delete(editor);
	} else {
		delete_subroutines::selection_delete(editor);
	}
}

/*
==============================================
			Delete Key Subroutines
==============================================
*/

// Subroutines for the different subroutine keys
mod delete_subroutines {
	use super::{
		super::navigation_keys::down_subroutines, end_key, left_arrow, up_arrow, EditorSpace,
	};

	// Backspace at the beginning of line, moving to the above line
	pub fn backspace_beginning_of_line(editor: &mut EditorSpace) {
		if editor.file_length > 0 {
			// Get the current editor state
			let state = editor.get_unredo_state();
			// Add a new undo state
			editor.unredo_stack.auto_update(state, true);

			// Move up one line
			up_arrow(editor);
			end_key(editor, true);
			// Line number of current line in the text
			let line_num = editor.get_line_num(editor.cursor_position[1]);

			// Delete the previous line and append its text content to the current line
			editor
				.blocks
				.as_mut()
				.unwrap()
				.delete_and_append_line(line_num)
				.unwrap_or_else(|err| panic!("Couldn't delete line {} | {}", line_num + 1, err));

			// Reduce the file length
			editor.file_length -= 1;
		}
	}

	// Backspace after the beginning of the line deletes a char normally
	pub fn backspace_normally(editor: &mut EditorSpace) {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);

		// Move left
		left_arrow(editor, true);

		// Remove one character
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_char_in_line(editor.text_position);
	}

	// Check if there is a selection that needs to be deleted
	pub fn selection_delete(editor: &mut EditorSpace) {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new undo state
		editor.unredo_stack.auto_update(state, true);
		// Delete the selection
		editor.delete_selection();
	}

	// Delete when there is no selection
	pub fn no_selection_delete(editor: &mut EditorSpace) {
		// Line number of current line in the text
		let line_num = editor.get_line_num(editor.cursor_position[1]);

		// The length of the current line
		let line = editor.blocks.as_ref().unwrap().get_current_line();

		// If not at the end of the current line
		if editor.text_position < line.len() {
			// Delete a character normally
			delete_normally(editor);
		// If not at end of last line of the file
		} else if line_num < editor.file_length - 1 {
			delete_end(editor, line_num);
		}
	}

	// Delete a single character normally
	fn delete_normally(editor: &mut EditorSpace) {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);

		// Delete next char
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_char_in_line(editor.text_position);
	}

	// Delete at the end of a line
	fn delete_end(editor: &mut EditorSpace, line_num: usize) {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new undo state
		editor.unredo_stack.auto_update(state, true);

		// Delete the below line and append its text content to the current line
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_and_append_line(line_num)
			.unwrap_or_else(|err| panic!("Couldn't delete line {} | {}", line_num + 1, err));

		// Check if the tracked Blocks location needs to be updated
		down_subroutines::check_tracked_location(editor);

		// Reduce the overall file length
		editor.file_length -= 1;
	}
}
