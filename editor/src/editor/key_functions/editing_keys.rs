use super::{
	navigation_keys::{down_arrow, end_key, home_key, left_arrow, up_arrow},
	selection_delete, EditorSpace,
};

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Add an undo state and delete the selection
		selection_delete(editor);
	// Update progress toward a new undo state if the current code is a space
	} else {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

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
	editor.stored_position = editor.cursor_position[0];
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Add an undo state and delete the selection
		selection_delete(editor);
	} else {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

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

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

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
	home_key(editor, true);
}

// Backspace at the beginning of line, moving to the above line
fn backspace_beginning_of_line(editor: &mut EditorSpace) {
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
fn backspace_normally(editor: &mut EditorSpace) {
	// Get the current editor state
	let state = editor.get_unredo_state();
	// Add a new unredo state if necessary
	editor.unredo_stack.auto_update(state, false);

	// Move left
	left_arrow(editor, true);
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Remove one character
	editor
		.blocks
		.as_mut()
		.unwrap()
		.delete_char_in_line(line_num, editor.text_position)
		.unwrap_or_else(|err| panic!("Couldn't delete char on line {} | {}", line_num, err));
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
			backspace_beginning_of_line(editor);
		// Otherwise, just move cursor left
		} else if editor.text_position != 0 {
			// Backspace normally, deleting one char
			backspace_normally(editor);
		}
	} else {
		// Add a new undo state and delete the selection
		selection_delete(editor);
	}
}

// Delete a character normally if there is no selection
fn no_selection_delete(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// The length of the current line
	let length = match editor.blocks.as_ref().unwrap().get_line_length(line_num) {
		Ok(len) => len,
		Err(err) => panic!("Couldn't get length of line {} | {}", line_num, err),
	};

	// If not at the end of the current line
	if editor.text_position < length {
		// Get the current editor state
		let state = editor.get_unredo_state();
		// Add a new unredo state if necessary
		editor.unredo_stack.auto_update(state, false);

		// Delete next char
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_char_in_line(line_num, editor.text_position)
			.unwrap_or_else(|err| panic!("Couldn't delete char on line {} | {}", line_num, err));

	// If not at end of last line
	} else if line_num < editor.file_length - 1 {
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
		// Reduce the overall file length
		editor.file_length -= 1;
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If there is no highlighted selection, delete normally
	if editor.selection.is_empty {
		// Delete character
		no_selection_delete(editor);
	} else {
		selection_delete(editor);
	}
}
