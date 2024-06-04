// Defines the logic of the movement keys that highlight selections of text.
// Also defines the Selection struct for track the highlighted selection.

use super::{
    down_arrow, end_key, home_key, left_arrow, right_arrow, up_arrow, Config, EditorSpace
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

// Keep track of which movement key is used
enum Movement {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	HOME,
	END,
}

impl Movement {
	// Uses a movement key based on the value of the enum
	pub fn take_movement(&self, editor: &mut EditorSpace, config: &Config) {
		match self {
			Self::UP => up_arrow(editor, config),
			Self::DOWN => down_arrow(editor, config),
			Self::LEFT => left_arrow(editor, config),
			Self::RIGHT => right_arrow(editor, config),
			Self::HOME => home_key(editor),
			Self::END => end_key(editor, config),
		};
	}
}

// Implement equality for the Movement enum
impl PartialEq for Movement {
	// Check whether the two enums are the same value
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UP, Self::UP) => true,
            (Self::DOWN, Self::DOWN) => true,
			(Self::LEFT, Self::LEFT) => true,
			(Self::RIGHT, Self::RIGHT) => true,
			(Self::HOME, Self::HOME) => true,
			(Self::END, Self::END) => true,
            _ => false,
        }
    }
}

// Shift + Right_Arrow highlights (or un-highlights) a selection of text as moving right
pub fn highlight_right(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, config, Movement::RIGHT);
	// Otherwise, add to the selection
	} else {
		// Move right and get location
		right_arrow(editor, config);
		let update = editor.pos;
		// If selection is now empty
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
		init_selection(editor, config, Movement::LEFT);
	// Otherwise, add to the selection
	} else {
		// Move left and get location
		left_arrow(editor, config);
		let update = editor.pos;
		// If selection is now empty
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
		init_selection(editor, config, Movement::UP);
	// Otherwise, add to the selection
	} else {
		// Store the current location
		let prior = editor.pos;
		// Move up and get location
		up_arrow(editor, config);
		let update = editor.pos;
		// If the selection is now empty (but not on first line)
		if update == editor.selection.start && update.1 > 0 {
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
		init_selection(editor, config, Movement::DOWN);
	// Otherwise, add to the selection
	} else {
		// Store the current location
		let prior = editor.pos;
		// Move down and get location
		down_arrow(editor, config);
		let update = editor.pos;
		// If the selection is now empty (but not on last line)
		if update == editor.selection.end && update.1 < editor.content.len() - 1 {
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

// Shift + End will highlight (or un-highlight) until the end of the line
pub fn highlight_end(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection.is_empty {
		init_selection(editor, config, Movement::END);
	// Otherwise, add to the selection
	} else {
		end_helper(editor, config);
	}
}

// The logic for the end_highlight if a selection already exists.
// It's its own function to improve readability.
fn end_helper(editor: &mut EditorSpace, config: &Config) {
	// Store the current location
	let prior = editor.pos;
	// Move to end of line and get location
	end_key(editor, config);
	let update = editor.pos;
	
	// If selection is now empty
	if update == editor.selection.end {
		// Reset selection
		editor.selection.is_empty = true;
	} else {
		// If only one line
		if editor.selection.start.1 == editor.selection.end.1 && update.1 == editor.selection.start.1 {
			// If cursor before selection
			if prior.0 <= editor.selection.start.0 {
				// Set start to original end
				editor.selection.start = editor.selection.original_pos;
				// Set end to end of line
				editor.selection.end = update;
			// Otherwise, update endpoint
			} else {
				editor.selection.end = update;
			}
		// If at last line
		} else if prior.1 >= editor.selection.end.1 {
			// Update end
			editor.selection.end = update;
		// If at first line
		} else {
			// Deselect start
			editor.selection.start = update;
		}
	}
}

// Initialize selection for right, down, or end
fn init_selection(editor: &mut EditorSpace, config: &Config, movement: Movement) {
	// Store the starting position of the cursor
	editor.selection.original_cursor = editor.cursor_pos;
	// Store the original starting position in the text
	editor.selection.original_pos = editor.pos;

	// Initialize highlighting forward
	if movement == Movement::RIGHT || movement == Movement::DOWN || movement == Movement::END {
		// Get the start point
		editor.selection.start = editor.pos;
		// Move
		movement.take_movement(editor, config);
		// Get endpoint
		editor.selection.end = editor.pos;
		// Flag selection as being not empty
		editor.selection.is_empty = false;
	// Initialize highlighting backwards
	} else {
		// Get endpoint
		editor.selection.end = editor.pos;
		// Move
		movement.take_movement(editor, config);
		// Get start point
		editor.selection.start = editor.pos;
		// Flag selection as being not empty
		editor.selection.is_empty = false;
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

	// Filename for tests
	const HIGHLIGHT_VERTICAL: &str = "../editor/test_files/highlight_vertical.txt";

	// ----------------------
	// Highlight Right Tests
	// ----------------------

	// Test highlighting 3 characters to the right
	#[test]
	fn highlight_right_3_chars() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
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
		let filename = String::from(HIGHLIGHT_VERTICAL);
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
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

		assert_eq!(selected_string, "56789abcd");
	}

	// ---------------------
	// Highlight Left Tests
	// ---------------------

	// Test highlighting 3 characters to the left
	#[test]
	fn highlight_left_3_chars() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
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
		let filename = String::from(HIGHLIGHT_VERTICAL);
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
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);
		
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
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 1]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

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
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

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
		// Check that the content of the highlighted section is correct
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 1]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

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
		// Check that the content of the highlighted section is correct
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 1]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

		assert_eq!(selected_string, "opqr987654321+_)=-");
	}

	// --------------------
	// Highlight End Tests
	// --------------------

	// Select (and deselect) until end of line on one line
	#[test]
	fn highlight_end_one_line_select() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.set_starting_pos((0, 0), 100, 6);
		editor.pos = (3, 1);
		editor.selection.original_pos = (6, 1);
		editor.selection.start = (3, 1);
        editor.selection.end = (6, 1);
        editor.selection.is_empty = false;

		// Highlight until the end of the line
		highlight_selection::highlight_end(&mut editor, &config);

		// Check selection bounds
		assert_eq!(editor.selection.start, (6, 1));
		assert_eq!(editor.selection.end, (9, 1));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.selection.start.1]
			[(editor.selection.start.0)..(editor.selection.end.0)];
		assert_eq!(selected_string, "ghi");
	}

	// Select until end of line on last line of multiline selection
	#[test]
	fn highlight_end_multiline_last_select() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.set_starting_pos((0, 0), 100, 6);
		editor.pos = (4, 3);
		editor.selection.original_pos = (4, 1);
		editor.selection.start = (4, 1);
        editor.selection.end = (4, 3);
        editor.selection.is_empty = false;

		// Highlight until the end of the line
		highlight_selection::highlight_end(&mut editor, &config);

		// Check selection bounds
		assert_eq!(editor.selection.start, (4, 1));
		assert_eq!(editor.selection.end, (9, 3));

		// Check that the content of the highlighted section is correct
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 1]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

		assert_eq!(selected_string, "efghi!@#$%^&*(jklmnopqr");
	}

	// Deselect until end of line on first line of multiline selection
	#[test]
	fn highlight_end_multiline_first_deselect() {
		let config = Config::default();
		let filename = String::from(HIGHLIGHT_VERTICAL);
		let mut editor = EditorSpace::new(filename, &config);

		// Set starting selection
		editor.set_starting_pos((0, 0), 100, 6);
		editor.pos = (4, 1);
		editor.selection.original_pos = (4, 5);
		editor.selection.start = (4, 1);
        editor.selection.end = (4, 5);
        editor.selection.is_empty = false;

		// Highlight until the end of the line
		highlight_selection::highlight_end(&mut editor, &config);

		// Check selection bounds
		assert_eq!(editor.selection.start, (9, 1));
		assert_eq!(editor.selection.end, (4, 5));

		// Check that the content of the highlighted section is correct
		let mut selected_string = String::from(&editor.content[editor.selection.start.1][editor.selection.start.0..]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 1]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 2]);
		selected_string.push_str(&editor.content[editor.selection.start.1 + 3]);
		selected_string.push_str(&editor.content[editor.selection.end.1][..editor.selection.end.0]);

		assert_eq!(selected_string, "!@#$%^&*(jklmnopqr987654321+_)=");
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

