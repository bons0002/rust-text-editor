// Defines the logic of the movement keys that highlight selections of text

use super::{
    Config,
    EditorSpace,
    down_arrow,
    left_arrow,
    right_arrow,
    up_arrow,
};

// Shift + Right_Arrow highlights (or un-highlights) a selection of text as moving right
pub fn highlight_right(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get the start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move right
		right_arrow(editor, config);
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Move right and get location
		right_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If last char
		if update == editor.selection.1 {
			// Reset selection
			editor.selection = ((-1, -1), (-1, -1));
		// If after on last line
		} else if update.1 >= editor.selection.1.1 {
			// If before end of selection on last line
			if update.0 < editor.selection.1.0 && update.1 == editor.selection.1.1 {
				// Deselect
				editor.selection = (update, editor.selection.1);
			// Otherwise, add to the front of the selection
			} else {
				editor.selection = (editor.selection.0, update);
			}
		// If not on last line
		} else {
			editor.selection = (update, editor.selection.1);
		}
	}
}

// Shift + Left_Arrow highlights (or un-highlights) a selection of text as moving left
pub fn highlight_left(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move left
		left_arrow(editor, config);
		// Get start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Move left and get location
		left_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If last char
		if update == editor.selection.0 {
			// Reset selection
			editor.selection = ((-1, -1), (-1, -1));
		// If before or on first line
		} else if update.1 <= editor.selection.0.1 {
			// If after beginning of selection on first line
			if update.0 > editor.selection.0.0 && update.1 == editor.selection.0.1 {
				// Deselect
				editor.selection = (editor.selection.0, update);
			// Otherwise, add to the front of the selection
			} else {
				editor.selection = (update, editor.selection.1);
			}
		// If not on first line
		} else {
			editor.selection = (editor.selection.0, update);
		}
	}
}

// Shift + Up_Arrow highlights (or un-highlights) lines above in the text
pub fn highlight_up(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move up
		up_arrow(editor, config);
		// Get start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Store the current location
		let prior = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move up and get location
		up_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If the selection is now empty
		if update == editor.selection.0 {
			// Reset selection
			editor.selection = ((-1, -1), (-1, -1));
		// If moving at the beginning of the selection
		} else if prior.1 <= editor.selection.0.1 {
			// Update the beginning of the selection
			editor.selection = (update, editor.selection.1);
		// If moving at the end of the selection
		} else {
			// Deselect from the end
			editor.selection = (editor.selection.0, update);
		}
	}
}

// Shift + Down_Arrow highlights (or un-highlights) lines below in the text
pub fn highlight_down(editor: &mut EditorSpace, config: &Config) {
	// If there is no selection, initialize it
	if editor.selection == ((-1, -1), (-1, -1)) {
		// Get start point
		let start = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move down
		down_arrow(editor, config);
		// Get endpoint
		let end = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Initialize selection
		editor.selection = (start, end);
	} else {
		// Store the current location
		let prior = (editor.pos.0 as isize, editor.pos.1 as isize);
		// Move down and get location
		down_arrow(editor, config);
		let update = (editor.pos.0 as isize, editor.pos.1 as isize);
		// If the selection is now empty
		if update == editor.selection.1 {
			// Reset selection
			editor.selection = ((-1, -1), (-1, -1));
		// If moving at the end of the selection
		} else if prior.1 >= editor.selection.1.1 {
			// Update the end of the selection
			editor.selection = (editor.selection.0, update);
		// If moving at the beginning of the selection
		} else {
			// Deselect from the beginning
			editor.selection = (update, editor.selection.1);
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
		assert_eq!(editor.selection, ((0, 1), (3, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
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
		assert_eq!(editor.selection, ((4, 0), (4, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let selected_string_2 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
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
		assert_eq!(editor.selection, ((1, 0), (4, 0)));

		// Check that the content of the highlighted section is correct
		let selected_string = &editor.content[editor.pos.1]
			[(editor.selection.0).0 as usize..(editor.selection.1).0 as usize];
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
		assert_eq!(editor.selection, ((4, 0), (4, 1)));

		// Check that the content of the highlighted section is correct
		let selected_string_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let selected_string_2 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
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
		assert_eq!(editor.selection, ((5, 1), (5, 3)));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let temp_2 = &editor.content[editor.selection.0.1 as usize + 1];
		let temp_3 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
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
		editor.selection = ((5, 1), (5, 3));

		// Deselect one line
		highlight_selection::highlight_up(&mut editor, &config);

		// Check selection bounds
		assert_eq!(editor.selection, ((5, 1), (5, 2)));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let temp_2 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
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
		assert_eq!(editor.selection, ((5, 3), (5, 5)));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let temp_2 = &editor.content[editor.selection.0.1 as usize + 1];
		let temp_3 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
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
		editor.selection = ((5, 0), (5, 5));

		// Deselect three lines
		for _i in 0..3 {
			highlight_selection::highlight_down(&mut editor, &config);
		}

		// Check selection bounds
		assert_eq!(editor.selection, ((5, 3), (5, 5)));

		// Check that the content of the highlighted section is correct
		let temp_1 = &editor.content[editor.selection.0.1 as usize][editor.selection.0.0 as usize..];
		let temp_2 = &editor.content[editor.selection.0.1 as usize + 1];
		let temp_3 = &editor.content[editor.selection.1.1 as usize][..editor.selection.1.0 as usize];
		let mut selected_string = String::from(temp_1);
		selected_string.push_str(temp_2);
		selected_string.push_str(temp_3);

		assert_eq!(selected_string, "opqr987654321+_)=-");
	}
}

