/*
=================================================
			Highlight Selection Tests
=================================================
*/

use super::*;

// Test the delete_selection funciton on a mutliline selection
#[test]
fn delete_selection_multiline() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(HIGHLIGHT_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	key_functions::highlight_selection::highlight_down(&mut editor);
	key_functions::highlight_selection::highlight_right(&mut editor);
	editor.delete_selection();

	// Create a vector of all the lines in the first three blocks
	let content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	// What the text should look like
	let expected_text: &str = "bcdefghi\
		!@#$%🥹^&*(\
		jklmnopqr\
		987654321\
		+_)=-\\🥹,./";

	assert_eq!(actual_text, expected_text);
}

// Test the delete_selection function on a oneline selection
#[test]
fn delete_selection_oneline() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(HIGHLIGHT_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	// Move right 3 times
	for _i in 0..3 {
		key_functions::right_arrow(&mut editor);
	}
	// Highlight 5 chars
	for _i in 0..5 {
		key_functions::highlight_selection::highlight_right(&mut editor);
	}
	// Delete the selection
	editor.delete_selection();

	// Create a vector of all the lines in the first three blocks
	let content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	// What the text should look like
	let expected_text: &str = "1239🥹\
		abcdefghi\
		!@#$%🥹^&*(\
		jklmnopqr\
		987654321\
		+_)=-\\🥹,./";

	assert_eq!(actual_text, expected_text);
}

// Delete a selection that ends at the very end of the small file
#[test]
fn delete_end_selection_small_file() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(HIGHLIGHT_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	// Move to the end of the file
	for _i in 0..20 {
		key_functions::down_arrow(&mut editor);
		key_functions::right_arrow(&mut editor);
	}

	// Highlight left 5 characters
	for _i in 0..5 {
		key_functions::highlight_selection::highlight_left(&mut editor);
	}
	// Highlight up 2 lines
	for _i in 0..2 {
		key_functions::highlight_selection::highlight_up(&mut editor);
	}

	// Delete this selection
	editor.delete_selection();

	// Create a vector of all the lines in the first three blocks
	let content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	// What the text should look like
	let expected_text = "123456789🥹\
        abcdefghi\
        !@#$%🥹^&*(\
        jklmn";

	assert_eq!(actual_text, expected_text);
}

// Delete a selection at the end of a large (multi-block) file
#[test]
fn delete_end_selection_large_file() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 200, 50);

	// Move to the end of the file
	for _i in 0..3000 {
		key_functions::down_arrow(&mut editor);
		key_functions::right_arrow(&mut editor);
	}

	// Highlight up
	for _i in 0..(2419 - 49) {
		key_functions::highlight_selection::highlight_up(&mut editor);
	}
	key_functions::highlight_selection::highlight_end(&mut editor);
	// Highlight left 10 characters
	for _i in 0..10 {
		key_functions::highlight_selection::highlight_left(&mut editor);
	}

	// Delete the highlighted selection
	editor.delete_selection();

	// Create a vector of all the lines in the first three blocks
	let content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	// What the text should look like
	let expected_text = DELETED_BLOCKS_RESULT;

	assert_eq!(actual_text, expected_text);
}
