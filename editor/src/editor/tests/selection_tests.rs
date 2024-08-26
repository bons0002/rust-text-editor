/*
=================================================
			Highlight Selection Tests
=================================================
*/

use key_functions::{
	backspace, delete_key, down_arrow, end_key,
	highlight_selection::{
		highlight_down, highlight_end, highlight_home, highlight_page_down, highlight_right,
		highlight_up,
	},
	right_arrow,
};

use super::*;

// Test deleting a selection of text on a single line
#[test]
fn single_line_selection_deletion() {
	// Make and editor for the HIGHLIGHT_FILE
	let mut editor = construct_editor(HIGHLIGHT_FILE);

	// Go down two lines
	for _i in 0..2 {
		down_arrow(&mut editor);
	}
	// Highlight six characters to the right
	for _i in 0..6 {
		highlight_right(&mut editor);
	}
	// Delete this highlighted selection
	editor.delete_selection();

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());

	// Vector of the lines of the SINGLE_LINE_SELECTION_DELETION constant
	let expected_content: Vec<&str> = SINGLE_LINE_SELECTION_DELETION.split('\n').collect();

	// Check that the deletion was performed correctly
	assert_eq!(actual_content, expected_content);
}

// Test deleting a selection that spans multiple lines of text
#[test]
fn mutli_line_selection_deletion() {
	// Make and editor for the HIGHLIGHT_FILE
	let mut editor = construct_editor(HIGHLIGHT_FILE);

	// Move down
	down_arrow(&mut editor);
	// Move right three chars
	for _i in 0..3 {
		right_arrow(&mut editor, true);
	}
	// Highlight down three lines
	for _i in 0..3 {
		highlight_down(&mut editor);
	}
	// Highlight right two chars
	for _i in 0..2 {
		highlight_right(&mut editor);
	}
	// Delete this selection
	editor.delete_selection();

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Vector of the lines of the MULTI_LINE_SELECTION_DELETION constant
	let expected_content: Vec<&str> = MULTI_LINE_SELECTION_DELETION.split('\n').collect();

	// Check that the deletion was performed correctly
	assert_eq!(actual_content, expected_content);
}

// Test deleting a selection over mutliple Blocks (from front to back)
#[test]
#[ignore]
fn multi_block_selection_deletion_front_to_back() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Highlight the entire file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		highlight_down(&mut editor);
	}
	highlight_end(&mut editor);
	// Delete the entire file (this time using the backspace key)
	backspace(&mut editor);

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Get the length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();

	// Check that the content of this file is now empty
	assert_eq!(actual_content, vec![""]);
	assert_eq!(actual_length, 1); // 1 empty line at the beginning
}

// Test deleting a selection over mutliple Blocks (from back to front)
#[test]
#[ignore]
fn multi_block_selection_deletion_back_to_front() {
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
	end_key(&mut editor, true);

	// Highlight the entire file
	for i in 0..330 {
		// Ensure that the blocks are fully updated every 50 iterations
		if i % 50 == 0 {
			let _ = editor.get_paragraph();
		}
		highlight_up(&mut editor);
	}
	highlight_home(&mut editor);
	// Delete the entire file (this time using the delete key)
	delete_key(&mut editor);

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Get the length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();

	// Check that the content of this file is now empty
	assert_eq!(actual_content, vec![""]);
	assert_eq!(actual_length, 1); // 1 empty line at the beginning
}

// Test repeatedly deleting selections from the file
#[test]
#[ignore]
fn repeated_selection_deletion() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Repeatedly delete 50 lines
	for _i in 0..7 {
		// Highlight 50 lines down
		for j in 0..50 {
			// Ensure that the blocks are fully updated every 50 iterations
			if j % 50 == 0 {
				let _ = editor.get_paragraph();
			}
			highlight_down(&mut editor)
		}
		// Delete the selection
		editor.delete_selection();
	}
	// Delete the last line
	highlight_end(&mut editor);
	editor.delete_selection();

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Get the length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();

	// Check that the content of this file is now empty
	assert_eq!(actual_content, vec![""]);
	assert_eq!(actual_length, 1); // 1 empty line at the beginning
}

// Test highlighting with the page down key
#[test]
#[ignore]
fn page_down_selection() {
	// Make and editor for the GENOME_FILE
	let mut editor = construct_editor(GENOME_FILE);

	// Highlight the whole file
	for i in 0..10 {
		if i % 2 == 0 {
			editor.get_paragraph();
		}
		highlight_page_down(&mut editor);
	}
	highlight_end(&mut editor);
	// Delete the selection
	editor.delete_selection();

	// The experimental contents of the Blocks
	let actual_content = get_content(editor.blocks.as_ref().unwrap().clone());
	// Check that the file is empty
	assert_eq!(actual_content, vec![""]);
}
