/*
====================================
			Blocks Tests
====================================
*/

use super::*;

// Test that initializing a Blocks struct correctly loads in the first block
#[test]
fn blocks_create_test() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	// Create a string from the content of the first block
	let content: Vec<String> = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	// The text that gets loaded in
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first block of this file.
	Remove the last line of the first block by taking a slice up
	to (5053 - 62) (There are 62 newline characters). */
	let expected_text = String::from(&FIRST_BLOCK_GENOME[..(5053 - 62)]);

	// Check that these blocks are the same
	assert_eq!(actual_text, expected_text);
}

// Test the push_tail function to add a new block to the Blocks
#[test]
fn push_tail_test() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the editor
	let _ = editor.init_editor((0, 0), 50, 50);
	// Clone the blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	// Insert a block into the new blocks
	let _ = blocks.push_tail(&mut editor);
	// Set the blocks to the new copy
	editor.blocks = Some(blocks);

	// Create a vector of all the lines in the first two blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	content.extend(
		editor.blocks.as_ref().unwrap().blocks_list[1]
			.content
			.clone(),
	);
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the second block by taking a slice up
	to (5096 - 63) (There are 63 newline characters). */
	let expected_text = String::from(FIRST_BLOCK_GENOME) + &SECOND_BLOCK_GENOME[..(5096 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test the push_head function to add a new block at the beginning of the Blocks struct
#[test]
fn push_head_test() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Create a new Blocks struct starting at the second block of the file
	let blocks = Blocks::new(&mut editor, 1).unwrap();
	editor.blocks = Some(blocks);
	// Create a copy of the Blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	// Insert a new block at the front of the Blocks
	let _ = blocks.push_head(&mut editor);
	// Set this copy as the new editor Blocks
	editor.blocks = Some(blocks);

	// Create a vector of all the lines in the first two blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	content.extend(
		editor.blocks.as_ref().unwrap().blocks_list[1]
			.content
			.clone(),
	);
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the second block by taking a slice up
	to (5096 - 63) (There are 63 newline characters). */
	let expected_text = String::from(FIRST_BLOCK_GENOME) + &SECOND_BLOCK_GENOME[..(5096 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test creating a block using a small file
#[test]
fn small_file_block_test() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the small file
	let mut editor = EditorSpace::new(String::from(SMALL_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	// Create a string from the content of the single block
	let content: Vec<String> = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	let mut actual_text = String::new();
	actual_text.extend(content);

	// The expected contents of the small block
	let expected_text = String::from(
		"#include<stdio.h> 🥹🇺🇸🇳🇴\
        \
        void test_func() {\
        \tprintf(\"Testing the save feature\\n\");\
        }\
        \
        int main() {\
        \tprintf(\"I've almost done it!\\n\");\
        \ttest_func();\
        \
        \treturn 0;\
        }\
        ",
	);

	// Check that the expected equals the actual
	assert_eq!(actual_text, expected_text);
}

// Test that pressing down arrow past the end of the current block loads a new tail block
#[test]
fn down_arrow_block_load() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);

	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	/* Down arrow into the next block (current block is 63 lines long).
	This should cause a second block to be loaded into the Blocks struct. */
	for _i in 0..70 {
		down_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first two blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	content.extend(
		editor.blocks.as_ref().unwrap().blocks_list[1]
			.content
			.clone(),
	);
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the second block by taking a slice up
	to (5096 - 63) (There are 63 newline characters). */
	let expected_text = String::from(FIRST_BLOCK_GENOME) + &SECOND_BLOCK_GENOME[..(5096 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that pressing the up arrow before the beginning of the head block will load a new head
#[test]
fn up_arrow_block_load() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);
	// Create a new Blocks struct starting at the second block of the file
	let blocks = Blocks::new(&mut editor, 1).unwrap();
	editor.blocks = Some(blocks);

	/* Up Arrow into the previous block.
	This should load a new head block. */
	for _i in 0..5 {
		up_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first two blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	content.extend(
		editor.blocks.as_ref().unwrap().blocks_list[1]
			.content
			.clone(),
	);
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the second block by taking a slice up
	to (5096 - 63) (There are 63 newline characters). */
	let expected_text = String::from(FIRST_BLOCK_GENOME) + &SECOND_BLOCK_GENOME[..(5096 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that multiple blocks can be loaded in succession from the down arrow
#[test]
fn repeated_load_down() {
	// Create a default config
	let config = Config::default();
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);

	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	/* Down arrow through multiple blocks. */
	for _i in 0..140 {
		down_arrow(&mut editor);
	}

	/* This should be the first two blocks of this file.
	Remove the last line of the third block by taking a slice up
	to (5106 - 63) (There are 63 newline characters). */
	let expected_text =
		String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME + &THIRD_GENOME_BLOCK[..(5106 - 63)];

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}
	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that the length of Blocks struct is correct
#[test]
fn block_length() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	// The calculated length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();
	// What the length should be (for the first block of GENOME_FILE)
	let expected_length = 63;
	// Check that actual = expected
	assert_eq!(actual_length, expected_length);

	// Push a block to the tail
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	let _ = blocks.push_tail(&mut editor);
	editor.blocks = Some(blocks);

	// The calculated length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();
	// What the length should be (for the first two blocks of GENOME_FILE)
	let expected_length = 127;
	// Check that actual = expected
	assert_eq!(actual_length, expected_length);

	// Push a block to the tail
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	let _ = blocks.push_tail(&mut editor);
	editor.blocks = Some(blocks);

	// The calculated length of the Blocks
	let actual_length = editor.blocks.as_ref().unwrap().len();
	// What the length should be (for the first two blocks of GENOME_FILE)
	let expected_length = 191;
	// Check that actual = expected
	assert_eq!(actual_length, expected_length);
}

// Test pop_head via the down arrow key
#[test]
fn pop_head_down_arrow() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);

	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	/* Down arrow through multiple blocks. */
	for _i in 0..210 {
		down_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}

	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the fourth block by taking a slice up
	to (5106 - 63) (There are 63 newline characters).
	Additionally, concatenate the last line of the first block
	with the second block in order to fix the first line of the
	second block. */
	let expected_text = String::from(&FIRST_BLOCK_GENOME[(5054 - 63)..])
		+ SECOND_BLOCK_GENOME
		+ THIRD_GENOME_BLOCK
		+ &FOURTH_BLOCK_GENOME[..(5101 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test pop_tail via the up arrow key
#[test]
fn pop_tail_up_arrow() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);

	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);
	// Create a new Blocks struct starting at the fourth block of the file
	let blocks = Blocks::new(&mut editor, 3).unwrap();
	editor.blocks = Some(blocks);

	/* Up arrow through multiple blocks. */
	for _i in 0..150 {
		up_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}

	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the third block by taking a slice up
	to (5106 - 63) (There are 63 newline characters). */
	let expected_text =
		String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME + &THIRD_GENOME_BLOCK[..(5106 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test unloading and loading blocks while moving up and down in the file
#[test]
fn unload_blocks_up_and_down() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);

	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	/* Down arrow through multiple blocks.
	This should unload the first block. */
	for _i in 0..210 {
		down_arrow(&mut editor);
	}
	/* Up arrow back to first block.
	This should reload the first block and
	unload the fourth block. */
	for _i in 0..210 {
		up_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}

	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the third block by taking a slice up
	to (5106 - 63) (There are 63 newline characters). */
	let expected_text =
		String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME + &THIRD_GENOME_BLOCK[..(5106 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);

	/* Down arrow through multiple blocks.
	This should unload the first block. */
	for _i in 0..210 {
		down_arrow(&mut editor);
	}

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}

	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	/* This should be the first two blocks of this file.
	Remove the last line of the fourth block by taking a slice up
	to (5106 - 63) (There are 63 newline characters).
	Additionally, concatenate the last line of the first block
	with the second block in order to fix the first line of the
	second block. */
	let expected_text = String::from(&FIRST_BLOCK_GENOME[(5054 - 63)..])
		+ SECOND_BLOCK_GENOME
		+ THIRD_GENOME_BLOCK
		+ &FOURTH_BLOCK_GENOME[..(5101 - 63)];

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test deleting a large number of lines
#[test]
fn delete_lines_test() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	for _i in 0..2398 {
		editor.blocks.as_mut().unwrap().delete_line(0);
	}

	// Create a vector of all the lines in the first three blocks
	let mut content = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	for i in 1..3 {
		content.extend(
			editor.blocks.as_ref().unwrap().blocks_list[i]
				.content
				.clone(),
		);
	}

	// Convert this vector of lines to a string
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter());

	assert_eq!(actual_text, DELETE_LINES_TEST_RESULT);
}

#[test]
fn get_location_test() {
	// Create a default config
	let config = Config::default();
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE), config);
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

	for _i in 0..640 {
		key_functions::down_arrow(&mut editor);
	}
	// Should return block 2, line 1
	let loc = editor.blocks.as_ref().unwrap().get_location(640).unwrap();

	// Should be equal
	assert_eq!(loc, (2, 1));

	// Delete a line
	editor.blocks.as_mut().unwrap().delete_line(630);

	// Should return block 2, line 2
	let loc = editor.blocks.as_ref().unwrap().get_location(640).unwrap();

	assert_eq!(loc, (2, 2));
}
