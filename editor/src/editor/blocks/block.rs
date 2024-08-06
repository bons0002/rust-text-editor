use super::*;

use std::{
	io::{Error, Read, Seek, SeekFrom},
	str,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone)]
pub struct Block {
	// ID number of the current block
	pub block_num: usize,
	// The text content of the current block
	pub content: Vec<String>,
	// Flag for whether the last line ends in a newline (complete line)
	pub ends_with_newline: bool,
}

impl Block {
	// Create a new block
	pub fn new(editor: &mut EditorSpace, block_num: usize) -> Result<Self, Error> {
		// Number of bytes in a block of text (5 KiB)
		const BLOCK_SIZE: u64 = 5120;
		// Buffer that the bytes of the file are read into
		let mut buffer = [0; BLOCK_SIZE as usize];

		// Move to the position within the file for this block
		let _seek = editor
			.file
			.seek(SeekFrom::Start((block_num as u64) * BLOCK_SIZE))?;

		// Read in bytes
		let num_bytes = editor.file.read(&mut buffer)?;
		// Parse bytes to String vector (with newlines intact)
		let content: Vec<String> = str::from_utf8(&buffer[..num_bytes])
			.unwrap()
			.split_inclusive("\n")
			.into_iter()
			.map(String::from)
			.collect();

		// Check if the last line ends with a newline
		let ends_with_newline = content[content.len() - 1].ends_with("\n");

		// Trim the newlines
		let content = content
			.into_par_iter()
			.map(|line| String::from(line.trim_end()))
			.collect();

		// Return the block
		Ok(Block {
			block_num,
			content,
			ends_with_newline,
		})
	}

	// Calculate the starting line number of a block of text
	pub fn calc_line_num(editor: &mut EditorSpace, block_num: usize) -> Result<usize, Error> {
		let mut current_block = 0;
		// Total length of all blocks before the current one
		let mut total_length = 0;
		// Loop until the given block number is reached
		while current_block < block_num {
			// Construct a block
			let block = Block::new(editor, current_block)?;
			// Update the total length of blocks
			total_length += block.len();
			// Update the current block to be counted
			current_block += 1;
		}

		// Return the line number
		Ok(total_length)
	}

	// Get the length (in lines) of the current block
	pub fn len(&self) -> usize {
		// Return the length of the block
		match self.ends_with_newline {
			// If the last line is complete, include it
			true => self.content.len(),
			// Otherwise, don't (it's included in the next block)
			false => self.content.len() - 1,
		}
	}
}
