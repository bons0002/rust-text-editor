use super::{
	navigation_keys::{down_arrow, end_key, home_key, left_arrow, right_arrow, up_arrow},
	EditorSpace,
};

// Module containing direction keys to track movement
mod movement;
use movement::Movement;
// The struct containing the highlighted selection
pub mod selection;
// Subroutines for the highlighting functions
mod highlight_subroutines;

// Highlight (or un-highlight) to the left of the cursor
pub fn highlight_left(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::Left);
	// Otherwise, add to the existing selection
	} else {
		highlight_subroutines::left_subroutine(editor);
	}
}

// Highlight (or un-highlight) to the right of the cursor
pub fn highlight_right(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::Right);
	// Otherwise, add to the existing selection
	} else {
		highlight_subroutines::right_subroutine(editor);
	}
}

// Highlight (or un-highlight) up to the cursor position on the below line
pub fn highlight_down(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::Down);
	// Otherwise, add to the existing selection
	} else {
		// Update the selection boundaries
		highlight_subroutines::down_subroutine(editor);
	}
}

// Highlight (or un-highlight) up to the cursor position on the above line
pub fn highlight_up(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::Up);
	// Otherwise, add to the existing selection
	} else {
		// Update the selection boundaries
		highlight_subroutines::up_subroutine(editor);
	}
}

// Highlight (or un-highlight) to the beginning of the line
pub fn highlight_home(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::Home);
	// Otherwise, add to the existing selection
	} else {
		// Highlight to the beginning of the line
		highlight_subroutines::home_subroutine(editor);
	}
}

// Highlight (or un-highlight) to the end of the line
pub fn highlight_end(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::End);
	// Otherwise, add to the existing selection
	} else {
		// Highlight to the end of the line
		highlight_subroutines::end_subroutine(editor);
	}
}

// Highlight (or un-highlight) one page up
pub fn highlight_page_up(editor: &mut EditorSpace) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		highlight_subroutines::init_selection(editor, Movement::PageUp);
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
		highlight_subroutines::init_selection(editor, Movement::PageDown);
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
