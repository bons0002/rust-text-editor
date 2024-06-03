// Defines the logic of the movement keys that highlight selections of text.
// Also defines the Selection struct for track the highlighted selection.

use super::{
    Config,
    EditorSpace,
    down_arrow,
    left_arrow,
    right_arrow,
    up_arrow,
};

// Structure that keeps track of the highlighted selection of text
pub struct Selection {
    // The start point of the selection
    pub start: (usize, usize),
    // The endpoint of the selection
    pub end: (usize, usize),
    // Flag to track if selection is empty or not
    pub is_empty: bool,
	// Store the start point of the cursor
	pub original_cursor: (usize, usize),
	// Store the start point of the text position
	pub original_pos: (usize, usize),
}

impl Selection {
    // Create a new Selection struct
    pub fn new() -> Self {
        Selection {
            start: (0, 0),
            end: (0, 0),
            is_empty: true,
			original_cursor: (0, 0),
			original_pos: (0, 0),
        }
    }
}

// Shift + Right_Arrow highlights (or un-highlights) a selection of text as moving right
pub fn highlight_right(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		// Store the starting position of the cursor
		editor.selection.original_cursor = editor.cursor_pos;
		// Store the original starting position in the text
		editor.selection.original_pos = editor.pos;

		// Get the start point
		editor.selection.start = editor.pos;
		// Move right
		right_arrow(editor, config);
		// Get endpoint
		editor.selection.end = editor.pos;
		// Flag selection as being not empty
        editor.selection.is_empty = false;
	} else {
		// Move right and get location
		right_arrow(editor, config);
		let update = editor.pos;
		// If last char
		if update == editor.selection.end {
			// Reset selection
			editor.selection.is_empty = true;
		// If after on last line
		} else if update.1 >= editor.selection.end.1 {
			// If before end of selection on last line
			if update.0 < editor.selection.end.0 && update.1 == editor.selection.end.1 {
				// Deselect
				editor.selection.start = update;
			// Otherwise, add to the front of the selection
			} else {
				editor.selection.end = update;
			}
		// If not on last line
		} else {
			editor.selection.start = update;
		}
	}
}

// Shift + Left_Arrow highlights (or un-highlights) a selection of text as moving left
pub fn highlight_left(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		// Store the starting position of the cursor
		editor.selection.original_cursor = editor.cursor_pos;
		// Store the original starting position in the text
		editor.selection.original_pos = editor.pos;

		// Get endpoint
		editor.selection.end = editor.pos;
		// Move left
		left_arrow(editor, config);
		// Get start point
		editor.selection.start = editor.pos;
		// Flag selection as being not empty
        editor.selection.is_empty = false;
	} else {
		// Move left and get location
		left_arrow(editor, config);
		let update = editor.pos;
		// If last char
		if update == editor.selection.start {
			// Reset selection
			editor.selection.is_empty = true;
		// If before or on first line
		} else if update.1 <= editor.selection.start.1 {
			// If after beginning of selection on first line
			if update.0 > editor.selection.start.0 && update.1 == editor.selection.start.1 {
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

// Shift + Up_Arrow highlights (or un-highlights) lines above in the text
pub fn highlight_up(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		// Store the starting position of the cursor
		editor.selection.original_cursor = editor.cursor_pos;
		// Store the original starting position in the text
		editor.selection.original_pos = editor.pos;

		// Get endpoint
		editor.selection.end = editor.pos;
		// Move up
		up_arrow(editor, config);
		// Get start point
		editor.selection.start = editor.pos;
		// Flag selection as being not empty
        editor.selection.is_empty = false;
	} else {
		// Store the current location
		let prior = editor.pos;
		// Move up and get location
		up_arrow(editor, config);
		let update = editor.pos;
		// If the selection is now empty
		if update == editor.selection.start {
			// Reset selection
			editor.selection.is_empty = true;
		// If moving at the beginning of the selection
		} else if prior.1 <= editor.selection.start.1 {
			// Update the beginning of the selection
			editor.selection.start = update;
		// If moving at the end of the selection
		} else {
			// Deselect from the end
			editor.selection.end = update;
		}
	}
}

// Shift + Down_Arrow highlights (or un-highlights) lines below in the text
pub fn highlight_down(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		// Store the starting position of the cursor
		editor.selection.original_cursor = editor.cursor_pos;
		// Store the original starting position in the text
		editor.selection.original_pos = editor.pos;

		// Get start point
		editor.selection.start = editor.pos;
		// Move down
		down_arrow(editor, config);
		// Get endpoint
		editor.selection.end = editor.pos;
		// Flag selection as being not empty
        editor.selection.is_empty = false;
	} else {
		// Store the current location
		let prior = editor.pos;
		// Move down and get location
		down_arrow(editor, config);
		let update = editor.pos;
		// If the selection is now empty
		if update == editor.selection.end {
			// Reset selection
			editor.selection.is_empty = true;
		// If moving at the end of the selection
		} else if prior.1 >= editor.selection.end.1 {
			// Update the end of the selection
			editor.selection.end = update;
		// If moving at the beginning of the selection
		} else {
			// Deselect from the beginning
			editor.selection.start = update;
		}
	}
}


// -----------
// Unit Tests
// -----------
#[cfg(test)]
mod tests {
	use crate::editor::key_functions::highlight_selection;
	use crate::editor::EditorSpace;
	use config::config::Config;

	// Filenames for tests
	const HIGHLIGHT_HORIZONTAL: &str = "../editor/test_files/highlight_horizontal.txt";
	const HIGHLIGHT_VERTICAL: &str = "../editor/test_files/highlight_vertical.txt";

	// ----------------------
	// Highlight Right Tests
	// ----------------------

	// Test highlighting 3 characters to the right
	#[test]
	fn highlight_right_3_chars() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_HORIZONTAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((0, 0), 100, 100);
		editor.pos = (0, 1);

		// Select 3 characters
		for _i in 0..3 {
			highlight_selection::highlight_right(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (0, 1));
        assert_eq!(editor.selection.end, (3, 1));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.start).0..(editor.selection.end).0];
		assert_eq!(selected_string, "abc");
	}

	// Test whether highlight_right correctly wraps to the second line
	#[test]
	fn highlight_right_wrap() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_HORIZONTAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((100, 100), 100, 100);
		editor.pos = (4, 0);

		// Select 10 characters
		for _i in 0..10 {
			highlight_selection::highlight_right(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (4, 0));
        assert_eq!(editor.selection.end, (4, 1));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let selected_string_2 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(selected_string_1);
		selected_string.push_str(selected_string_2);

		assert_eq!(selected_string, "56789abcd");
	}

	// ---------------------
	// Highlight Left Tests
	// ---------------------

	// Test highlighting 3 characters to the left
	#[test]
	fn highlight_left_3_chars() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_HORIZONTAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((100, 100), 100, 100);
		editor.pos = (4, 0);

		// Select 3 characters
		for _i in 0..3 {
			highlight_selection::highlight_left(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (1, 0));
        assert_eq!(editor.selection.end, (4, 0));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.start).0..(editor.selection.end).0];
		assert_eq!(selected_string, "234");
	}

	// Test whether highlight_left correctly wraps to the first line
	#[test]
	fn highlight_left_wrap() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_HORIZONTAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting pos in text
		editor.set_starting_pos((1, 1), 10, 2);
		editor.cursor_pos = (4, 4);	// Arbitrary, just needs to satisfy `up_arrow` wrapping condition
		editor.pos = (4, 1);

		// Select 10 characters
		for _i in 0..10 {
			highlight_selection::highlight_left(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (4, 0));
        assert_eq!(editor.selection.end, (4, 1));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let selected_string_2 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(selected_string_1);
		selected_string.push_str(selected_string_2);
		
		assert_eq!(selected_string, "56789abcd");
	}

	// -------------------
	// Highlight Up Tests
	// -------------------

	// Test highlight_up for selecting two lines up
	#[test]
	fn highlight_up_select_two_lines() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting position
		editor.set_starting_pos((1,1), 9, 6);
		editor.cursor_pos = (5,7);
		editor.pos = (5,3);

		// Select 2 lines
		for _i in 0..2 {
			highlight_selection::highlight_up(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (5, 1));
        assert_eq!(editor.selection.end, (5, 3));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let temp_2 = &editor.content[editor.selection.start.1 + 1];
		let temp_3 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(temp_1);
		selected_string.push_str(temp_2);
		selected_string.push_str(temp_3);

		assert_eq!(selected_string, "fghi!@#$%^&*(jklmn");
	}

	// Test highlight_up deselect two lines
	#[test]
	fn highlight_up_deselect_one_lines() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting position
		editor.set_starting_pos((1,1), 9, 6);
		editor.cursor_pos = (5,7);
		editor.pos = (5,3);
		// Set starting selection
		editor.selection.start = (5, 1);
        editor.selection.end = (5, 3);
        editor.selection.is_empty = false;

		// Deselect one line
		highlight_selection::highlight_up(&mut editor, &config);

		// Check selection bounds
		assert_eq!(editor.selection.start, (5, 1));
        assert_eq!(editor.selection.end, (5, 2));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let temp_2 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(temp_1);
		selected_string.push_str(temp_2);

		assert_eq!(selected_string, "fghi!@#$%");
	}

	// ---------------------
	// Highlight Down Tests
	// ---------------------

	// Test highlight_down for selecting two lines down
	#[test]
	fn highlight_down_select_two_lines() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting position
		editor.set_starting_pos((1,1), 9, 6);
		editor.pos = (5,3);

		// Select 2 lines
		for _i in 0..2 {
			highlight_selection::highlight_down(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (5, 3));
        assert_eq!(editor.selection.end, (5, 5));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let temp_2 = &editor.content[editor.selection.start.1 + 1];
		let temp_3 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(temp_1);
		selected_string.push_str(temp_2);
		selected_string.push_str(temp_3);

		assert_eq!(selected_string, "opqr987654321+_)=-");
	}

	// Test highlight_down for deslecting three lines down
	#[test]
	fn highlight_down_deselect_three_lines() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting position
		editor.set_starting_pos((1,1), 9, 6);
		editor.pos = (5, 0);
		// Set starting selection
		editor.selection.start = (5, 0);
        editor.selection.end = (5, 5);
        editor.selection.is_empty = false;

		// Deselect three lines
		for _i in 0..3 {
			highlight_selection::highlight_down(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection.start, (5, 3));
        assert_eq!(editor.selection.end, (5, 5));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.start.1][editor.selection.start.0..];
		let temp_2 = &editor.content[editor.selection.start.1 + 1];
		let temp_3 = &editor.content[editor.selection.end.1][..editor.selection.end.0];
		let mut selected_string = String::from(temp_1);
		selected_string.push_str(temp_2);
		selected_string.push_str(temp_3);

		assert_eq!(selected_string, "opqr987654321+_)=-");
	}

	// ----------------------------------
	// Delete Highlighted Selection Test
	// ----------------------------------

	// Delete two (highlighted) lines down from the start
	#[test]
	fn delete_selection_down_two_lines() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.selection.start = (5, 0);
        editor.selection.end = (5, 3);
        editor.selection.is_empty = false;

		// Delete the selection
		editor.delete_selection();

		// Check that the selection is empty
		assert!(editor.selection.is_empty);

		// Check the content of the text after deletion
		let mut remaining_text = String::new();
		for line in editor.content {
			remaining_text.push_str(line.as_str());
		}

		assert_eq!(remaining_text, "12345opqr987654321+_)=-\\,./")
	}

	// Test deleting starting from the beginning of a line
	#[test]
	fn delete_at_beginning_of_line() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.selection.start = (0, 0);
        editor.selection.end = (5, 3);
        editor.selection.is_empty = false;

		// Delete the selection
		editor.delete_selection();

		// Check that the selection is empty
		assert!(editor.selection.is_empty);

		// Check the content of the text after deletion
		let mut remaining_text = String::new();
		for line in editor.content {
			remaining_text.push_str(line.as_str());
		}

		assert_eq!(remaining_text, "opqr987654321+_)=-\\,./");
	}

	// Test deleting starting from the beginning of a line
	#[test]
	fn delete_at_end_of_line() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.selection.start = (9, 0);
        editor.selection.end = (6, 4);
        editor.selection.is_empty = false;

		// Delete the selection
		editor.delete_selection();

		// Check that the selection is empty
		assert!(editor.selection.is_empty);

		// Check the content of the text after deletion
		let mut remaining_text = String::new();
		for line in editor.content {
			remaining_text.push_str(line.as_str());
		}

		assert_eq!(remaining_text, "123456789321+_)=-\\,./");
	}
}

