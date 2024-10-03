use super::{realign_cursor, EditorSpace};

// Logic for moving down without scrolling
pub fn down_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the next line
	editor.cursor_position[1] += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Realign the cursor with the stored cursor position
	realign_cursor(editor, line_num);
}

// Logic for loading blocks when moving down
pub fn down_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the tail (and remove head if necessary)
	blocks.as_mut().unwrap().push_tail(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;
}

// Logic for moving down while scrolling
pub fn down_with_scroll(editor: &mut EditorSpace) {
	// Scroll down
	editor.scroll_offset += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// If moving after the end of the block, insert a new tail
	if line_num
		>= editor.blocks.as_ref().unwrap().starting_line_num + editor.blocks.as_ref().unwrap().len()
		&& line_num < editor.file_length - 1
	{
		// Move down and load new blocks
		down_load_blocks(editor);
	}
	// Realign the cursor with the stored cursor position
	realign_cursor(editor, line_num);
}

// Update the Blocks location tracker when moving down
pub fn update_block_location(editor: &mut EditorSpace) {
	// Get the current block number
	let block_num = editor.blocks.as_ref().unwrap().curr_position[0];
	// If the current line number is beyond the current block
	if editor.blocks.as_ref().unwrap().curr_position[1]
		>= editor.blocks.as_ref().unwrap().blocks_list[block_num].len
	//&& block_num < editor.blocks.as_ref().unwrap().max_blocks - 1
	{
		// Increment the current block number and reset the current line number
		editor.blocks.as_mut().unwrap().curr_position = [block_num + 1, 0];
	}
}