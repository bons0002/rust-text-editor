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
	// Get the length (in lines) of the current block
	fn calc_len(&self) -> usize {
		self.content.len()
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

		// Move to the position within the file for this block
		let _seek = editor
			.file
			.seek(SeekFrom::Start((block_num as u64) * BLOCK_SIZE))?;
		// Read in bytes
		let num_bytes = editor.file.read(&mut buffer)?;
		// Parse bytes to String vector (with newlines intact)
		let mut content: Vec<String> = str::from_utf8(&buffer[..num_bytes])
			.unwrap()
			.split_inclusive('\n')
			.map(String::from)
			.collect();
		// Check if the last line ends with a newline
		let ends_with_newline = match content.last() {
			Some(line) => line.ends_with('\n'),
			None => true,
		};

		// For any block after the first one
		if block_num > 0 {
			// Move to the position within the file for this block
			let _seek = editor
				.file
				.seek(SeekFrom::Start(((block_num - 1) as u64) * BLOCK_SIZE))?;
			// Read in bytes
			let num_bytes = editor.file.read(&mut buffer)?;
			// Parse bytes to String vector (with newlines intact)
			let prev_block_content: Vec<String> = str::from_utf8(&buffer[..num_bytes])
				.unwrap()
				.split_inclusive('\n')
				.map(String::from)
				.collect();
			// Check if the previous block ends in a "complete" line
			let prev_newline = match prev_block_content.last() {
				Some(line) => line.ends_with('\n'),
				None => true,
			};
			// If it doesn't end in a newline, fix the first line of this block
			if !prev_newline {
				// Construct a "complete" line
				let line1 = prev_block_content.last().unwrap().clone() + content[0].as_str();
				// Set the first line of the block to this "fixed" first line
				content[0] = line1;
			}
		}
		// If the last line is incomplete, remove it
		if !ends_with_newline
			&& editor.file_length >= (editor.height.1 - editor.height.0)
			&& block_num < max_blocks - 1
		{
			content.pop();
		}
		// Trim the newlines
		let content = content
			.into_par_iter()
			.map(|line| String::from(line.trim_end()))
			.collect();
		// Create the block
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
