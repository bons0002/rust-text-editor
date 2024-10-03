use super::{realign_cursor, EditorSpace};

// Logic for moving up without scrolling
pub fn up_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the prev line
	editor.cursor_position[1] -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Realign the cursor with the stored cursor position
	realign_cursor(editor, line_num);
}

// Logic for moving up while scrolling
pub fn up_with_scroll(editor: &mut EditorSpace) {
	// Scroll up
	editor.scroll_offset -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Realign the cursor with the stored cursor position
	realign_cursor(editor, line_num);
}

// Logic for loading new blocks while moving up
pub fn up_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the head
	blocks.as_mut().unwrap().push_head(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;

	// Update scroll offset
	editor.scroll_offset += editor.blocks.as_ref().unwrap().get_head().len - 1;
}

// Update the Blocks location tracker when moving up
pub fn update_block_location(editor: &mut EditorSpace) {
	// Current block number
	let block_num = editor.blocks.as_ref().unwrap().curr_position[0];
	// If not on the first line of the block
	if editor.blocks.as_ref().unwrap().curr_position[1] > 0 {
		// Update the Blocks current location tracker
		editor.blocks.as_mut().unwrap().curr_position[1] -= 1;
	// If not on the first block
	} else if block_num > 0 {
		// Get the length of the previous block
		let block_len = editor.blocks.as_ref().unwrap().blocks_list[block_num - 1].len;
		// Update the Blocks tracker to the end of the previous block
		editor.blocks.as_mut().unwrap().curr_position = [block_num - 1, block_len - 1];
	}
}
