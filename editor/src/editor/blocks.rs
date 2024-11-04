use super::EditorSpace;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;

mod text_block;
pub use text_block::TextBlock;

// Contains blocks of text from a file
#[derive(Clone, Debug)]
pub struct Blocks {
	// The list of blocks
	pub blocks_list: Vec<TextBlock>,
	// The current [block_num, line_num (within block)]
	pub curr_position: [usize; 2],
	// The ID number of the first block
	pub head_block: usize,
	// The maximum number of blocks for the file
	pub max_blocks: usize,
	// The number of blocks
	num_blocks: usize,
	// The line number of the first line in the first block
	pub starting_line_num: usize,
	// The ID number of the last block
	pub tail_block: usize,
}

impl Blocks {
	// Create a new Blocks struct with all blocks between starting and ending blocks (inclusive)
	pub fn new(editor: &mut EditorSpace, block_num: usize, line_num: usize) -> Result<Self, Error> {
		// Ensure that the metadata of the file is up to date
		editor.file.sync_all()?;
		// Get the number of bytes in the file
		let size = editor.file.metadata()?.len() as usize;
		// Find the max number of blocks for this file
		let mut max_blocks = size.div_ceil(text_block::BLOCK_SIZE as usize);
		// Can't have max of 0 blocks
		if max_blocks == 0 {
			max_blocks += 1;
		}
		// Construct the block
		let mut blocks = Blocks {
			// Add the current block to the vector of blocks
			blocks_list: vec![TextBlock::new(editor, block_num, max_blocks)?],
			curr_position: [0, 0],
			head_block: block_num,
			num_blocks: 1,
			max_blocks,
			// Calculate the line number of the first line
			starting_line_num: TextBlock::calc_line_num(editor, block_num, max_blocks)?,
			tail_block: block_num,
		};
		// The current location in the block
		let location = blocks.get_location(line_num)?;
		// Update the tracked current location
		blocks.curr_position = [location.0, location.1];

		Ok(blocks)
	}

	// Create a new Blocks from a line number rather than a block number
	pub fn from_line(editor: &mut EditorSpace, line_num: usize) -> Result<Self, Error> {
		// Load in all blocks to find the block for the line number
		let block_num = Self::calc_block_num(editor, line_num)?;
		// Return the Blocks
		Blocks::new(editor, block_num, line_num)
	}

	// Return the head block
	pub fn get_head(&self) -> TextBlock {
		self.blocks_list[0].clone()
	}

	// Return the tail block
	fn get_tail(&self) -> TextBlock {
		self.blocks_list.iter().last().unwrap().clone()
	}

	// Insert the previous block at the head of the Blocks (blocks are contiguous here)
	pub fn push_head(
		&mut self,
		editor: &mut EditorSpace,
		can_unload: bool,
	) -> Result<usize, Error> {
		// Make sure that too many blocks aren't loaded
		if self.num_blocks < self.max_blocks {
			// Move the starting block to the previous block
			self.head_block -= 1;
			// Create a new block at the new starting block
			let block = TextBlock::new(editor, self.head_block, self.max_blocks)?;
			// Insert this new head block
			self.blocks_list.insert(0, block);
			// Update the starting line number
			self.starting_line_num -= self.get_head().len;
			// Update the number of blocks
			self.num_blocks += 1;
			// Increment the block number tracker
			self.curr_position[0] += 1;

			/* If there are more than (15KiB / BLOCK_SIZE) blocks loaded in and the tail
			block has not been modified, then remove the tail.
			Also, if there is a highlighted selection, don't unload blocks. */
			if self.num_blocks > (15360_usize.div_ceil(text_block::BLOCK_SIZE as usize))
				&& !self.get_tail().is_modified
				&& editor.selection.is_empty
				&& can_unload
			{
				self.pop_tail();
			}
		}

		// Return the block number
		Ok(self.head_block)
	}

	// Insert the next block at the tail of the Blocks (blocks are contiguous here)
	// Returns the tail block number if successful and -1 if failure
	pub fn push_tail(
		&mut self,
		editor: &mut EditorSpace,
		can_unload: bool,
	) -> Result<isize, Error> {
		// Make sure that too many blocks aren't loaded
		if self.num_blocks < self.max_blocks {
			// Update the tail block number
			self.tail_block += 1;
			// Create a new block at this new tail position
			let block = TextBlock::new(editor, self.tail_block, self.max_blocks)?;
			// Push this new tail
			self.blocks_list.push(block);
			// Update the number of blocks
			self.num_blocks += 1;

			/* If there are more than (15KiB / BLOCK_SIZE) blocks	 loaded in and the head
			block has not been modified, then remove the head.
			Also, if there is a highlighted selection, don't unload blocks. */
			if self.num_blocks > (15360_usize.div_ceil(text_block::BLOCK_SIZE as usize))
				&& !self.get_head().is_modified
				&& editor.selection.is_empty
				&& can_unload
			{
				let head_length = self.pop_head();
				// Subtract length of original head from scroll offset
				editor.scroll_offset -= head_length;
				// Reduce the tracked block number
				self.curr_position[0] -= 1;
			}
			// No error
			return Ok(self.tail_block as isize);
		}
		// Error
		Ok(-1)
	}

	// Insert a character into the correct line in the correct block
	pub fn insert_char_in_line(&mut self, text_position: usize, character: char) {
		// Get the (block num, line number) location
		let [block_num, line_num] = self.curr_position;
		// Insert the character into the correct block on the correct line
		self.blocks_list[block_num].content[line_num].insert(text_position, character);

		// Set this block as modified
		self.blocks_list[block_num].is_modified = true;
	}

	// Insert a newline and truncate the current line (returns true if successful)
	pub fn insert_new_line(&mut self, text_position: usize) {
		// Get the (block num, line number) location
		let [block_num, line_num] = self.curr_position;

		// The text of the current line
		let text = self.blocks_list[block_num].content[line_num].clone();
		// Get the rest of the line after the cursor
		let after_cursor = &text[text_position..];

		// Insert new row
		self.blocks_list[block_num]
			.content
			.insert(line_num + 1, String::from(after_cursor));
		// Remove the rest of the old row after the enter
		self.blocks_list[block_num].content[line_num].truncate(text_position);

		// Set this block as modified
		self.blocks_list[block_num].is_modified = true;

		// Increase the length of the Block
		self.blocks_list[block_num].len += 1;
	}

	// Delete a character from the given line at the given position
	pub fn delete_char_in_line(&mut self, text_position: usize) {
		// Get the (block num, line number) location
		let [block_num, line_num] = self.curr_position;

		// Get the line as graphemes
		let line: Vec<&str> = self.blocks_list[block_num].content[line_num]
			.grapheme_indices(true)
			.filter_map(|(pos, text)| {
				if pos != text_position {
					Some(text)
				} else {
					None
				}
			})
			.collect();

		// Recreate the line as a string
		let mut line_str = String::new();
		line_str.extend(line.iter().copied());
		// Set the line in the block to this new line
		self.blocks_list[block_num].content[line_num] = line_str;

		// Set this block as modified
		self.blocks_list[block_num].is_modified = true;
	}

	// Delete the given line
	pub fn delete_line(&mut self, line_num: usize) -> Result<String, Error> {
		// Get the (block num, line num) location of the below line
		let (block_num, line_num) = self.get_location(line_num)?;

		// Set block as modified
		self.blocks_list[block_num].is_modified = true;

		// Reduce the length of this block
		self.blocks_list[block_num].len -= 1;

		// Remove (and return) the below line
		Ok(self.blocks_list[block_num].content.remove(line_num))
	}

	// Delete the below line and append its text content to the end of the current line
	// Returns true if successful
	pub fn delete_and_append_line(&mut self, line_num: usize) -> Result<(), Error> {
		// Delete this lower line
		let text = self.delete_line(line_num + 1)?;
		// Get the rest of the line after the cursor
		let after_cursor = &text[0..];

		// Get the (block num, line number) location
		let [block_num, line_num] = self.curr_position;
		// Append the rest of the below line to the current line (where the cursor is moving to)
		self.blocks_list[block_num].content[line_num].push_str(after_cursor);

		// Set the current block as modified
		self.blocks_list[block_num].is_modified = true;

		Ok(())
	}

	// Return the current line
	pub fn get_current_line(&self) -> String {
		// Get the (block num, line number) location
		let [block_num, line_num] = self.curr_position;

		// Return a copy of the line
		match self.blocks_list[block_num].content.get(line_num) {
			Some(line) => line.clone(),
			None => String::new(),
		}
	}

	// Return the line for the given line number (Slower than get_current_line)
	pub fn get_some_line(&self, line_num: usize) -> Result<String, Error> {
		// Get the (block num, line number) location
		let (block_num, line_num) = self.get_location(line_num)?;

		// Return a copy of the line
		Ok(match self.blocks_list[block_num].content.get(line_num) {
			Some(line) => line.clone(),
			None => String::new(),
		})
	}

	// The number of lines in the entire Blocks
	pub fn len(&self) -> usize {
		/* Map reduce to sum the length of all the blocks' lengths.
		Not a huge advantage to parallelize this most of the time, but if
		a lot of blocks are loaded in at once, this could provide a small
		performance boost. */
		self.blocks_list
			.par_iter()
			.map(|block| block.len)
			.reduce(|| 0, |a, b| a + b)
	}

	// Update the line the cursor is on in the Blocks
	pub fn update_current_line(&mut self, text: String) {
		// Get the location of the line that needs to be updated
		let [block_num, line_num] = self.curr_position;
		// Update the line
		self.blocks_list[block_num].content[line_num] = text;
	}

	// Update the given line (slower than current line)
	pub fn update_some_line(&mut self, text: String, line_num: usize) -> Result<(), Error> {
		// Get the location of the line that needs to be updated
		let (block_num, line_num) = self.get_location(line_num)?;
		// Update the line
		self.blocks_list[block_num].content[line_num] = text;

		Ok(())
	}

	// Check that the Blocks is valid for the current widget
	pub fn check_blocks(&mut self, editor: &mut EditorSpace) {
		/* If the Blocks is too short, but there is more text to be shown,
		add a new TextBlock to the tail. */
		if self.len() < editor.height + editor.scroll_offset
			&& editor.file_length > editor.height
			&& self.tail_block < self.max_blocks - 1
		{
			// Add new tail block
			self.push_tail(editor, true).unwrap();
		}
	}

	// Load in all TextBlocks of a file into one Blocks
	pub fn load_all_blocks(&mut self, editor: &mut EditorSpace) {
		// The block number of the head and tail blocks respectively
		let (head_block, tail_block) = (self.head_block, self.tail_block);

		// Load in all blocks in the file that aren't currently in the Blocks
		for i in 0..self.max_blocks {
			// Don't bother with blocks that are already loaded in
			if i >= head_block && i <= tail_block {
				continue;
			// Load in blocks before the head block
			} else if i < head_block {
				match self.push_head(editor, false) {
					Ok(_) => (),
					Err(err) => {
						panic!("{}", err);
					}
				}
			// Load in blocks after the tail block
			} else if i > tail_block {
				match self.push_tail(editor, false) {
					Ok(_) => (),
					Err(err) => {
						panic!("{}", err);
					}
				}
			}
		}
	}

	// Return a tuple containing (block number, line number) for accessing the block content
	fn calc_block_num(editor: &mut EditorSpace, line_num: usize) -> Result<usize, Error> {
		// Create a new Blocks
		let blocks = Blocks::new(editor, 0, 0)?;
		// The max number of blocks
		let max_blocks = blocks.max_blocks;
		// Track the total lines over the blocks
		let mut lines = 0;
		// The starting line
		let mut start = lines;
		let mut block_num = 0;
		// Loop until within the correct block
		while line_num >= start && line_num < lines {
			// Create a block for the current block number
			let block = TextBlock::new(editor, block_num, max_blocks)?;
			// Increment block number
			block_num += 1;
			// Skip over empty blocks
			if block.len == 0 {
				continue;
			}
			// Starting line of this block
			start = lines;
			// Starting line of next block
			lines += block.len;
		}
		// Return the block number
		Ok(block_num)
	}

	// Return a tuple containing (block number, line number) for accessing the block content
	fn get_location(&self, line_num: usize) -> Result<(usize, usize), Error> {
		// Track the total lines over the blocks
		let mut lines = self.starting_line_num;
		// The starting line
		let mut start = lines;
		let mut block_num: Option<usize> = None;
		// Loop until within the correct block
		for block in &self.blocks_list {
			// Skip over empty blocks
			if block.len == 0 {
				continue;
			}
			// Starting line of this block
			start = lines;
			// Starting line of next block
			lines += block.len;
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

	// Remove the tail block
	fn pop_tail(&mut self) {
		// Reduce the number of blocks
		self.num_blocks -= 1;
		// Move the tail
		self.tail_block -= 1;
		// Remove the tail
		self.blocks_list.pop();
	}

	// Remove the head Block
	fn pop_head(&mut self) -> usize {
		// Get the length of the first block
		let length = self.get_head().len;
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
}

impl PartialEq for Blocks {
	fn eq(&self, other: &Self) -> bool {
		// If any of the Blocks fields don't align, return false
		if (self.head_block != other.head_block)
			|| (self.tail_block != other.tail_block)
			|| (self.starting_line_num != other.starting_line_num)
			|| (self.max_blocks != other.max_blocks)
			|| (self.num_blocks != other.num_blocks)
			|| (self.blocks_list != other.blocks_list)
		{
			return false;
		}
		// Otherwise, return true
		true
	}
}
