use crate::editor::EditorSpace;
use key_functions::{down_arrow, up_arrow};
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};

use super::*;

/*
====================================
            Blocks Tests
====================================
*/

// Test that initializing a Blocks struct correctly loads in the first block
#[test]
fn blocks_create_test() {
	// Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 500, 500);

	// Create a string from the content of the first block
	let content: Vec<String> = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
    // The text that gets loaded in
	let mut actual_text = String::new();
	actual_text.par_extend(content.into_par_iter().map(|line| line));

    // This should be the first block of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME);

	// Check that these blocks are the same
	assert_eq!(actual_text, expected_text);
    assert_eq!(editor.blocks.as_ref().unwrap().blocks_list[0].ends_with_newline, false);
}

// Test the insert_tail function to add a new block to the Blocks
#[test]
fn insert_tail_test() {
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
	// Initialize the editor
	let _ = editor.init_editor((0, 0), 500, 500);
	// Clone the blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	// Insert a block into the new blocks
	let _ = blocks.insert_tail(&mut editor);
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
	actual_text.par_extend(content.into_par_iter().map(|line| line));

    // This should be the first two blocks of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME;

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test the insert_head function to add a new block at the beginning of the Blocks struct
#[test]
fn insert_head_test() {
	// Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
	// Create a new Blocks struct starting at the second block of the file
	let blocks = Blocks::new(&mut editor, 1).unwrap();
	editor.blocks = Some(blocks);
	// Create a copy of the Blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	// Insert a new block at the front of the Blocks
	let _ = blocks.insert_head(&mut editor);
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
	actual_text.par_extend(content.into_par_iter().map(|line| line));

    // This should be the first two blocks of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME;

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test creating a block using a small file
#[test]
fn small_file_block_test() {
	// Create an editor over the small file
	let mut editor = EditorSpace::new(String::from(SMALL_FILE));
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 500, 500);

	// Create a string from the content of the single block
	let content: Vec<String> = editor.blocks.as_ref().unwrap().blocks_list[0]
		.content
		.clone();
	let mut actual_text = String::new();
	actual_text.extend(content.into_iter().map(|line| line));

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
    // Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
    // Create a default config
    let config = Config::default();
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

    /* Down arrow into the next block (current block is 63 lines long).
    This should cause a second block to be loaded into the Blocks struct. */
    for _i in 0..70 {
        down_arrow(&mut editor, &config);
    }

    // This should be the first two blocks of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME;

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
	actual_text.par_extend(content.into_par_iter().map(|line| line));

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that pressing the up arrow before the beginning of the head block will load a new head
#[test]
fn up_arrow_block_load() {
    // Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
    // Create a config
    let config = Config::default();
    // Initialize the block (among other things)
    let _ = editor.init_editor((0, 0), 50, 50);
	// Create a new Blocks struct starting at the second block of the file
	let blocks = Blocks::new(&mut editor, 1).unwrap();
	editor.blocks = Some(blocks);
    editor.scroll_offset = 63;

    /* Up Arrow into the previous block.
    This should load a new head block. */
    for _i in 0..5 {
        up_arrow(&mut editor, &config);
    }

    // This should be the first two blocks of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME;

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
	actual_text.par_extend(content.into_par_iter().map(|line| line));

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that multiple blocks can be loaded in succession from the down arrow
#[test]
fn repeated_load_down() {
    // Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
    // Create a default config
    let config = Config::default();
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

    /* Down arrow through multiple blocks. */
    for _i in 0..140 {
        down_arrow(&mut editor, &config);
    }

    // This should be the first two blocks of this file
	let expected_text = String::from(FIRST_BLOCK_GENOME) + SECOND_BLOCK_GENOME + THIRD_GENOME_BLOCK;

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
	actual_text.par_extend(content.into_par_iter().map(|line| line));

	// Compare the actual string to the expected
	assert_eq!(actual_text, expected_text);
}

// Test that the ends_with_newline_flag is being set properly
#[test]
fn newline_test() {
    // Editor that will load in one block from the genome file
	let mut editor = EditorSpace::new(String::from(NEWLINE_FILE));
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 500, 500);

    // Check the newline flag
    let block = editor.blocks.as_ref().unwrap().blocks_list[0].clone();
    assert!(block.ends_with_newline);
}

// Test that the length of Blocks struct is correct
#[test]
fn block_length() {
    // Editor that will load in one block from the `GRCh38_50_rna` file
	let mut editor = EditorSpace::new(String::from(GENOME_FILE));
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 500, 500);

    // The calculated length of the Blocks
    let actual_length = editor.blocks.as_ref().unwrap().len();
    // What the length should be (for the first block of GENOME_FILE)
    let expected_length = 63;
    // Check that actual = expected
    assert_eq!(actual_length, expected_length);

    // Push a block to the tail
    let mut blocks = editor.blocks.as_ref().unwrap().clone();
    let _ = blocks.insert_tail(&mut editor);
    editor.blocks = Some(blocks);

    // The calculated length of the Blocks
    let actual_length = editor.blocks.as_ref().unwrap().len();
    // What the length should be (for the first two blocks of GENOME_FILE)
    let expected_length = 127;
    // Check that actual = expected
    assert_eq!(actual_length, expected_length);

    // Push a block to the tail
    let mut blocks = editor.blocks.as_ref().unwrap().clone();
    let _ = blocks.insert_tail(&mut editor);
    editor.blocks = Some(blocks);

    // The calculated length of the Blocks
    let actual_length = editor.blocks.as_ref().unwrap().len();
    // What the length should be (for the first two blocks of GENOME_FILE)
    let expected_length = 191;
    // Check that actual = expected
    assert_eq!(actual_length, expected_length);
}

#[test]
fn idk() {
    // Create an editor over the genome file
	let mut editor = EditorSpace::new(String::from(PRACTICAL_FILE));
	// Initialize the block (among other things)
	let _ = editor.init_editor((0, 0), 50, 50);

    // Create a vector of all the lines in the first block
	let content = editor.blocks.as_ref().unwrap().blocks_list[0]
        .content
        .clone();

    // Convert this vector of lines to a string
    let mut actual_text = String::new();
    actual_text.par_extend(content.into_par_iter().map(|line| line));
    // What the block should be
    let expected_text = PRACTICAL_FIRST_BLOCK;
    // Test that they are the same
    assert_eq!(actual_text, expected_text);

    // Insert a new block
    let mut blocks = editor.blocks.as_ref().unwrap().clone();
    let _ = blocks.insert_tail(&mut editor);
    editor.blocks = Some(blocks);

    // Create a vector of all the lines in the first two block
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
    actual_text.par_extend(content.into_par_iter().map(|line| line));
    // What the blocks should be
    let expected_text = String::from(PRACTICAL_FIRST_BLOCK) + PRACTICAL_SECOND_BLOCK;
    // Test that they are the same
    assert_eq!(actual_text, expected_text);
}


/*
========================================
            Blocks Constants
========================================
*/

// Small file with very little text
const SMALL_FILE: &str = "../editor/test_files/small_text.txt";

// Large file of part of the human genome
const GENOME_FILE: &str = "../editor/test_files/GRCh38_50_rna.fna";

// File where the first 5KiB block ends in a newline
const NEWLINE_FILE: &str = "../editor/test_files/ends_with_newline.txt";

// An actual source code file
const PRACTICAL_FILE: &str = "../editor/src/lib.rs";

// The first block of the genome file
const FIRST_BLOCK_GENOME: &str =
	">NM_000014.6 Homo sapiens alpha-2-macroglobulin (A2M), transcript variant 1, mRNA\
    GGGACCAGATGGATTGTAGGGAGTAGGGTACAATACAGTCTGTTCTCCTCCAGCTCCTTCTTTCTGCAACATGGGGAAGA\
    ACAAACTCCTTCATCCAAGTCTGGTTCTTCTCCTCTTGGTCCTCCTGCCCACAGACGCCTCAGTCTCTGGAAAACCGCAG\
    TATATGGTTCTGGTCCCCTCCCTGCTCCACACTGAGACCACTGAGAAGGGCTGTGTCCTTCTGAGCTACCTGAATGAGAC\
    AGTGACTGTAAGTGCTTCCTTGGAGTCTGTCAGGGGAAACAGGAGCCTCTTCACTGACCTGGAGGCGGAGAATGACGTAC\
    TCCACTGTGTCGCCTTCGCTGTCCCAAAGTCTTCATCCAATGAGGAGGTAATGTTCCTCACTGTCCAAGTGAAAGGACCA\
    ACCCAAGAATTTAAGAAGCGGACCACAGTGATGGTTAAGAACGAGGACAGTCTGGTCTTTGTCCAGACAGACAAATCAAT\
    CTACAAACCAGGGCAGACAGTGAAATTTCGTGTTGTCTCCATGGATGAAAACTTTCACCCCCTGAATGAGTTGATTCCAC\
    TAGTATACATTCAGGATCCCAAAGGAAATCGCATCGCACAATGGCAGAGTTTCCAGTTAGAGGGTGGCCTCAAGCAATTT\
    TCTTTTCCCCTCTCATCAGAGCCCTTCCAGGGCTCCTACAAGGTGGTGGTACAGAAGAAATCAGGTGGAAGGACAGAGCA\
    CCCTTTCACCGTGGAGGAATTTGTTCTTCCCAAGTTTGAAGTACAAGTAACAGTGCCAAAGATAATCACCATCTTGGAAG\
    AAGAGATGAATGTATCAGTGTGTGGCCTATACACATATGGGAAGCCTGTCCCTGGACATGTGACTGTGAGCATTTGCAGA\
    AAGTATAGTGACGCTTCCGACTGCCACGGTGAAGATTCACAGGCTTTCTGTGAGAAATTCAGTGGACAGCTAAACAGCCA\
    TGGCTGCTTCTATCAGCAAGTAAAAACCAAGGTCTTCCAGCTGAAGAGGAAGGAGTATGAAATGAAACTTCACACTGAGG\
    CCCAGATCCAAGAAGAAGGAACAGTGGTGGAATTGACTGGAAGGCAGTCCAGTGAAATCACAAGAACCATAACCAAACTC\
    TCATTTGTGAAAGTGGACTCACACTTTCGACAGGGAATTCCCTTCTTTGGGCAGGTGCGCCTAGTAGATGGGAAAGGCGT\
    CCCTATACCAAATAAAGTCATATTCATCAGAGGAAATGAAGCAAACTATTACTCCAATGCTACCACGGATGAGCATGGCC\
    TTGTACAGTTCTCTATCAACACCACCAATGTTATGGGTACCTCTCTTACTGTTAGGGTCAATTACAAGGATCGTAGTCCC\
    TGTTACGGCTACCAGTGGGTGTCAGAAGAACACGAAGAGGCACATCACACTGCTTATCTTGTGTTCTCCCCAAGCAAGAG\
    CTTTGTCCACCTTGAGCCCATGTCTCATGAACTACCCTGTGGCCATACTCAGACAGTCCAGGCACATTATATTCTGAATG\
    GAGGCACCCTGCTGGGGCTGAAGAAGCTCTCCTTCTATTATCTGATAATGGCAAAGGGAGGCATTGTCCGAACTGGGACT\
    CATGGACTGCTTGTGAAGCAGGAAGACATGAAGGGCCATTTTTCCATCTCAATCCCTGTGAAGTCAGACATTGCTCCTGT\
    CGCTCGGTTGCTCATCTATGCTGTTTTACCTACCGGGGACGTGATTGGGGATTCTGCAAAATATGATGTTGAAAATTGTC\
    TGGCCAACAAGGTGGATTTGAGCTTCAGCCCATCACAAAGTCTCCCAGCCTCACACGCCCACCTGCGAGTCACAGCGGCT\
    CCTCAGTCCGTCTGCGCCCTCCGTGCTGTGGACCAAAGCGTGCTGCTCATGAAGCCTGATGCTGAGCTCTCGGCGTCCTC\
    GGTTTACAACCTGCTACCAGAAAAGGACCTCACTGGCTTCCCTGGGCCTTTGAATGACCAGGACAATGAAGACTGCATCA\
    ATCGTCATAATGTCTATATTAATGGAATCACATATACTCCAGTATCAAGTACAAATGAAAAGGATATGTACAGCTTCCTA\
    GAGGACATGGGCTTAAAGGCATTCACCAACTCAAAGATTCGTAAACCCAAAATGTGTCCACAGCTTCAACAGTATGAAAT\
    GCATGGACCTGAAGGTCTACGTGTAGGTTTTTATGAGTCAGATGTAATGGGAAGAGGCCATGCACGCCTGGTGCATGTTG\
    AAGAGCCTCACACGGAGACCGTACGAAAGTACTTCCCTGAGACATGGATCTGGGATTTGGTGGTGGTAAACTCAGCAGGT\
    GTGGCTGAGGTAGGAGTAACAGTCCCTGACACCATCACCGAGTGGAAGGCAGGGGCCTTCTGCCTGTCTGAAGATGCTGG\
    ACTTGGTATCTCTTCCACTGCCTCTCTCCGAGCCTTCCAGCCCTTCTTTGTGGAGCTCACAATGCCTTACTCTGTGATTC\
    GTGGAGAGGCCTTCACACTCAAGGCCACGGTCCTAAACTACCTTCCCAAATGCATCCGGGTCAGTGTGCAGCTGGAAGCC\
    TCTCCCGCCTTCCTAGCTGTCCCAGTGGAGAAGGAACAAGCGCCTCACTGCATCTGTGCAAACGGGCGGCAAACTGTGTC\
    CTGGGCAGTAACCCCAAAGTCATTAGGAAATGTGAATTTCACTGTGAGCGCAGAGGCACTAGAGTCTCAAGAGCTGTGTG\
    GGACTGAGGTGCCTTCAGTTCCTGAACACGGAAGGAAAGACACAGTCATCAAGCCTCTGTTGGTTGAACCTGAAGGACTA\
    GAGAAGGAAACAACATTCAACTCCCTACTTTGTCCATCAGGTGGTGAGGTTTCTGAAGAATTATCCCTGAAACTGCCACC\
    AAATGTGGTAGAAGAATCTGCCCGAGCTTCTGTCTCAGTTTTGGGAGACATATTAGGCTCTGCCATGCAAAACACACAAA\
    ATCTTCTCCAGATGCCCTATGGCTGTGGAGAGCAGAATATGGTCCTCTTTGCTCCTAACATCTATGTACTGGATTATCTA\
    AATGAAACACAGCAGCTTACTCCAGAGATCAAGTCCAAGGCCATTGGCTATCTCAACACTGGTTACCAGAGACAGTTGAA\
    CTACAAACACTATGATGGCTCCTACAGCACCTTTGGGGAGCGATATGGCAGGAACCAGGGCAACACCTGGCTCACAGCCT\
    TTGTTCTGAAGACTTTTGCCCAAGCTCGAGCCTACATCTTCATCGATGAAGCACACATTACCCAAGCCCTCATATGGCTC\
    TCCCAGAGGCAGAAGGACAATGGCTGTTTCAGGAGCTCTGGGTCACTGCTCAACAATGCCATAAAGGGAGGAGTAGAAGA\
    TGAAGTGACCCTCTCCGCCTATATCACCATCGCCCTTCTGGAGATTCCTCTCACAGTCACTCACCCTGTTGTCCGCAATG\
    CCCTGTTTTGCCTGGAGTCAGCCTGGAAGACAGCACAAGAAGGGGACCATGGCAGCCATGTATATACCAAAGCACTGCTG\
    GCCTATGCTTTTGCCCTGGCAGGTAACCAGGACAAGAGGAAGGAAGTACTCAAGTCACTTAATGAGGAAGCTGTGAAGAA\
    AGACAACTCTGTCCATTGGGAGCGCCCTCAGAAACCCAAGGCACCAGTGGGGCATTTTTACGAACCCCAGGCTCCCTCTG\
    CTGAGGTGGAGATGACATCCTATGTGCTCCTCGCTTATCTCACGGCCCAGCCAGCCCCAACCTCGGAGGACCTGACCTCT\
    GCAACCAACATCGTGAAGTGGATCACGAAGCAGCAGAATGCCCAGGGCGGTTTCTCCTCCACCCAGGACACAGTGGTGGC\
    TCTCCATGCTCTGTCCAAATATGGAGCAGCCACATTTACCAGGACTGGGAAGGCTGCACAGGTGACTATCCAGTCTTCAG\
    GGACATTTTCCAGCAAATTCCAAGTGGACAACAACAACCGCCTGTTACTGCAGCAGGTCTCATTGCCAGAGCTGCCTGGG\
    GAATACAGCATGAAAGTGACAGGAGAAGGATGTGTCTACCTCCAGACATCCTTGAAATACAATATTCTCCCAGAAAAGGA\
    AGAGTTCCCCTTTGCTTTAGGAGTGCAGACTCTGCCTCAAACTTGTGATGAACCCAAAGCCCACACCAGCTTCCAAATCT\
    CCCTAAGTGTCAGTTACACAGGGAGCCGCTCTGCCTCCAACATGGCGATCGTTGATGTGAAGATGGTCTCTGGCTTCATT\
    CCCCTGAAGCCAACAGTGAAAATGCTTGAAAGATCTAACCATGTGAGCCGGACAGAAGTCAGCAGCAACCATGTCTTGAT\
    TTACCTTGATAAGGTGTCAAATCAGACACTGAGCTTGTTCTTCACGGTTCTGCAAGATGTCCCAGTAAGAGATCTGAAAC\
    CAGCCATAGTGAAAGTCTATGATTACTACGAGACGGATGAGTTTGCAATTGCTGAGTACAATGCTCCTTGCAGCAAAGAT\
    CTTGGAAATGCTTGAAGACCACAAGGCTGAAAAGTGCTTTGCTGGAGTCCTGTTCTCAGAGCTCCACAGAAGACACGTGT\
    TTTTGTATCTTTAAAGACTTGATGAATAAACACTTTTTCTGGTCAATGTC\
    >NM_000015.3 Homo sapiens N-acetyltransferase 2 (NAT2), mRNA\
    ACTTTATTACAGACCTTGGAAGCAAGAGGATTGCATTCAGCCTAGTTCCTGGTTGCTGGCCAAAGGGATCATGGACATTG\
    AAGCATATTTTGAAAGAATTGGCTATAAGAACTCTAGGAACAAATTGGACTTGGAAACATTAACTGACATTCTTGAGCAC\
    CAGATCCGGGCTGTTCCCTTTGAGAACCTTAACATGCATTGTGGGCAAGCCATGGAGTTGGGCTTAGAGGCTATTTTTGA\
    TCACATTGTAAGAAGAAACCGGGGTGGGTGGTGTCTCCAGGTCAATCAACTTCTGTACTGGGCTCT"; // 64 lines (last line incomplete = 63)

// Second block of the genome file
const SECOND_BLOCK_GENOME: &str = 
    "GACCACAATCGGTT\
    TTCAGACCACAATGTTAGGAGGGTATTTTTACATCCCTCCAGTTAACAAATACAGCACTGGCATGGTTCACCTTCTCCTG\
    CAGGTGACCATTGACGGCAGGAATTACATTGTCGATGCTGGGTCTGGAAGCTCCTCCCAGATGTGGCAGCCTCTAGAATT\
    AATTTCTGGGAAGGATCAGCCTCAGGTGCCTTGCATTTTCTGCTTGACAGAAGAGAGAGGAATCTGGTACCTGGACCAAA\
    TCAGGAGAGAGCAGTATATTACAAACAAAGAATTTCTTAATTCTCATCTCCTGCCAAAGAAGAAACACCAAAAAATATAC\
    TTATTTACGCTTGAACCTCGAACAATTGAAGATTTTGAGTCTATGAATACATACCTGCAGACGTCTCCAACATCTTCATT\
    TATAACCACATCATTTTGTTCCTTGCAGACCCCAGAAGGGGTTTACTGTTTGGTGGGCTTCATCCTCACCTATAGAAAAT\
    TCAATTATAAAGACAATACAGATCTGGTCGAGTTTAAAACTCTCACTGAGGAAGAGGTTGAAGAAGTGCTGAGAAATATA\
    TTTAAGATTTCCTTGGGGAGAAATCTCGTGCCCAAACCTGGTGATGGATCCCTTACTATTTAGAATAAGGAACAAAATAA\
    ACCCTTGTGTATGTATCACCCAACTCACTAATTATCAACTTATGTGCTATCAGATATCCTCTCTACCCTCACGTTATTTT\
    GAAGAAAATCCTAAACATCAAATACTTTCATCCATAAAAATGTCAGCATTTATTAAAAAACAATAACTTTTTAAAGAAAC\
    ATAAGGACACATTTTCAAATTAATAAAAATAAAGGCATTTTAAGGATGGCCTGTGATTATCTTGGGAAGCAGAGTGATTC\
    ATGCTAGAAAACATTTAATATTGATTTATTGTTGAATTCATAGTAAATTTTTACTGGTAAATGAATAAAGAATATTGTGG\
    AAAAA\
    >NM_000016.6 Homo sapiens acyl-CoA dehydrogenase medium chain (ACADM), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    AGAGGAGTCCCGCGTTCGGGGAGTATGTCAAGGCCGTGACCCGTGTATTATTGTCCGAGTGGCCGGAACGGGAGCCAACA\
    TGGCAGCGGGGTTCGGGCGATGCTGCAGGGTCCTGAGAAGTATTTCTCGTTTTCATTGGAGATCACAGCATACAAAAGCC\
    AATCGACAACGTGAACCAGGATTAGGATTTAGTTTTGAGTTCACCGAACAGCAGAAAGAATTTCAAGCTACTGCTCGTAA\
    ATTTGCCAGAGAGGAAATCATCCCAGTGGCTGCAGAATATGATAAAACTGGTGAATATCCAGTCCCCCTAATTAGAAGAG\
    CCTGGGAACTTGGTTTAATGAACACACACATTCCAGAGAACTGTGGAGGTCTTGGACTTGGAACTTTTGATGCTTGTTTA\
    ATTAGTGAAGAATTGGCTTATGGATGTACAGGGGTTCAGACTGCTATTGAAGGAAATTCTTTGGGGCAAATGCCTATTAT\
    TATTGCTGGAAATGATCAACAAAAGAAGAAGTATTTGGGGAGAATGACTGAGGAGCCATTGATGTGTGCTTATTGTGTAA\
    CAGAACCTGGAGCAGGCTCTGATGTAGCTGGTATAAAGACCAAAGCAGAAAAGAAAGGAGATGAGTATATTATTAATGGT\
    CAGAAGATGTGGATAACCAACGGAGGAAAAGCTAATTGGTATTTTTTATTGGCACGTTCTGATCCAGATCCTAAAGCTCC\
    TGCTAATAAAGCCTTTACTGGATTCATTGTGGAAGCAGATACCCCAGGAATTCAGATTGGGAGAAAGGAATTAAACATGG\
    GCCAGCGATGTTCAGATACTAGAGGAATTGTCTTCGAAGATGTGAAAGTGCCTAAAGAAAATGTTTTAATTGGTGACGGA\
    GCTGGTTTCAAAGTTGCAATGGGAGCTTTTGATAAAACCAGACCTGTAGTAGCTGCTGGTGCTGTTGGATTAGCACAAAG\
    AGCTTTGGATGAAGCTACCAAGTATGCCCTGGAAAGGAAAACTTTCGGAAAGCTACTTGTAGAGCACCAAGCAATATCAT\
    TTATGCTGGCTGAAATGGCAATGAAAGTTGAACTAGCTAGAATGAGTTACCAGAGAGCAGCTTGGGAGGTTGATTCTGGT\
    CGTCGAAATACCTATTATGCTTCTATTGCAAAGGCATTTGCTGGAGATATTGCAAATCAGTTAGCTACTGATGCTGTGCA\
    GATACTTGGAGGCAATGGATTTAATACAGAATATCCTGTAGAAAAACTAATGAGGGATGCCAAAATCTATCAGATTTATG\
    AAGGTACTTCACAAATTCAAAGACTTATTGTAGCCCGTGAACACATTGACAAGTACAAAAATTAAAAAAATTACTGTAGA\
    AATATTGAATAACTAGAACACAAGCCACTGTTTCAGCTCCAGAAAAAAGAAAGGGCTTTAACGTTTTTTCCAGTGAAAAC\
    AAATCCTCTTATATTAAATCTAAGCAACTGCTTATTATAGTAGTTTATACTTTTGCTTAACTCTGTTATGTCTCTTAAGC\
    AGGTTTGGTTTTTATTAAAATGATGTGTTTTCTTTAGTACCACTTTACTTGAATTACATTAACCTAGAAAACTACATAGG\
    TTATTTTGATCTCTTAAGATTAATGTAGCAGAAATTTCTTGGAATTTTATTTTTGTAATGACAGAAAAGTGGGCTTAGAA\
    AGTATTCAAGATGTTACAAAATTTACATTTAGAAAATATTGTAGTATTTGAATACTGTCAACTTGACAGTAACTTTGTAG\
    ACTTAATGGTATTATTAAAGTTCTTTTTATTGCAGTTTGGAAAGCATTTGTGAAACTTTCTGTTTGGCACAGAAACAGTC\
    AAAATTTTGACATTCATATTCTCCTATTTTACAGCTACAAGAACTTTCTTGAAAATCTTATTTAATTCTGAGCCCATATT\
    TCACTTACCTTATTTAAAATAAATCAATAAAGCTTGCCTTAAATTATTTTTATATGACTGTTGGTCTCTAGGTAGCCTTT\
    GGTCTATTGTACACAATCTCATTTCATATGTTTGCATTTTGGCAAAGAACTTAATAAAATTGTTCAGTGCTTATTATCAT\
    ATCTTTCTGTATTTTTTCCAGGAAATTTCATTACTTCGTGTAATAGTGTATATTTCTTGTATTTACTATGATGAAAAAAG\
    GTCGTTTTAATTTTGAATTGAATAAAGTTACCTGTTCATTTTTTATTAGATATTTTAAAGACTTCAGAAAATATAAATAT\
    GAAATAATTTAAGAACCCAAA\
    >NM_000017.4 Homo sapiens acyl-CoA dehydrogenase short chain (ACADS), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    ACTCCGGAACAGCGCGCTCGCAGCGGGAGGTCGCGAAGCCTGGGACTGTGTCTGTCGCCCATGGCCGCCGCGCTGCTCGC\
    CCGGGCCTCGGGCCCTGCCCGCAGAGCTCTCTGTCCTAGGGCCTGGCGGCAGTTACACACCATCTACCAGTCTGTGGAAC\
    TGCCCGAGACACACCAGATGTTGCTCCAGACATGCCGGGACTTTGCCGAGAAGGAGTTGTTTCCCATTGCAGCCCAGGTG\
    GATAAGGAACATCTCTTCCCAGCGGCTCAGGTGAAGAAGATGGGCGGGCTTGGGCTTCTGGCCATGGACGTGCCCGAGGA\
    GCTTGGCGGTGCTGGCCTCGATTACCTGGCCTACGCCATCGCCATGGAGGAGATCAGCCGTGGCTGCGCCTCCACCGGAG\
    TCATCATGAGTGTCAACAACTCTCTCTACCTGGGGCCCATCTTGAAGTTTGGCTCCAAGGAGCAGAAGCAGGCGTGGGTC\
    ACGCCTTTCACCAGTGGTGACAAAATTGGCTGCTTTGCCCTCAGCGAACCAGGGAACGGCAGTGATGCAGGAGCTGCGTC\
    CACCACCGCCCGGGCCGAGGGCGACTCATGGGTTCTGAATGGAACCAAAGCCTGGATCACCAATGCCTGGGAGGCTTCGG\
    CTGCCGTGGTCTTTGCCAGCACGGACAGAGCCCTGCAAAACAAGGGCATCAGTGCCTTCCTGGTCCCCATGCCAACGCCT\
    GGGCTCACGTTGGGGAAGAAAGAAGACAAGCTGGGCATCCGGGGCTCATCCACGGCCAACCTCATCTTTGAGGACTGTCG\
    CATCCCCAAGGACAGCATCCTGGGGGAGCCAGGGATGGGCTTCAAGATAGCCATGCAAACCCTGGACATGGGCCGCATCG\
    GCATCGCCTCCCAGGCCCTGGGCATTGCCCAGACCGCCCTCGATTGTGCTGTGAACTACGCTGAGAATCGCATGGCCTTC\
    GGGGCGCCCCTCACCAAGCTCCAGGTCATCCAGTTCAAGTTGGCAGACATGGCCCTGGCCCTGGAGAGTGCCCGGCTGCT\
    GACCTGGCGCGCTGCCATGCTGAAGGATAACAAGAAGCCTTTCATCAAGGAGGCAGCCATGGCCAAGCTGGCCGCCTCGG\
    AGGCCGCGACCGCCATCAGCCACCAGGCCATCCAGATCCTGGGCGGCATGGGCTACGTGACAGAGATGCCGGCAGAGCGG\
    CACTACCGCGACGCCCGCATCACTGAGATCTACGAGGGCACCAGCGAAATCCAGCGGCTGGTGATCGCCGGGCATCTGCT\
    CAGGAGCTACCGGAGCTGAGCCCGCGGCGGACTGCCCCAGGACTGCGGGAAGGCGCGGGAGCCAGGGGCCTCCACCCCAA\
    CCCCGGCTCAGAGACTGGGCGGCCCGGCGGGGGCTCCCTGGGGACCCCAGATGGGCTCAGTGCTGCCACCCAGATCAGAT\
    CACATGGGAATGAGGCCCTCCGACCATTGGCAGCTCCGCCTCTGGGCCTTTCCGCCTCCTCACCACTGTGCCTCAAGTTC\
    CTCATCTAAGTGGCCCTGGCCTC"; // 65 lines (last line incomplete = 64)

// Third block of the genome file
const THIRD_GENOME_BLOCK: &str = 
    "CTGGGGGCGGGGTTGTGGGGGGGCTGAGCGACACTCAGGGACACCTCAGTTGTCCTC\
    CCGCGGGCCCTGGTGCCCTGGCATGAAGGCCCAGTGCGACAGGCCCTTGGTGGGGTCTGTCTTTTCCTTGAGGTCAGAGG\
    TCAGGAGCAGGGCTGGGGTCAGGATGACGAGGCCTGGGGTCCTGGTGTTGGGCAGGTGGTGGGGCTGGGCCATGGAGCTG\
    GCCCAGAGGCCCCTCAGCCCTTTGTAAAGTCTGATGAAGGCAGGGGTGGTGATTCATGCTGTGTGACTGACTGTGGGTAA\
    TAAACACACCTGTCCCCCA\
    >NM_000018.4 Homo sapiens acyl-CoA dehydrogenase very long chain (ACADVL), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    AGAGCTGGGTCAGAGCTCGAGCCAGCGGCGCCCGGAGAGATTCGGAGATGCAGGCGGCTCGGATGGCCGCGAGCTTGGGG\
    CGGCAGCTGCTGAGGCTCGGGGGCGGAAGCTCGCGGCTCACGGCGCTCCTGGGGCAGCCCCGGCCCGGCCCTGCCCGGCG\
    GCCCTATGCCGGGGGTGCCGCTCAGCTGGCTCTGGACAAGTCAGATTCCCACCCCTCTGACGCTCTGACCAGGAAAAAAC\
    CGGCCAAGGCGGAATCTAAGTCCTTTGCTGTGGGAATGTTCAAAGGCCAGCTCACCACAGATCAGGTGTTCCCATACCCG\
    TCCGTGCTCAACGAAGAGCAGACACAGTTTCTTAAAGAGCTGGTGGAGCCTGTGTCCCGTTTCTTCGAGGAAGTGAACGA\
    TCCCGCCAAGAATGACGCTCTGGAGATGGTGGAGGAGACCACTTGGCAGGGCCTCAAGGAGCTGGGGGCCTTTGGTCTGC\
    AAGTGCCCAGTGAGCTGGGTGGTGTGGGCCTTTGCAACACCCAGTACGCCCGTTTGGTGGAGATCGTGGGCATGCATGAC\
    CTTGGCGTGGGCATTACCCTGGGGGCCCATCAGAGCATCGGTTTCAAAGGCATCCTGCTCTTTGGCACAAAGGCCCAGAA\
    AGAAAAATACCTCCCCAAGCTGGCATCTGGGGAGACTGTGGCCGCTTTCTGTCTAACCGAGCCCTCAAGCGGGTCAGATG\
    CAGCCTCCATCCGAACCTCTGCTGTGCCCAGCCCCTGTGGAAAATACTATACCCTCAATGGAAGCAAGCTTTGGATCAGT\
    AATGGGGGCCTAGCAGACATCTTCACGGTCTTTGCCAAGACACCAGTTACAGATCCAGCCACAGGAGCCGTGAAGGAGAA\
    GATCACAGCTTTTGTGGTGGAGAGGGGCTTCGGGGGCATTACCCATGGGCCCCCTGAGAAGAAGATGGGCATCAAGGCTT\
    CAAACACAGCAGAGGTGTTCTTTGATGGAGTACGGGTGCCATCGGAGAACGTGCTGGGTGAGGTTGGGAGTGGCTTCAAG\
    GTTGCCATGCACATCCTCAACAATGGAAGGTTTGGCATGGCTGCGGCCCTGGCAGGTACCATGAGAGGCATCATTGCTAA\
    GGCGGTAGATCATGCCACTAATCGTACCCAGTTTGGGGAGAAAATTCACAACTTTGGGCTGATCCAGGAGAAGCTGGCAC\
    GGATGGTTATGCTGCAGTATGTAACTGAGTCCATGGCTTACATGGTGAGTGCTAACATGGACCAGGGAGCCACGGACTTC\
    CAGATAGAGGCCGCCATCAGCAAAATCTTTGGCTCGGAGGCAGCCTGGAAGGTGACAGATGAATGCATCCAAATCATGGG\
    GGGTATGGGCTTCATGAAGGAACCTGGAGTAGAGCGTGTGCTCCGAGATCTTCGCATCTTCCGGATCTTTGAGGGGACAA\
    ATGACATTCTTCGGCTGTTTGTGGCTCTGCAGGGCTGTATGGACAAAGGAAAGGAGCTCTCTGGGCTTGGCAGTGCTCTA\
    AAGAATCCCTTTGGGAATGCTGGCCTCCTGCTAGGAGAGGCAGGCAAACAGCTGAGGCGGCGGGCAGGGCTGGGCAGCGG\
    CCTGAGTCTCAGCGGACTTGTCCACCCGGAGTTGAGTCGGAGTGGCGAGCTGGCAGTACGGGCTCTGGAGCAGTTTGCCA\
    CTGTGGTGGAGGCCAAGCTGATAAAACACAAGAAGGGGATTGTCAATGAACAGTTTCTGCTGCAGCGGCTGGCAGACGGG\
    GCCATCGACCTCTATGCCATGGTGGTGGTTCTCTCGAGGGCCTCAAGATCCCTGAGTGAGGGCCACCCCACGGCCCAGCA\
    TGAGAAAATGCTCTGTGACACCTGGTGTATCGAGGCTGCAGCTCGGATCCGAGAGGGCATGGCCGCCCTGCAGTCTGACC\
    CCTGGCAGCAAGAGCTCTACCGCAACTTCAAAAGCATCTCCAAGGCCTTGGTGGAGCGGGGTGGTGTGGTCACCAGCAAC\
    CCACTTGGCTTCTGAATACTCCCGGCCAGGGCCTGTCCCAGTTATGTGCCTTCCCTCAAGCCAAAGCCGAAGCCCCTTTC\
    CTTAAGGCCCTGGTTTGTCCCGAAGGGGCCTAGTGTTCCCAGCACTGTGCCTGCTCTCAAGAGCACTTACTGCCTCGCAA\
    ATAATAAAAATTTCTAGCCAGTCA\
    >NM_000019.4 Homo sapiens acetyl-CoA acetyltransferase 1 (ACAT1), transcript variant 2, mRNA; nuclear gene for mitochondrial product\
    AGTCTACGCCTGTGGAGCCGATACTCAGCCCTCTGCGACCATGGCTGTGCTGGCGGCACTTCTGCGCAGCGGCGCCCGCA\
    GCCGCAGCCCCCTGCTCCGGAGGCTGGTGCAGGAAATAAGATATGTGGAACGGAGTTATGTATCAAAACCCACTTTGAAG\
    GAAGTGGTCATAGTAAGTGCTACAAGAACACCCATTGGATCTTTTTTAGGCAGCCTTTCCTTGCTGCCAGCCACTAAGCT\
    TGGTTCCATTGCAATTCAGGGAGCCATTGAAAAGGCAGGGATTCCAAAAGAAGAAGTGAAAGAAGCATACATGGGTAATG\
    TTCTACAAGGAGGTGAAGGACAAGCTCCTACAAGGCAGGCAGTATTGGGTGCAGGCTTACCTATTTCTACTCCATGTACC\
    ACCATAAACAAAGTTTGTGCTTCAGGAATGAAAGCCATCATGATGGCCTCTCAAAGTCTTATGTGTGGACATCAGGATGT\
    GATGGTGGCAGGTGGGATGGAGAGCATGTCCAATGTTCCATATGTAATGAACAGAGGATCAACACCATATGGTGGGGTAA\
    AGCTTGAAGATTTGATTGTAAAAGACGGGCTAACTGATGTCTACAATAAAATTCATATGGGCAGCTGTGCTGAGAATACA\
    GCAAAGAAGCTGAATATTGCACGAAATGAACAGGACGCTTATGCTATTAATTCTTATACCAGAAGTAAAGCAGCATGGGA\
    AGCTGGGAAATTTGGAAATGAAGTTATTCCTGTCACAGTTACAGTAAAAGGTCAACCAGATGTAGTGGTGAAAGAAGATG\
    AAGAATATAAACGTGTTGATTTTAGCAAAGTTCCAAAGCTGAAGACAGTTTTCCAGAAAGAAAATGGCACAGTAACAGCT\
    GCCAATGCCAGTACACTGAATGATGGAGCAGCTGCTCTGGTTCTCATGACGGCAGATGCAGCGAAGAGGCTCAATGTTAC\
    ACCACTGGCAAGAATAGTAGCATTTGCTGACGCTGCTGTAGAACCTATTGATTTTCCAATTGCTCCTGTATATGCTGCAT\
    CTATGGTTCTTAAAGATGTGGGATTGAAAAAAGAAGATATTGCAATGTGGGAAGTAAATGAAGCCTTTAGTCTGGTTGTA\
    CTAGCAAACATTAAAATGTTGGAGATTGATCCCCAAAAAGTGAATATCAATGGAGGAGCTGTTTCTCTGGGACATCCAAT\
    TGGGATGTCTGGAGCCAGGATTGTTGGTCATTTGACTCATGCCTTGAAGCAAGGAGAATACGGTCTTGCCAGTATTTGCA\
    ATGGAGGAGGAGGTGCTTCTGCCATGCTAATTCAGAAGCTGTAGACAACCTCTGCTATTTAAGGAGACAACCCTATGTGA\
    CCAGAAGGCCTGCTGTAATCAGTGTGACTACTGTGGGTCAGCTTATATTCAGATAAGCTGTTTCATTTTTTATTATTTTC\
    TATGTTAACTTTTAAAAATCAAAATGATGAAATCCCAAAACATTTTGAAATTAAAAATAAATTTCTTCTTCTGCTTTTTT\
    CTTGGTAACCTTGAAAA\
    >NM_000020.3 Homo sapiens activin A receptor like type 1 (ACVRL1), transcript variant 1, mRNA\
    CCCAGTCCCGGGAGGCTGCCGCGCCAGCTGCGCCGAGCGAGCCCCTCCCCGGCTCCAGCCCGGTCCGGGGCCGCGCCCGG\
    ACCCCAGCCCGCCGTCCAGCGCTGGCGGTGCAACTGCGGCCGCGCGGTGGAGGGGAGGTGGCCCCGGTCCGCCGAAGGCT\
    AGCGCCCCGCCACCCGCAGAGCGGGCCCAGAGGGACCATGACCTTGGGCTCCCCCAGGAAAGGCCTTCTGATGCTGCTGA\
    TGGCCTTGGTGACCCAGGGAGACCCTGTGAAGCCGTCTCGGGGCCCGCTGGTGACCTGCACGTGTGAGAGCCCACATTGC\
    AAGGGGCCTACCTGCCGGGGGGCCTGGTGCACAGTAGTGCTGGTGCGGGAGGAGGGGAGGCACCCCCAGGAACATCGGGG\
    CTGCGGGAACTTGCACAGGGAGCTCTGCAGGGGGCGCCCCACCGAGTTCGTCAACCACTACTGCTGCGACAGCCACCTCT\
    GCAACCACAACGTGTCCCTGGTGCTGGAGGCCACCCAACCTCCTTCGGAGCAGCCGGGAACAGATGGCCAGCTGGCCCTG\
    ATCCTGGGCCCCGTGCTGGCCTTGCTGGCCCTGGTGGCCCTGGGTGTCCTGGGCCTGTGGCATGTCCGACGGAGGCAGGA\
    GAAGCAGCGTGGC"; // 65 lines (last line incomplete = 64)

// First block of the practical file
const PRACTICAL_FIRST_BLOCK: &str =
    "pub mod editor {\
\
        \tuse std::{\
            \t\tfs::{File, OpenOptions},\
            \t\tio::{self, BufRead, Error},\
            \t\tpath::Path,\
            \t\ttime::Duration,\
        \t};\
\
        \tuse crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};\
        \tuse ratatui::{\
            \t\tstyle::Style,\
            \t\ttext::{Line, Span, Text},\
            \t\twidgets::Paragraph,\
        \t};\
        \tuse rayon::iter::{\
            \t\tIndexedParallelIterator, IntoParallelIterator, ParallelBridge, ParallelExtend,\
            \t\tParallelIterator,\
        \t};\
\
        \tuse config::config::Config;\
\
        \tmod blocks;\
        \tuse blocks::Blocks;\
\
        \t// Module containing all the functionality of each key. Called in handle_input\
        \tmod key_functions;\
        \tuse key_functions::highlight_selection::Selection;\
        \tuse unicode_segmentation::UnicodeSegmentation;\
\
        \t// Testing module found at crate/src/editor/tests.rs\
        \t#[cfg(test)]\
        \tmod tests;\
\
        \tpub struct EditorSpace {\
            \t\t// Text block of current frame\
            \t\tpub blocks: Option<Blocks>,\
            \t\t// Flag for whether to break rendering loop in main app\
            \t\tpub break_loop: bool,\
            \t\t// Position on the current line of text\
            \t\tpub text_position: usize,\
            \t\t// Position of cursor on the screen (and in the text)\
            \t\tpub cursor_position: [usize; 2],\
            \t\t// Name of file opened in current editor frame\
            \t\tpub filename: String,\
            \t\t// The file that is open\
            \t\tpub file: File,\
            \t\t// The length of the entire file that is being openned\
            \t\tpub file_length: usize,\
            \t\t// Vertical bounds of the editor block\
            \t\tpub height: (usize, usize),\
            \t\t// Horizontal bounds of the editor block\
            \t\tpub width: (usize, usize),\
            \t\t// Sets the amount to scroll the text\
            \t\tscroll_offset: usize,\
            \t\t// Structure keeping track of the highlighted selection of text\
            \t\tselection: Selection,\
            \t\t// Track if the starting cursor position has already been set\
            \t\tpub start_cursor_set: bool,\
        \t}\
\
        \timpl EditorSpace {\
            \t\tpub fn new(filename: String) -> Self {\
                \t\t\t// Check if a file exists, if not create it\
                \t\t\tif !Path::new(&filename).exists() {\
                    \t\t\t\tFile::create(&filename).unwrap();\
                \t\t\t}\
                \t\t\t// Open the file in read-write mode\
                \t\t\tlet file = match OpenOptions::new().read(true).write(true).open(&filename) {\
                    \t\t\t\tOk(file) => file,\
                    \t\t\t\tErr(err) => panic!(\"{}\", err),\
                \t\t\t};\
                \t\t\t// Construct an EditorSpace\
                \t\t\tEditorSpace {\
                    \t\t\t\tblocks: None,\
                    \t\t\t\tbreak_loop: false,\
                    \t\t\t\ttext_position: 0,\
                    \t\t\t\tcursor_position: [0, 0],\
                    \t\t\t\tfilename,\
                    \t\t\t\tfile,\
                    \t\t\t\tfile_length: 0,\
                    \t\t\t\theight: (0, 0),\
                    \t\t\t\twidth: (0, 0),\
                    \t\t\t\tscroll_offset: 0,\
                    \t\t\t\tselection: Selection::new(),\
                    \t\t\t\tstart_cursor_set: false,\
                \t\t\t}\
            \t\t}\
\
            \t\t// Set the starting Position of the editing space cursor\
            \t\tfn init_starting_position(&mut self, start: (usize, usize), width: usize, height: usize) {\
                \t\t\t// Set the bounds of the block\
                \t\t\tself.width = (start.0, start.0 + width);\
                \t\t\tself.height = (start.1, start.1 + height);\
\
                \t\t\t// Set the cursor to the beginning of the block\
                \t\t\tself.cursor_position = [0, 0];\
\
                \t\t\t// Flag that cursor has been initialized\
                \t\t\tself.start_cursor_set = true;\
            \t\t}\
\
            \t\t// Initialize the file length variable\
            \t\tfn init_file_length(&mut self) -> Result<usize, Error> {\
                \t\t\t// Open the file\
                \t\t\tlet file = File::open(&self.filename)?;\
                \t\t\t// Count the lines of the file (in parallel)\
                \t\t\tself.file_length = io::BufReader::new(file).lines().par_bridge().count();\
                \t\t\t// Return the file length\
                \t\t\tOk(self.file_length)\
            \t\t}\
\
            \t\t// Create the first block when the editor is opened\
            \t\tfn init_first_block(&mut self) -> Result<usize, Error> {\
                \t\t\t// Create a block at block number 0\
                \t\t\tlet blocks = Blocks::new(self, 0)?;\
                \t\t\t// Wrap this Blocks in an Option\
                \t\t\tself.blocks = Some(blocks);\
                \t\t\t// Return 0 to indicate success\
                \t\t\tOk(0)\
            \t\t}\
\
            \t\t// Initialize the editor\
            \t\tpub fn init_editor(\
                \t\t\t&mut self,\
                \t\t\tstart: (usize, usize),\
                \t\t\twidth: usize,\
                \t\t\theight: usize,\
            \t\t) -> Result<&str, Error> {\
                \t\t\t// Initialize the starting position of the screen cursor\
                \t\t\tself.init_starting_position(start, width, height);\
                \t\t\t// Initialize the file length\
                \t\t\tself.init_file_length()?;\
                \t\t\t// Create the first block of text in Blocks\
                \t\t\tself.init_first_block()?;\
                \t\t\t// Return the string \"Success\" (arbitrary)\
                \t\t\tOk(\"Success\")\
            \t\t}\
\
            \t\t// Highlight a specific character on the line within the highlighting selection\
            \t\tfn highlight_char(\
                \t\t\t&self,\
                \t\t\tconfig: &Config,\
                \t\t\tidx: usize,\
                \t\t\tloc: usize,\
                \t\t\tcharacter: String,\
            \t\t) -> Span {\
                \t\t\tif !self.selection.is_empty {\
                    \t\t\t\t// If only one line\
                    \t\t\t\tif idx == self.selection.start[1]\
                        \t\t\t\t\t&& self.selection.start[1] == self.selection.end[1]\
                    \t\t\t\t{\
                        \t\t\t\t\t// If within selection, highlight character\
                        \t\t\t\t\tif loc >= self.selection.start[0] && loc < self.selection.end[0] {\
                            \t\t\t\t\t\tSpan::from(character)\
                                \t\t\t\t\t\t\t.style(Style::default().bg(config.theme.selection_highlight))\
                        \t\t\t\t\t} else {\
                            \t\t\t\t\t\tSpan::from(character)\
                        \t\t\t\t\t}\
                    \t\t\t\t// If on first line (and there are multiple lines in selection)\
                    \t\t\t\t} else if idx == self.selection.start[1] {\
                        \t\t\t\t\t// Highlight all characters on the line after the cursor\
                        \t\t\t\t\tif loc >= self.selection.start[0] {\
                            \t\t\t\t\t\tSpan::from(character)\
                                \t\t\t\t\t\t\t.style(Style::default().bg(config.theme.selection_highlight))\
                        \t\t\t\t\t} else {\
                            \t\t\t\t\t\tSpan::from(character)\
                        \t\t\t\t\t}\
                    \t\t\t\t// If on last line (and there are multiple lines in selection)\
                    \t\t\t\t} else if idx == self.selection.end[1] {\
                        \t\t\t\t\t// Highlight all characters on the line before the cursor\
                        \t\t\t\t\tif loc < s";

// Second block of the practical file
const PRACTICAL_SECOND_BLOCK: &str =
    "elf.selection.end[0] {\
                            \t\t\t\t\t\tSpan::from(character)\
                                \t\t\t\t\t\t\t.style(Style::default().bg(config.theme.selection_highlight))\
                        \t\t\t\t\t} else {\
                            \t\t\t\t\t\tSpan::from(character)\
                        \t\t\t\t\t}\
                    \t\t\t\t// If between first and last line in multine selection\
                    \t\t\t\t} else if idx > self.selection.start[1] && idx < self.selection.end[1] {\
                        \t\t\t\t\tSpan::from(character)\
                            \t\t\t\t\t\t.style(Style::default().bg(config.theme.selection_highlight))\
                    \t\t\t\t// If not in selection\
                    \t\t\t\t} else {\
                        \t\t\t\t\tSpan::from(character)\
                    \t\t\t\t}\
                \t\t\t} else {\
                    \t\t\t\tSpan::from(character)\
                \t\t\t}\
            \t\t}\
\
            \t\t// Create a Line struct from the given String line\
            \t\tfn parse_line(&self, config: &Config, idx: usize, line: &str) -> Line {\
                \t\t\t// Split the line into individual words\
                \t\t\tlet characters: Vec<&str> = line.graphemes(true).collect();\
                \t\t\tlet mut spans: Vec<Span> = Vec::new();\
                \t\t\t// Iterate through each character on the line\
                \t\t\tspans.par_extend(\
                    \t\t\t\tcharacters\
                        \t\t\t\t\t.into_par_iter()\
                        \t\t\t\t\t.enumerate()\
                        \t\t\t\t\t.map(|(loc, character)| {\
                            \t\t\t\t\t\tmatch character {\
                                \t\t\t\t\t\t\t\"\\t\" => {\
                                    \t\t\t\t\t\t\t\t// Start tab with a vertical line\
                                    \t\t\t\t\t\t\t\tlet mut tab_char = String::from(\"\\u{023D0}\");\
                                    \t\t\t\t\t\t\t\t// Iterator to create a string of tab_width - 1 number of spaces\
                                    \t\t\t\t\t\t\t\ttab_char.push_str(&\" \".repeat(config.tab_width - 1));\
                                    \t\t\t\t\t\t\t\t// Highlight this spaces representation of a tab\
                                    \t\t\t\t\t\t\t\tself.highlight_char(config, idx, loc, tab_char)\
                                \t\t\t\t\t\t\t}\
                                \t\t\t\t\t\t\t_ => {\
                                    \t\t\t\t\t\t\t\t// Highlight this (non-tab) character\
                                    \t\t\t\t\t\t\t\tself.highlight_char(config, idx, loc, String::from(character))\
                                \t\t\t\t\t\t\t}\
                            \t\t\t\t\t\t}\
                        \t\t\t\t\t}),\
                \t\t\t);\
\
                \t\t\t// Return the line\
                \t\t\tLine::from(spans)\
            \t\t}\
\
            \t\t// Get the current line number\
            \t\tfn get_line_num(&self) -> usize {\
                \t\t\tself.cursor_position[1] + self.scroll_offset\
            \t\t}\
\
            \t\t// Return the vector as a paragraph\
            \t\tpub fn get_paragraph(&self, config: &Config) -> Paragraph {\
                \t\t\t// Clone the blocks of text\
                \t\t\tlet blocks = self.blocks.clone();\
\
                \t\t\t// Convert the blocks into one text vector\
                \t\t\tlet mut text: Vec<String> = Vec::new();\
\
                \t\t\t// Iterate through the blocks that are currently loaded in\
                \t\t\tfor block in blocks.unwrap().blocks_list {\
                    \t\t\t\t// Add all of the lines in these blocks into the `text` vector\
                    \t\t\t\tblock.content.into_par_iter().collect_into_vec(&mut text);\
                \t\t\t}\
\
                \t\t\t// Create a vector of Lines from the text\
                \t\t\tlet mut lines: Vec<Line> = text\
                    \t\t\t\t.into_par_iter()\
                    \t\t\t\t.enumerate()\
                    \t\t\t\t.map(|(idx, line)| {\
                        \t\t\t\t\t// If the line is empty, return a blank line\
                        \t\t\t\t\tif line.is_empty() {\
                            \t\t\t\t\t\treturn Line::from(String::new());\
                        \t\t\t\t\t}\
                        \t\t\t\t\tself.parse_line(config, idx, &line)\
                    \t\t\t\t})\
                    \t\t\t\t.collect();\
\
                \t\t\t// The current line number in the text\
                \t\t\tlet line_num = self.get_line_num() - self.blocks.as_ref().unwrap().starting_line_num;\
\
                \t\t\t// Highlight the line that the cursor is on\
                \t\t\tlines[line_num] = lines[line_num].clone().style(\
                    \t\t\t\tStyle::default()\
                        \t\t\t\t\t.fg(config.theme.line_highlight_fg_color)\
                        \t\t\t\t\t.bg(config.theme.line_highlight_bg_color),\
                \t\t\t);\
\
                \t\t\t// Return a paragraph from the lines\
                \t\t\tParagraph::new(Text::from(lines)).scroll((self.scroll_offset as u16, 0))\
            \t\t}\
\
            \t\t// TODO: UPDATE FILE LENGTH WHEN DELETING MULTILINE SELECTION\
            \t\t// Delete the highlighted selection of text\
            \t\t/*\
            \t\tfn delete_selection(&mut self) {\
                \t\t\t// Get everything before the selected text on the beginning line\
                \t\t\tlet mut before_selection =\
                    \t\t\t\tString::from(&self.block[self.selection.start[1]][..self.selection.start[0]]);\
                \t\t\t// Get everything after the selected text on the ending line\
                \t\t\tlet after_selection =\
                    \t\t\t\tString::from\
                    \t\t\t\t(&self.block[self.selection.end[1]][self.selection.end[0]..]);\
\
                \t\t\tlet idx = self.selection.start[1] + 1;\
                \t\t\t// Remove the middle lines of the selection\
                \t\t\tfor _i in (self.selection.start[1] + 1)..(self.selection.end[1] + 1) {\
                    \t\t\t\tself.block.remove(idx);\
                \t\t\t}\
\
                \t\t\t// Concat the block after the selection to before the selection\
                \t\t\tbefore_selection.push_str(after_selection.as_str());\
                \t\t\t// Set the line to the new string\
                \t\t\tself.block[self.selection.start[1]] = before_selection;\
\
                \t\t\t// Reset the selection\
                \t\t\tself.selection.is_empty = true;\
\
                \t\t\t// Move cursor back to original Position\
                \t\t\tif self.text_position == self.selection.end {\
                    \t\t\t\tself.text_position = [\
                        \t\t\t\t\tself.selection.original_text_position.0,\
                        \t\t\t\t\tself.selection.original_text_position.1,\
                    \t\t\t\t];\
                    \t\t\t\tself.cursor_position = [\
                        \t\t\t\t\tself.selection.original_cursor_position.0,\
                        \t\t\t\t\tself.selection.original_cursor_position.1,\
                    \t\t\t\t];\
                \t\t\t}\
            \t\t}\
            \t\t*/\
\
            \t\t// Get the key pressed\
            \t\tpub fn handle_input(&mut self, config: &Config) {\
                \t\t\t// Non-blocking read\
                \t\t\tif event::poll(Duration::from_millis(50)).unwrap() {\
                    \t\t\t\t// Read input\
                    \t\t\t\tmatch event::read().unwrap() {\
                        \t\t\t\t\t// Return the character if only a key (without moodifier key) is pressed\
                        \t\t\t\t\tEvent::Key(KeyEvent {\
                            \t\t\t\t\t\tcode,\
                            \t\t\t\t\t\tmodifiers: KeyModifiers::NONE,\
                            \t\t\t\t\t\t..\
                        \t\t\t\t\t}) => {\
                            \t\t\t\t\t\t// Return the key\
                            \t\t\t\t\t\tmatch code {\
                                \t\t\t\t\t\t\t// If normal character, insert that character\
                                \t\t\t\t\t\t\tKeyCode::Char(code) => key_functions::char_key(self, code),\
                                \t\t\t\t\t\t\t// If Enter was pressed, insert newline\
                                \t\t\t\t\t\t\tKeyCode::Enter => key_functions::enter_key(self, config),\
                                \t\t\t\t\t\t\t// If tab was pressed, insert tab character\
                                \t\t\t\t\t\t\tKeyCode::Tab => key_functions::tab_key(self, config),\
                                \t\t\t\t\t\t\t/";