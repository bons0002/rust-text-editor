use super::EditorSpace;
use std::io::Error;

mod block;
pub use block::Block;

// Contains blocks of text from a file
pub struct Blocks {
	// The ID number of the first block
	pub starting_block: usize,
	// The number of blocks
	num_blocks: usize,
	// The list of blocks
	pub blocks: Vec<Block>,
	// Flag to check if any of the blocks have been edited
	// If the is false, then the blocks can be refreshed freely
	pub is_modified: bool,
}

impl Blocks {
	// Create a new Blocks struct with all blocks between starting and ending blocks (inclusive)
	pub fn new(editor: &mut EditorSpace, block_num: usize) -> Result<Self, Error> {
		// The vector of blocks
		let blocks: Vec<Block>;
		// The current block
		let mut block: Block;

		// If non-zero block number
		if block_num > 0 {
			// Construct the previous block
			let prev_block = Block::new(editor, block_num - 1)?;
			// Construct the current block
			block = Block::new(editor, block_num)?;

			// If the last line of the previous block isn't "complete",
			// then the first line of the current block isn't "complete"
			if !prev_block.ends_with_newline {
				// Construct a "complete" line
				let line1 = prev_block.content[prev_block.content.len() - 1].clone()
					+ block.content[0].as_str();
				// Set the first line of the current block to this corrected line
				block.content[0] = line1;
			}
		// If zero block number, construct the first block
		} else {
			// Construct block 0
			block = Block::new(editor, block_num)?;
		}
		// Add the current block to the vector of blocks
		blocks = vec![block];

		// Construct the block
		Ok(Blocks {
			starting_block: block_num,
			num_blocks: 1,
			blocks,
			is_modified: false,
		})
	}

	// Calculate the block number of a block containing the given line number
	pub fn calc_block_num(editor: &mut EditorSpace, line_num: usize) -> Result<usize, Error> {
		// Track the current block
		let mut block_num = 0;
		// Track the total length up to the current block
		let mut total_length = 0;

		// Loop until the correct block is found
		loop {
			// Add to the running total of the length of blocks
			total_length += Block::block_length(editor, block_num)?;
			// If the length becomes larger than the line number, then return this
			// block becuause it is the first block to contain the line number
			if total_length > line_num {
				return Ok(block_num);
			}
			// Otherwise, continue on with the next block
			block_num += 1;
		}
	}

	// Insert the previous block at the head of the Blocks (blocks are contiguous here)
	pub fn insert_head(&mut self, editor: &mut EditorSpace) -> Result<usize, Error> {
		// Move the starting block to the previous block
		self.starting_block -= 1;
		// Create a new block at the new starting block
		let mut block = Block::new(editor, self.starting_block)?;
		// If this new head doesn't end in a "complete" line, remove it
		// The previous head would have already had its first line fixed
		if !block.ends_with_newline {
			block.content.pop();
		}
		// Insert this new head block
		self.blocks.insert(0, block);
		// Return the block number
		Ok(self.starting_block)
	}

	// Insert the next block at the tail of the Blocks (blocks are contiguous here)
	pub fn insert_tail(&mut self, editor: &mut EditorSpace) -> Result<usize, Error> {
		// Get the block number of the new tail
		let tail = self.starting_block + self.blocks.len();
		// Create a new block at this new tail position
		let mut block = Block::new(editor, tail)?;
		// Check if the previous tail ends in a "complete" line
		let prev_block = self.blocks[self.blocks.len() - 1].clone();
		// If it doesn't, fix the first line of this new tail
		if !prev_block.ends_with_newline {
			// Construct this fixed line
			let line1 = prev_block.content[prev_block.content.len() - 1].clone()
				+ block.content[0].as_str();
			// Set the first line to this fixed line
			block.content[0] = line1;
		}
		// Push this new tail
		self.blocks.push(block);
		// Return this block number
		Ok(tail)
	}
}
