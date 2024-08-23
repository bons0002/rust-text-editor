use super::*;

use std::{
	io::{Error, Read, Seek, SeekFrom},
	str,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

// Number of bytes in a block of text (5 KiB)
pub const BLOCK_SIZE: u64 = 5120;

#[derive(Clone)]
pub struct TextBlock {
	// ID number of the current block
	pub block_num: usize,
	// The text content of the current block
	pub content: Vec<String>,
	// Flag that tracks whether this block has been modified
	pub is_modified: bool,
	// Length of the Block (in lines)
	pub len: usize,
}

impl TextBlock {
	/* Parse a buffer of bytes from the text file to
	a vector of lines of text (as Strings). */
	fn parse_content(
		editor: &mut EditorSpace,
		block_num: usize,
		buffer: &mut [u8; BLOCK_SIZE as usize],
	) -> Result<Vec<String>, Error> {
		// Move to the position within the file for this block
		let _seek = editor
			.file
			.seek(SeekFrom::Start((block_num as u64) * BLOCK_SIZE))?;

		// Read in bytes
		let num_bytes = editor.file.read(buffer)?;

		/* Parse bytes to String vector (with newlines intact)
		and return it. */
		Ok(str::from_utf8(&buffer[..num_bytes])
			.unwrap()
			.split_inclusive('\n')
			.map(String::from)
			.collect())
	}

	/* Complete the first line of this block if the previous
	block doens't end in a newline (current first line is only
	part of a line). */
	fn fix_first_line(
		editor: &mut EditorSpace,
		buffer: &mut [u8; BLOCK_SIZE as usize],
		block_num: usize,
		content: &mut [String],
	) -> Result<usize, Error> {
		// Move to the position within the file for this block
		let _seek = editor
			.file
			.seek(SeekFrom::Start(((block_num - 1) as u64) * BLOCK_SIZE))?;
		// Read in bytes
		let num_bytes = editor.file.read(buffer)?;
		// Parse bytes to String vector (with newlines intact)
		let prev_block_content = String::from(str::from_utf8(&buffer[..num_bytes]).unwrap());
		// Check if the previous block ends in a "complete" line
		let prev_newline = prev_block_content.ends_with('\n');
		// If it doesn't end in a newline, fix the first line of this block
		if !prev_newline {
			// The starting index of the last line of the previous block
			let last_line_start = prev_block_content.match_indices('\n').last().unwrap().0 + 1;
			// Construct a "complete" line
			let line1 = String::from(&prev_block_content[last_line_start..]) + content[0].as_str();
			// Set the first line of the block to this "fixed" first line
			content[0] = line1;
		}

		Ok(0)
	}

	// Get the length (in lines) of the current block
	fn calc_len(&self) -> usize {
		self.content.len()
	}

	// Create a new, incomplete block from the given content vector
	fn construct_block(
		ends_with_newline: bool,
		block_num: usize,
		max_blocks: usize,
		content: &mut Vec<String>,
	) -> TextBlock {
		// If the last line is incomplete, remove it
		if !ends_with_newline
			//&& editor.file_length >= (editor.height.1 - editor.height.0)
			&& block_num < max_blocks - 1
		{
			content.pop();
		}
		// Push a blank new line if the last block ends in a newline char
		if ends_with_newline && block_num == max_blocks - 1 {
			content.push(String::from(""));
		}
		// Trim the newlines
		let content = content
			.into_par_iter()
			.map(|line| String::from(line.trim_end()))
			.collect();
		// Create and return the block
		let mut block = TextBlock {
			block_num,
			content,
			// Can't be modified if new
			is_modified: false,
			len: 0,
		};
		// Calculate the length of the block
		block.len = block.calc_len();

		// Return the block
		block
	}

	/* Create a new block.
	This function is disgustingly long. */
	pub fn new(
		editor: &mut EditorSpace,
		block_num: usize,
		max_blocks: usize,
	) -> Result<Self, Error> {
		// Buffer that the bytes of the file are read into
		let mut buffer = [0; BLOCK_SIZE as usize];

		// Get the lines of text from the file
		let mut content = Self::parse_content(editor, block_num, &mut buffer)?;

		// For any block after the first one
		if block_num > 0 {
			// Fix the first line of the block if necessary
			Self::fix_first_line(editor, &mut buffer, block_num, &mut content)?;
		}

		// Check if the last line ends with a newline
		let ends_with_newline = match content.last() {
			Some(line) => line.ends_with('\n'),
			None => true,
		};

		// Create an unfinished block
		let block = Self::construct_block(ends_with_newline, block_num, max_blocks, &mut content);

		// Return the block
		Ok(block)
	}

	// Calculate the starting line number of a block of text
	pub fn calc_line_num(
		editor: &mut EditorSpace,
		block_num: usize,
		max_blocks: usize,
	) -> Result<usize, Error> {
		let mut current_block = 0;
		// Total length of all blocks before the current one
		let mut total_length = 0;
		// Loop until the given block number is reached
		while current_block < block_num {
			// Construct a block
			let block = TextBlock::new(editor, current_block, max_blocks)?;
			// Update the total length of blocks
			total_length += block.len;
			// Update the current block to be counted
			current_block += 1;
		}

		// Return the line number
		Ok(total_length)
	}
}
