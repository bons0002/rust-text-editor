use super::*;

use std::{
	io::{Error, Read, Seek, SeekFrom},
	str,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

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
		// Number of bytes in a block of text (3 KiB)
		const BLOCK_SIZE: u64 = 3072;
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
			.par_bridge()
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

	// Get the length (in lines) of the specific block_num (even if the block doesn't exist)
	pub fn block_length(editor: &mut EditorSpace, block_num: usize) -> Result<usize, Error> {
		// Create the specific block
		let block = Block::new(editor, block_num)?;

		// Return the length of the block
		match block.ends_with_newline {
			// If the last line is complete, include it
			true => Ok(block.content.len()),
			// Otherwise, don't (it's included in the next block)
			false => Ok(block.content.len() - 1),
		}
	}

	// Get the length (in lines) of the current block
	pub fn get_block_length(&self) -> usize {
		// Return the length of the block
		match self.ends_with_newline {
			// If the last line is complete, include it
			true => self.content.len(),
			// Otherwise, don't (it's included in the next block)
			false => self.content.len() - 1,
		}
	}
}
