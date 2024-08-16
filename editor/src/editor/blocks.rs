use super::EditorSpace;
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;

mod block;
pub use block::Block;

// Contains blocks of text from a file
#[derive(Clone)]
pub struct Blocks {
	// The ID number of the first block
	pub head_block: usize,
	// The ID number of the last block
	pub tail_block: usize,
	// The line number of the first line in the first block
	pub starting_line_num: usize,
	// The maximum number of blocks for the file
	pub max_blocks: usize,
	// The number of blocks
	num_blocks: usize,
	// The list of blocks
	pub blocks_list: Vec<Block>,
}

impl Blocks {
	// Create a new Blocks struct with all blocks between starting and ending blocks (inclusive)
	pub fn new(editor: &mut EditorSpace, block_num: usize) -> Result<Self, Error> {
		// Ensure that the metadata of the file is up to date
		editor.file.sync_all()?;
		// Get the number of bytes in the file
		let size = editor.file.metadata()?.len() as usize;
		// Find the max number of blocks for this file
		let max_blocks = size.div_ceil(block::BLOCK_SIZE as usize);
		// Construct the block
		Ok(Blocks {
			head_block: block_num,
			tail_block: block_num,
			// Calculate the line number of the first line
			starting_line_num: Block::calc_line_num(editor, block_num)?,
			num_blocks: 1,
			max_blocks,
			// Add the current block to the vector of blocks
			blocks_list: vec![Block::new(editor, block_num)?],
		})
	}

	// Return the head block
	pub fn get_head(&self) -> Block {
		self.blocks_list[0].clone()
	}

	// Return the tail Block
	fn get_tail(&self) -> Block {
		self.blocks_list.iter().last().unwrap().clone()
	}

	// Remove the tail Block
	fn pop_tail(&mut self) {
		// Reduce the number of blocks
		self.num_blocks -= 1;
		// Move the tail
		self.tail_block -= 1;
		// Remove the tail
		self.blocks_list.pop();
	}

	// Insert the previous block at the head of the Blocks (blocks are contiguous here)
	pub fn push_head(&mut self, editor: &mut EditorSpace) -> Result<usize, Error> {
		// Make sure that too many blocks aren't loaded
		if self.num_blocks < self.max_blocks {
			// Move the starting block to the previous block
			self.head_block -= 1;
			// Create a new block at the new starting block
			let block = Block::new(editor, self.head_block)?;
			// Insert this new head block
			self.blocks_list.insert(0, block);
			// Update the starting line number
			self.starting_line_num -= self.get_head().len();
			// Update the number of blocks
			self.num_blocks += 1;

			/* If there are more than three blocks loaded in and the tail
			block has not been modified, then remove the tail.
			Also, if there is a highlighted selection, don't unload blocks. */
			if self.num_blocks > 3 && !self.get_tail().is_modified && editor.selection.is_empty {
				self.pop_tail();
			}
		}

		// Return the block number
		Ok(self.head_block)
	}

	// Remove the head Block
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
	// Returns the tail block number if successful and -1 if failure
	pub fn push_tail(&mut self, editor: &mut EditorSpace) -> Result<isize, Error> {
		// Make sure that too many blocks aren't loaded
		if self.num_blocks < self.max_blocks {
			// Update the tail block number
			self.tail_block += 1;
			// Create a new block at this new tail position
			let block = Block::new(editor, self.tail_block)?;
			// Push this new tail
			self.blocks_list.push(block);
			// Update the number of blocks
			self.num_blocks += 1;

			// Length of the head block
			let head_length: usize = self.get_head().len();
			/* If there are more than three blocks loaded in and the head
			block has not been modified, then remove the head.
			Also, if there is a highlighted selection, don't unload blocks. */
			if self.num_blocks > 3 && !self.get_head().is_modified && editor.selection.is_empty {
				self.pop_head();
				// Subtract length of original head from scroll offset
				editor.scroll_offset -= head_length;
			}
			// No error
			return Ok(self.tail_block as isize);
		}
		// Error
		Ok(-1)
	}

	// Return a tuple containing (block number, line number) for accessing the block content
	pub fn get_location(&self, line_num: usize) -> Result<(usize, usize), Error> {
		// Track the total lines over the blocks
		let mut lines = self.starting_line_num;
		// The starting line
		let mut start = lines;
		let mut block_num: Option<usize> = None;
		// Loop until within the correct block
		for block in &self.blocks_list {
			// Skip over empty blocks
			if block.content.is_empty() {
				continue;
			}
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
		// Return (block number, line number within block)
		match block_num.map(|num| (num - self.head_block, line_num - start)) {
			Some(location) => Ok(location),
			None => Err(Error::other(format!(
				/* Return the source file name, line number error occurred in this source file,
				and line_num argument that was passed to this function. */
				"{}::get_location: line {}. Couldn't get location for `line_num = {}`",
				file!(),
				line!(),
				line_num
			))),
		}
	}

	// Insert a character into the correct line in the correct block
	// Returns true if successful
	pub fn insert_char_in_line(
		&mut self,
		line_num: usize,
		text_position: usize,
		character: char,
	) -> Result<bool, Error> {
		// Get the (block num, line number) location
		let location = self.get_location(line_num)?;
		// Insert the character into the correct block on the correct line
		self.blocks_list[location.0].content[location.1].insert(text_position, character);

		// Set this block as modified
		self.blocks_list[location.0].is_modified = true;

		// Return true to denote success
		Ok(true)
	}

	// Insert a newline and truncate the current line (returns true if successful)
	pub fn insert_new_line(
		&mut self,
		line_num: usize,
		text_position: usize,
	) -> Result<bool, Error> {
		// Get the (block num, line number) location
		let location = self.get_location(line_num)?;

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

		// Return true if no error
		Ok(true)
	}

	// Delete a character from the given line at the given position
	pub fn delete_char_in_line(
		&mut self,
		line_num: usize,
		text_position: usize,
	) -> Result<bool, Error> {
		// Get the (block num, line number) location
		let location = self.get_location(line_num)?;

		// Remove a character from the line
		self.blocks_list[location.0].content[location.1].remove(text_position);

		// Set this block as modified
		self.blocks_list[location.0].is_modified = true;

		// Return true to denote no error
		Ok(true)
	}

	// Fully delete the given line
	pub fn delete_line(&mut self, line_num: usize) -> Result<String, Error> {
		// Get the (block num, line num) location of the below line
		let location = self.get_location(line_num)?;

		// Set block as modified
		self.blocks_list[location.0].is_modified = true;

		// Remove (and return) the below line
		Ok(self.blocks_list[location.0].content.remove(location.1))
	}

	// Delete the below line and append its text content to the end of the current line
	// Returns true if successful
	pub fn delete_and_append_line(&mut self, line_num: usize) -> Result<bool, Error> {
		// Delete the below line
		let text = self.delete_line(line_num + 1)?;

		// Get the rest of the line after the cursor
		let after_cursor = &text[0..];

		// Get the (block num, line number) location
		let curr_location = self.get_location(line_num)?;

		// Append the rest of the below line to the current line (where the cursor is moving to)
		self.blocks_list[curr_location.0].content[curr_location.1].push_str(after_cursor);

		// Set the current block as modified
		self.blocks_list[curr_location.0].is_modified = true;

		// Return true to denote no error
		Ok(true)
	}

	// Return the line at the given line number
	pub fn get_line(&self, line_num: usize) -> Result<String, Error> {
		// Get the (block num, line number) location
		let location = self.get_location(line_num)?;

		// Return a copy of the line
		Ok(self.blocks_list[location.0].content[location.1].clone())
	}

	// Set the line in the Blocks (returns true if successful)
	pub fn set_line(&mut self, line_num: usize, text: &str) -> Result<bool, Error> {
		// Get the (block num, line number) location
		let location = self.get_location(line_num)?;

		// Set the line in the block to the given line
		self.blocks_list[location.0].content[location.1] = String::from(text);

		// Set block as modified
		self.blocks_list[location.0].is_modified = true;

		// Return true to denote no error
		Ok(true)
	}

	// Return the length of the specified line
	pub fn get_line_length(&self, line_num: usize) -> Result<usize, Error> {
		Ok(self.get_line(line_num)?.graphemes(true).count())
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
