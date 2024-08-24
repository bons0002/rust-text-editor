/*
==========================================================
			Blocks Tests (for 5KiB TextBlocks)
==========================================================
*/

use key_functions::{backspace, delete_key, down_arrow, end_key, enter_key, home_key, up_arrow};

use super::*;

// Test the construction of a Blocks
#[test]
fn blocks_construction() {
	// Make and editor for the GENOME_FILE
	let editor = construct_editor(GENOME_FILE);

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Vector of the lines of the GENOME_BLOCK_1 constant
	let expected_content: Vec<&str> = GENOME_BLOCK_1.split('\n').collect();

	// Check that these are equal
	assert_eq!(actual_content, expected_content);
}

// Test the construction of a Blocks from a small (smaller than widget) file
#[test]
fn small_blocks_construction() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Vector of the lines of the SMALL_FILE_BLOCK constant
	let expected_content: Vec<&str> = SMALL_FILE_BLOCK.split('\n').collect();

	// Check that these are equal
	assert_eq!(actual_content, expected_content);

	// Move to the end of the file to ensure can't move beyond (or load in empty blocks)
	for i in 0..200 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Check that these are equal still
	assert_eq!(actual_content, expected_content);
}

// Test pushing to the tail (and popping from the head) of the Blocks when moving downward
#[test]
fn push_tail_blocks() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move to the end of the file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Construct vectors for each expected block
	let results = vec![
		GENOME_BLOCK_3.split('\n'),
		GENOME_BLOCK_4.split('\n'),
		GENOME_BLOCK_5.split('\n'),
	];

	// Create one vector of all expected lines
	let mut expected_content = Vec::new();
	for block in results {
		expected_content.extend(block);
	}

	// Check the Blocks is accurate
	assert_eq!(actual_content, expected_content);
}

/* Test pushing to the head (and popping from the tail).
This function first moves to the end of the GENOME_FILE which
should pop the head blocks (as long as the above test succeeds); it
then moves back to the front of the file which should pop the tail blocks.
Therefore, this test also inadvertently tests moving back and forth within
the text.
DEPENDENCY: push_tail_blocks test */
#[test]
fn push_head_blocks() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move to the end of the file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}
	// Move to the beginning of the file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		up_arrow(&mut editor);
	}

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Construct vectors for each expected block
	let results = vec![
		GENOME_BLOCK_1.split('\n'),
		GENOME_BLOCK_2.split('\n'),
		GENOME_BLOCK_3.split('\n'),
	];

	// Create one vector of all expected lines
	let mut expected_content = Vec::new();
	for block in results {
		expected_content.extend(block);
	}

	// Check the Blocks is accurate
	assert_eq!(actual_content, expected_content);
}

// Test that the length of the Blocks is calculated correctly
#[test]
fn blocks_length() {
	// Make and editor for the GENOME_FILE
	let mut genome_editor = construct_editor(GENOME_FILE);

	// Move to the end of the file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = genome_editor.get_paragraph();
		}
		down_arrow(&mut genome_editor);
	}

	// Get the length of the genome Blocks
	let actual_genome_length = genome_editor.blocks.as_ref().unwrap().len();
	// Check that it calculated the correct length (193)
	assert_eq!(actual_genome_length, 193);

	// Test with a different file
	// Make and editor for the SMALL_FILE
	let small_editor = construct_editor(SMALL_FILE);

	let actual_small_length = small_editor.blocks.as_ref().unwrap().len();
	// Check that it calculated the correct length (12)
	assert_eq!(actual_small_length, 13);
}

// Test that the length of the blocks updates correctly when being modified
#[test]
fn modified_blocks_length() {
	// Make and editor for the SMALL_FILE
	let mut editor = construct_editor(SMALL_FILE);

	// Move to the end of the file
	for _i in 0..20 {
		down_arrow(&mut editor);
	}

	// Deletion sequence
	home_key(&mut editor);
	backspace(&mut editor);
	home_key(&mut editor);
	backspace(&mut editor);

	// Get the length after the deletion sequence
	let first_length = editor.blocks.as_ref().unwrap().len();
	// Should be 10
	assert_eq!(first_length, 11);

	// Insertion sequence

	for _i in 0..3 {
		up_arrow(&mut editor);
	}
	for _i in 0..3 {
		enter_key(&mut editor);
	}
	for _i in 0..5 {
		up_arrow(&mut editor);
	}
	enter_key(&mut editor);

	// Get the length after the insertion sequence
	let second_length = editor.blocks.as_ref().unwrap().len();
	// Should be 14
	assert_eq!(second_length, 15);
}

// Test deleting all lines of a file starting from the end
#[test]
#[ignore]
fn end_of_file_deletion() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move to the end of the file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}
	end_key(&mut editor);

	// Delete all text in the file
	for i in 0..26000 {
		// Ensure that the blocks are fully updated every 1000 iterations
		if i % 1000 == 0 {
			let _ = editor.get_paragraph();
		}
		backspace(&mut editor);
	}

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Check that the file is blank
	assert_eq!(actual_content, vec![""]);
}

// Test deleting all lines of a file starting from the beginning
#[test]
#[ignore]
fn beginning_of_file_deletion() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Delete all text in the file
	for i in 0..26000 {
		// Ensure that the blocks are fully updated every 1000 iterations
		if i % 1000 == 0 {
			let _ = editor.get_paragraph();
		}
		delete_key(&mut editor);
	}

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Check that the file is blank
	assert_eq!(actual_content, vec![""]);
}

// Test deleting disjointed lines at different places
#[test]
fn disjointed_deletion() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Move down 20 lines
	for i in 0..20 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor);
	backspace(&mut editor);

	// Move to end of file
	for i in 0..310 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		down_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor);
	backspace(&mut editor);

	// Move up 100 lines
	for i in 0..100 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		up_arrow(&mut editor);
	}
	// Delete line
	home_key(&mut editor);
	backspace(&mut editor);

	// Verify that three lines have been deleted (and no panic occurred)
	let length = editor.blocks.as_ref().unwrap().len();
	assert_eq!(length, 317);
}
