/*
===========================================
			KEY FUNCTIONS TESTS
===========================================

All of these test are run serially because they share
files.
*/

use super::*;
use key_functions::{
	backspace, down_arrow, highlight_selection::highlight_down, home_key, save_key_combo,
};
use serial_test::serial;
use std::fs::{self, read_to_string};

/* Test saving the small file.
The small file ends in an empty line,
so this checks that that line gets saved. */
#[test]
#[ignore]
#[serial]
fn save_key_combo_small_file() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(SMALL_FILE) + "-debug");

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
#[serial]
fn save_key_combo_genome_file() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(GENOME_FILE) + "-debug");

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
#[serial]
fn save_key_combo_highlight_file() {
	// Make and editor for the HIGHLIGHT_FILE
	let mut editor = construct_editor(HIGHLIGHT_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(HIGHLIGHT_FILE) + "-debug");

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
#[serial]
fn modified_small_file_save() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(SMALL_FILE) + "-debug");

	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor);
	backspace(&mut editor);
	// Move down three lines
	home_key(&mut editor);
	// Move down three lines
	for _i in 0..3 {
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor);
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
#[serial]
fn modified_large_file_save() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);
	// The filename of the debug file
	let debug_filename = &(String::from(GENOME_FILE) + "-debug");

	// Move down one line
	down_arrow(&mut editor);
	home_key(&mut editor);

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
