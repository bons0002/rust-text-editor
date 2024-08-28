/*
===========================================
			KEY FUNCTIONS TESTS
===========================================
*/

use super::*;
use key_functions::{highlight_selection::*, *};
use std::fs::{self, read_to_string};

/*
==================================
			SAVE TESTS
==================================
*/

/* Test saving the small file.
The small file ends in an empty line,
so this checks that that line gets saved. */
#[test]
#[ignore]
fn save_key_combo_small_file() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(SMALL_FILE) + "-debug-test-0");

	// Ensure that the metadata of the file is up to date
	editor.file.sync_all().unwrap();
	// Get the number of bytes in this file
	let original_file_length = editor.file.metadata().unwrap().len();

	// Write the file to a different debug file
	save_key_combo(&mut editor, true, debug_filename);

	// Get the lines of the original file
	let original_text = read_to_string(debug_filename).unwrap();
	let original_lines: Vec<String> = original_text.split('\n').map(String::from).collect();

	// Open the file in read-write mode
	let file = match OpenOptions::new()
		.read(true)
		.write(true)
		.open(debug_filename)
	{
		Ok(file) => file,
		Err(err) => panic!("{}", err),
	};

	// Ensure that the metadata of the file is up to date
	file.sync_all().unwrap();
	// Get the number of bytes in this file
	let debug_file_length = file.metadata().unwrap().len();

	// Check that the files are the same length
	assert_eq!(original_file_length, debug_file_length);

	// Get the lines of the debug file
	let debug_text = read_to_string(debug_filename).unwrap();
	let debug_lines: Vec<String> = debug_text.split('\n').map(String::from).collect();

	// Check that the lines of the file were saved correctly
	assert_eq!(original_lines, debug_lines);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

/* Test saving the GENOME_FILE.
This tests whether mutliple block length
files will be saved properly. */
#[test]
#[ignore]
fn save_key_combo_genome_file() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(GENOME_FILE) + "-debug-test-1");

	// Ensure that the metadata of the file is up to date
	editor.file.sync_all().unwrap();
	// Get the number of bytes in this file
	let original_file_length = editor.file.metadata().unwrap().len();

	// Write the file to a different debug file
	save_key_combo(&mut editor, true, debug_filename);

	// Get the lines of the original file
	let original_text = read_to_string(debug_filename).unwrap();
	let original_lines: Vec<String> = original_text.split('\n').map(String::from).collect();

	// Open the file in read-write mode
	let file = match OpenOptions::new()
		.read(true)
		.write(true)
		.open(debug_filename)
	{
		Ok(file) => file,
		Err(err) => panic!("{}", err),
	};

	// Ensure that the metadata of the file is up to date
	file.sync_all().unwrap();
	// Get the number of bytes in this file
	let debug_file_length = file.metadata().unwrap().len();

	// Check that the files are the same length
	assert_eq!(original_file_length, debug_file_length);

	// Get the lines of the debug file
	let debug_text = read_to_string(debug_filename).unwrap();
	let debug_lines: Vec<String> = debug_text.split('\n').map(String::from).collect();

	// Check that the lines of the file were saved correctly
	assert_eq!(original_lines, debug_lines);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

/* Test saving the highlight test file.
This file includes unicode characters.
Also, I just felt like having a test for each of the existing files.
Also tests repeated saves. */
#[test]
#[ignore]
fn save_key_combo_highlight_file() {
	// Make and editor for the HIGHLIGHT_FILE
	let mut editor = construct_editor(HIGHLIGHT_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(HIGHLIGHT_FILE) + "-debug-test-2");

	// Ensure that the metadata of the file is up to date
	editor.file.sync_all().unwrap();
	// Get the number of bytes in this file
	let original_file_length = editor.file.metadata().unwrap().len();

	/* Write the file to a different debug file.
	Also, test that repeated saves don't change anything or
	cause a panic. */
	save_key_combo(&mut editor, true, debug_filename);
	save_key_combo(&mut editor, true, debug_filename);
	save_key_combo(&mut editor, true, debug_filename);

	// Get the lines of the original file
	let original_text = read_to_string(debug_filename).unwrap();
	let original_lines: Vec<String> = original_text.split('\n').map(String::from).collect();

	// Open the file in read-write mode
	let file = match OpenOptions::new()
		.read(true)
		.write(true)
		.open(debug_filename)
	{
		Ok(file) => file,
		Err(err) => panic!("{}", err),
	};

	// Ensure that the metadata of the file is up to date
	file.sync_all().unwrap();
	// Get the number of bytes in this file
	let debug_file_length = file.metadata().unwrap().len();

	// Check that the files are the same length
	assert_eq!(original_file_length, debug_file_length);

	// Get the lines of the debug file
	let debug_text = read_to_string(debug_filename).unwrap();
	let debug_lines: Vec<String> = debug_text.split('\n').map(String::from).collect();

	// Check that the lines of the file were saved correctly
	assert_eq!(original_lines, debug_lines);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

// Test saving a modified small file
#[test]
#[ignore]
fn modified_small_file_save() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(SMALL_FILE) + "-debug-test-3");

	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor, true);
	backspace(&mut editor);
	// Move down three lines
	home_key(&mut editor, true);
	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor, true);
	backspace(&mut editor);

	// Write the file to a different debug file
	save_key_combo(&mut editor, true, debug_filename);

	// Get a vector of the lines saved to the debug file
	let saved_text = read_to_string(debug_filename).unwrap();
	let saved_content: Vec<String> = saved_text.split('\n').map(String::from).collect();

	// The expected lines of the debug file
	let expected_content: Vec<String> = MODIFIED_SMALL_SAVE_FILE
		.split('\n')
		.map(String::from)
		.collect();

	// Check that the modified blocks were saved correctly
	assert_eq!(saved_content, expected_content);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

// Test saving a modified large file
#[test]
#[ignore]
fn modified_large_file_save() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(GENOME_FILE) + "-debug-test-4");

	// Move down one line
	down_arrow(&mut editor);
	home_key(&mut editor, true);

	// Highlight down 312 lines
	for i in 0..312 {
		// Ensure that the Blocks are loaded correctly (every 50 iterations)
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		highlight_down(&mut editor);
	}
	// Delete the selection
	backspace(&mut editor);

	// Write the file to a different debug file
	save_key_combo(&mut editor, true, debug_filename);

	/* Check that the scroll offset is correct.
	The top line of the widget should be the second line
	(scroll offset = 1). */
	assert_eq!(editor.scroll_offset, 1);
	/* Check that the cursor's line is correct.
	Since the top line of the widget is the 2nd line,
	the cursor should be on the top line. */
	assert_eq!(editor.cursor_position[1], 0);

	// Get a vector of the lines saved to the debug file
	let saved_text = read_to_string(debug_filename).unwrap();
	let saved_content: Vec<String> = saved_text.split('\n').map(String::from).collect();

	// The expected lines of the debug file
	let expected_content: Vec<String> = MODIFIED_LARGE_SAVE_FILE
		.split('\n')
		.map(String::from)
		.collect();

	// Check that the modified blocks were saved correctly
	assert_eq!(saved_content, expected_content);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

// Test saving GENOME_FILE multiple times, each time editing the file
#[test]
#[ignore]
fn multiple_modifications_save() {
	// Make and editor for the GENOME_FILE
	let mut genome_editor = construct_editor(GENOME_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(GENOME_FILE) + "-debug-test-5");

	// Write the file to a different debug file
	save_key_combo(&mut genome_editor, true, debug_filename);

	// Create a new editor for this debug file
	let mut editor = construct_editor(debug_filename);

	// Move down one line
	down_arrow(&mut editor);
	home_key(&mut editor, true);

	// Highlight down 58 lines
	for i in 0..58 {
		// Ensure that the Blocks are loaded correctly (every 50 iterations)
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		highlight_down(&mut editor);
	}
	// Delete the selection
	backspace(&mut editor);
	// Write to the file in-place
	save_key_combo(&mut editor, false, debug_filename);

	// Move down one line
	down_arrow(&mut editor);
	home_key(&mut editor, true);

	// Highlight down 101 lines
	for i in 0..101 {
		// Ensure that the Blocks are loaded correctly (every 50 iterations)
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		highlight_down(&mut editor);
	}
	// Delete the selection
	backspace(&mut editor);
	// Write the file in-place
	save_key_combo(&mut editor, false, debug_filename);

	// Move down one line
	down_arrow(&mut editor);
	home_key(&mut editor, true);

	// Highlight down 151 lines
	for i in 0..151 {
		// Ensure that the Blocks are loaded correctly (every 50 iterations)
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		highlight_down(&mut editor);
	}
	// Delete the selection
	backspace(&mut editor);
	// Write the file in-place
	save_key_combo(&mut editor, false, debug_filename);

	// Get a vector of the lines saved to the debug file
	let saved_text = read_to_string(debug_filename).unwrap();
	let saved_content: Vec<String> = saved_text.split('\n').map(String::from).collect();

	// The expected lines of the debug file
	let expected_content: Vec<String> = MULTIPLE_MODIFICATIONS_SAVE
		.split('\n')
		.map(String::from)
		.collect();

	// Check that the modified blocks were saved correctly
	assert_eq!(saved_content, expected_content);

	// Delete the debug file
	fs::remove_file(debug_filename).unwrap();
}

/*
===================================
			ARROW TESTS
===================================
*/

// Use the right arrow key to move to the end of the file
#[test]
#[ignore]
fn move_right_through_entire_file() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move right through the entire file
	for _i in 0..26000 {
		right_arrow(&mut editor, true);
	}

	// Should be on last line
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	assert_eq!(line_num, 319);
}

#[test]
#[ignore]
fn move_left_through_entire_file() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move to the end of the file
	for _i in 0..330 {
		down_arrow(&mut editor);
	}
	end_key(&mut editor, true);

	// Move left through the entire file
	for _i in 0..26000 {
		left_arrow(&mut editor, true);
	}

	// Should be on last line
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	assert_eq!(line_num, 0);
}

/*
===========================================
			STORED CURSOR TESTS
===========================================
*/

// Test storing the cursor when moving right
#[test]
fn store_cursor_right() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);

	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	home_key(&mut editor, true);

	// Move right 30 times
	for _i in 0..30 {
		right_arrow(&mut editor, true);
	}
	// Move down for lines
	for i in 0..4 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 1),
			1 => assert_eq!(editor.cursor_position[0], 0),
			2 => assert_eq!(editor.cursor_position[0], 12),
			3 => {
				/* Check that the cursor ended in the correct location.
				It is at 33 not 30 because the line starts with a four wide tab,
				and the emoji is 2 wide. */
				assert_eq!(editor.cursor_position[0], 33);
			}
			_ => (),
		}
	}
}

// Test storing the cursor when moving left
#[test]
fn store_cursor_left() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);

	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	home_key(&mut editor, true);

	// Move right 30 times (and not storing cursor)
	for _i in 0..30 {
		right_arrow(&mut editor, false);
	}
	// Move left 8 times
	for _in in 0..8 {
		left_arrow(&mut editor, true);
	}
	// Move down 3 lines
	for i in 0..4 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 1),
			1 => assert_eq!(editor.cursor_position[0], 0),
			2 => {
				// Check that the cursor stops at the end of the line
				assert_eq!(editor.cursor_position[0], 12);
			}
			3 => {
				/* Check that the cursor ended in the correct location.
				It is at 25 not 22 because the line starts with a four wide tab. */
				assert_eq!(editor.cursor_position[0], 25);
			}
			_ => (),
		}
	}
}

// Test storing the cursor position for the home and end keys
#[test]
fn store_cursor_home_end() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);

	// Move to the end of the first line
	end_key(&mut editor, true);
	assert_eq!(editor.cursor_position[0], 17);

	// Move down two lines
	for i in 0..2 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 0),
			1 => assert_eq!(editor.cursor_position[0], 17),
			_ => (),
		}
	}
	// Move to the end of the line
	end_key(&mut editor, true);
	assert_eq!(editor.cursor_position[0], 18);
	// Move to the beginning of the line
	home_key(&mut editor, true);
	assert_eq!(editor.cursor_position[0], 0);

	// Move down four lines
	for _i in 0..4 {
		down_arrow(&mut editor);
		assert_eq!(editor.cursor_position[0], 0);
	}
}

/* The previous tests checked the cursor by moving down, this will check that
it works up as well. */
#[test]
fn store_cursor_up() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// Move to the end of the first line
	end_key(&mut editor, true);
	assert_eq!(editor.cursor_position[0], 17);

	// Move down five lines
	for i in 0..5 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 0),
			1 => assert_eq!(editor.cursor_position[0], 17),
			2 => assert_eq!(editor.cursor_position[0], 17),
			3 => assert_eq!(editor.cursor_position[0], 1),
			4 => assert_eq!(editor.cursor_position[0], 0),
			_ => (),
		}
	}
	// Move up five lines
	for i in 0..5 {
		up_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 1),
			1 => assert_eq!(editor.cursor_position[0], 17),
			2 => assert_eq!(editor.cursor_position[0], 17),
			3 => assert_eq!(editor.cursor_position[0], 0),
			4 => assert_eq!(editor.cursor_position[0], 17),
			_ => (),
		}
	}
}

// Test storing the cursor when moving right at the end of a line
#[test]
fn store_cursor_end_of_line_right() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// Move to the end of the first line
	end_key(&mut editor, true);

	// Move right six times (starting at the end of the line)
	for i in 0..6 {
		right_arrow(&mut editor, true);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 0),
			1 => assert_eq!(editor.cursor_position[0], 0),
			2 => assert_eq!(editor.cursor_position[0], 1),
			3 => assert_eq!(editor.cursor_position[0], 2),
			4 => assert_eq!(editor.cursor_position[0], 3),
			5 => assert_eq!(editor.cursor_position[0], 4),
			_ => (),
		}
	}
	// Move down four lines
	for i in 0..4 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 4),
			1 => assert_eq!(editor.cursor_position[0], 1),
			2 => assert_eq!(editor.cursor_position[0], 0),
			3 => assert_eq!(editor.cursor_position[0], 4),
			_ => (),
		}
	}
}

// Test storing the cursor when moving left at the beginning of a line
#[test]
fn store_cursor_beginning_of_line_left() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// Move down two lines
	for _i in 0..2 {
		down_arrow(&mut editor);
	}
	// Make sure at beginning of line
	home_key(&mut editor, true);

	// Move left twice
	for i in 0..2 {
		left_arrow(&mut editor, true);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 0),
			1 => assert_eq!(editor.cursor_position[0], 17),
			_ => (),
		}
	}
	// Move down three times
	for i in 0..3 {
		down_arrow(&mut editor);
		match i {
			0 => assert_eq!(editor.cursor_position[0], 0),
			1 => assert_eq!(editor.cursor_position[0], 17),
			2 => assert_eq!(editor.cursor_position[0], 17),
			_ => (),
		}
	}
}

/*
==============================================
			PAGE UP AND DOWN TESTS
==============================================
*/

// Test the page up key
#[test]
fn page_up_test() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move down 100 lines
	for _i in 0..100 {
		down_arrow(&mut editor);
	}
	// Page up twice
	for i in 0..3 {
		page_up(&mut editor);
		match i {
			// (Height of editor is only 48 here)
			0 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 52),
			1 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 4),
			2 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 0),
			_ => (),
		}
	}
}

// Test the page down key
#[test]
fn page_down_test() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);
	// Page down twice
	for i in 0..8 {
		page_down(&mut editor);
		match i {
			// (Height of editor is only 48 here)
			0 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 48),
			1 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 96),
			2 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 144),
			3 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 192),
			4 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 240),
			5 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 288),
			6 => assert_eq!(editor.get_line_num(editor.cursor_position[1]), 319),
			_ => (),
		}
	}
}

// Test inserting a new line at the end of the file
#[test]
fn end_of_file_new_line_insert() {
	// Make an editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// Move to the end of the file
	page_down(&mut editor);
	// Check that the cursor is on the last line
	assert_eq!(editor.get_line_num(editor.cursor_position[1]), 12);

	// Insert a new line
	enter_key(&mut editor);
	// Ensure the cursor is moved down
	down_arrow(&mut editor);
	// Check that the cursor moved to this new last line
	assert_eq!(editor.get_line_num(editor.cursor_position[1]), 13);
}

// Test deleting the last empty line of the file, then pressing enter
#[test]
fn end_of_file_delete_and_enter() {
	// Make an editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// Move to the end of the file
	page_down(&mut editor);
	// Delete the last line
	backspace(&mut editor);
	// Check that the cursor is on the last line
	assert_eq!(editor.get_line_num(editor.cursor_position[1]), 11);

	// Move left
	left_arrow(&mut editor, true);
	// Add a new line
	enter_key(&mut editor);
	// Move down to make sure on the last line
	down_arrow(&mut editor);
	// Check that the cursor is on the new last line
	assert_eq!(editor.get_line_num(editor.cursor_position[1]), 12);
}
