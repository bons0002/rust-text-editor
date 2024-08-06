use super::EditorSpace;
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;

mod block;
pub use block::Block;

// Contains blocks of text from a file
#[derive(Clone)]
pub struct Blocks {
	// The ID number of the first block
	head_block: usize,
	// The ID number of the last block
	tail_block: usize,
	// The line number of the first line in the first block
	pub starting_line_num: usize,
	// The number of blocks
	num_blocks: usize,
	// The list of blocks
	pub blocks_list: Vec<Block>,
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

			/* If the last line of the previous block isn't "complete",
			then the first line of the current block isn't "complete" */
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
		// Calculate the line_number of the first line
		let starting_line_num = Block::calc_line_num(editor, block_num)?;
		// Add the current block to the vector of blocks
		blocks = vec![block];

		// Construct the block
		Ok(Blocks {
			head_block: block_num,
			tail_block: block_num,
			starting_line_num,
			num_blocks: 1,
			blocks_list: blocks,
		})
	}

	// Return the head block
	pub fn get_head(&self) -> Block {
		self.blocks_list[0].clone()
	}

	// Insert the previous block at the head of the Blocks (blocks are contiguous here)
	pub fn push_head(&mut self, editor: &mut EditorSpace) -> Result<usize, Error> {
		// Move the starting block to the previous block
		self.head_block -= 1;
		// Create a new block at the new starting block
		let mut block = Block::new(editor, self.head_block)?;
		/* If this new head doesn't end in a "complete" line, remove it
		The previous head would have already had its first line fixed */
		if !block.ends_with_newline {
			block.content.pop();
			// This block now ends with a "complete" line
			block.ends_with_newline = true;
		}
		// Insert this new head block
		self.blocks_list.insert(0, block);
		// Update the starting line number
		self.starting_line_num -= self.get_head().len();
		// Update the number of blocks
		self.num_blocks += 1;
		// Return the block number
		Ok(self.head_block)
	}

	fn pop_head(&mut self) -> usize {
		// Get the length of the first block
		let length = self.get_head().len();
		// Remove the first block
		self.blocks_list.remove(0);

		// There is now one less block
		self.num_blocks -= 1;
		// Move the head to the next block
		self.head_block += 1;
		// Update the starting line number to the beginning of the new head
		self.starting_line_num += length;

		// Return the length of the original head block
		length
	}

	// Insert the next block at the tail of the Blocks (blocks are contiguous here)
	pub fn push_tail(&mut self, editor: &mut EditorSpace) -> Result<usize, Error> {
		// Update the tail block number
		self.tail_block += 1;
		// Create a new block at this new tail position
		let mut block = Block::new(editor, self.tail_block)?;
		// Location of previous tail
		let loc = self.blocks_list.len() - 1;
		// Check if the previous tail ends in a "complete" line
		let prev_block = self.blocks_list[loc].clone();
		// If it doesn't, fix the first line of this new tail
		if !prev_block.ends_with_newline {
			// Construct this fixed line
			let line1 =
				prev_block.content.into_iter().last().clone().unwrap() + block.content[0].as_str();
			// Set the first line to this fixed line
			block.content[0] = line1;
			/* Remove last line of previous tail
			because it is "incomplete" and the first line
			of the next block "completes" it (done above) */
			self.blocks_list[loc].content.pop();
			/* The "incomplete" last line has been removed, so
			the last line is now "complete" and ends with a
			newline character */
			self.blocks_list[loc].ends_with_newline = true;
		}
		// Push this new tail
		self.blocks_list.push(block);
		// Update the number of blocks
		self.num_blocks += 1;

		// Length of the head block
		let mut head_length: usize = 0;
		/* If there are more than three blocks loaded in and the head
		block has not been modified, then remove the head */
		if self.num_blocks > 3 && !self.get_head().is_modified {
			self.pop_head();
			head_length = self.get_head().len();
		}

		// Return the length of the head block (to be removed from the scroll offset)
		Ok(head_length)
	}

	// Return a tuple containing (block number, line number) for accessing the block content
	fn get_location(&self, line_num: usize) -> Option<(usize, usize)> {
		// Track the lines over the blocks
		let mut lines = self.starting_line_num;
		let mut start = lines;
		let mut block_num: Option<usize> = None;
		// Loop until correct block
		for block in &self.blocks_list {
			// Starting line of this block
			start = lines;
			// Starting line of next block
			lines += block.content.len();
			// If the line number is in this block, break loop
			if line_num >= start && line_num < lines {
				block_num = Some(block.block_num);
				break;
			}
		}
		match block_num {
			Some(num) => Some((num - self.head_block, line_num - start)),
			None => None,
		}
	}

	// Insert a character into the correct line in the correct block
	pub fn insert_char_in_line(&mut self, line_num: usize, text_position: usize, character: char) {
		// Make a copy of the blocks
		let blocks = self.clone();
		// Get the (block num, line number) location
		let location = match blocks.get_location(line_num) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};
		// Insert the character into the correct block on the correct line
		self.blocks_list[location.0].content[location.1].insert(text_position, character);

		// Set this block as modified
		self.blocks_list[location.0].is_modified = true;
	}

	// Insert a newline and truncate the current line
	pub fn insert_new_line(&mut self, line_num: usize, text_position: usize) {
		// Make a copy of the blocks
		let blocks = self.clone();
		// Get the (block num, line number) location
		let location = match blocks.get_location(line_num) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};

		// The text of the current line
		let text = self.blocks_list[location.0].content[location.1].clone();
		// Get the rest of the line after the cursor
		let after_cursor = &text[text_position..];

		// Insert new row
		self.blocks_list[location.0]
			.content
			.insert(location.1 + 1, String::from(after_cursor));
		// Remove the rest of the old row after the enter
		self.blocks_list[location.0].content[location.1].truncate(text_position);

		// Set this block as modified
		self.blocks_list[location.0].is_modified = true;
	}

	// Delete a character from the given line at the given position
	pub fn delete_char_in_line(&mut self, line_num: usize, text_position: usize) {
		// Make a copy of the blocks
		let blocks = self.clone();
		// Get the (block num, line number) location
		let location = match blocks.get_location(line_num) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};

		// Remove a character from the line
		self.blocks_list[location.0].content[location.1].remove(text_position);

		// Set this block as modified
		self.blocks_list[location.0].is_modified = true;
	}

	// Delete the below line and append its text content to the end of the current line
	pub fn delete_line(&mut self, line_num: usize) {
		// Make a copy of the blocks
		let blocks = self.clone();
		// Get the (block num, line number) location
		let prev_location = match blocks.get_location(line_num + 1) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};

		// The text of the current line
		let text = self.blocks_list[prev_location.0].content[prev_location.1].clone();
		// Get the rest of the line after the cursor
		let after_cursor = &text[0..];

		// Get the (block num, line number) location
		let curr_location = match blocks.get_location(line_num) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};

		// Remove the below line
		self.blocks_list[prev_location.0]
			.content
			.remove(prev_location.1);

		// Append the rest of the below line to the current line (where the cursor is moving to)
		self.blocks_list[curr_location.0].content[curr_location.1].push_str(after_cursor);

		// Set both blocks as modified
		self.blocks_list[prev_location.0].is_modified = true;
		self.blocks_list[curr_location.0].is_modified = true;
	}

	// Return the line at the given line number
	pub fn get_line(&self, line_num: usize) -> String {
		// Make a copy of the blocks
		let blocks = self.clone();
		// Get the (block num, line number) location
		let location = match blocks.get_location(line_num) {
			Some(location) => location,
			None => panic!("Couldn't retrieve location"),
		};

		// Return a copy of the line
		self.blocks_list[location.0].content[location.1].clone()
	}

	// Return the length of the specified line
	pub fn get_line_length(&self, line_num: usize) -> usize {
		self.get_line(line_num).graphemes(true).count()
	}

	// The number of lines in the entire Blocks
	pub fn len(&self) -> usize {
		// Clone the blocks
		let blocks = self.blocks_list.clone();
		// Variable to track the total length of all the blocks
		let mut length = 0;
		// Loop through the blocks
		for block in blocks {
			// Update the total length
			length += block.len();
		}
		length
	}
}
