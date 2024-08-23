// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`
// Contains the logic for all the keys pressed

use super::blocks::Blocks;
use super::EditorSpace;
use rayon::iter::ParallelExtend;
use std::{
	fs::{File, OpenOptions},
	io::Write,
};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

// Contains logic for all highlighting keys
pub mod highlight_selection;

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Insert the character into the correct line in the correct block
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, code)
		.unwrap_or_else(|err| panic!("Couldn't insert char on line {} | {}", line_num, err));

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += 1;
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Insert tab character into the line
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_char_in_line(line_num, editor.text_position, '\t')
		.unwrap_or_else(|err| panic!("Couldn't insert char on line {} | {}", line_num, err));

	// Move cursor
	editor.text_position += 1;
	editor.cursor_position[0] += editor.config.tab_width;
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
	// If there is a highlighted selection
	if !editor.selection.is_empty {
		// Delete the selection
		editor.delete_selection();
	}

	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Insert a new line and truncate the current one (after the cursor)
	editor
		.blocks
		.as_mut()
		.unwrap()
		.insert_new_line(line_num, editor.text_position)
		.unwrap_or_else(|err| panic!("Couldn't insert new line {} | {}", line_num, err));

	// Add a line to the overall file length
	editor.file_length += 1;

	// Reset cursor to beginning of line
	down_arrow(editor);
	home_key(editor);
}

// Backspace at the beginning of line, moving to the above line
fn backspace_beginning_of_line(editor: &mut EditorSpace) {
	if editor.file_length > 1 {
		// Move up one line
		up_arrow(editor);
		end_key(editor);
		// Line number of current line in the text
		let line_num = editor.get_line_num(editor.cursor_position[1]);

		// Delete the previous line and append its text content to the current line
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_and_append_line(line_num)
			.unwrap_or_else(|err| panic!("Couldn't delete line {} | {}", line_num + 1, err));

		// Reduce the file length
		editor.file_length -= 1;
	}
}

// Backspace after the beginning of the line deletes a char normally
fn backspace_normally(editor: &mut EditorSpace) {
	// Move left
	left_arrow(editor);
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Remove one character
	editor
		.blocks
		.as_mut()
		.unwrap()
		.delete_char_in_line(line_num, editor.text_position)
		.unwrap_or_else(|err| panic!("Couldn't delete char on line {} | {}", line_num, err));
}

// Functionality of the backspace key
pub fn backspace(editor: &mut EditorSpace) {
	// If there is no highlighted selection, backspace normally
	if editor.selection.is_empty {
		// The current line number
		let line_num = editor.get_line_num(editor.cursor_position[1]);
		// Remove empty line
		// If cursor at beginning of line, move to above line
		if editor.text_position == 0 && line_num != 0 {
			// Backspace at beginning of the line
			backspace_beginning_of_line(editor);
		// Otherwise, just move cursor left
		} else if editor.text_position != 0 {
			// Backspace normally, deleting one char
			backspace_normally(editor);
		}
	} else {
		// Delete the selection
		editor.delete_selection();
	}
}

// Delete a character normally if there is no selection
fn no_selection_delete(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// The length of the current line
	let length = match editor.blocks.as_ref().unwrap().get_line_length(line_num) {
		Ok(len) => len,
		Err(err) => panic!("Couldn't get length of line {} | {}", line_num, err),
	};

	// If not at the end of the current line
	if editor.text_position < length {
		// Delete next char
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_char_in_line(line_num, editor.text_position)
			.unwrap_or_else(|err| panic!("Couldn't delete char on line {} | {}", line_num, err));
	// If not at end of last line
	} else if line_num < editor.file_length - 1 {
		// Delete the below line and append its text content to the current line
		editor
			.blocks
			.as_mut()
			.unwrap()
			.delete_and_append_line(line_num)
			.unwrap_or_else(|err| panic!("Couldn't delete line {} | {}", line_num + 1, err));
		// Reduce the overall file length
		editor.file_length -= 1;
	}
}

// Functionality of the delete key
pub fn delete_key(editor: &mut EditorSpace) {
	// If there is no highlighted selection, delete normally
	if editor.selection.is_empty {
		// Delete character
		no_selection_delete(editor);
	} else {
		// Delete the selection
		editor.delete_selection();
	}
}

// Logic for moving left for a non-tab char
fn left_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str) {
	// Create a cursor to navigate the grapheme cluster
	let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
	// Get the previous location in the text
	let loc = cursor.prev_boundary(line, 0);
	// Set the text position
	let loc = match loc {
		Ok(num) => match num {
			Some(num) => num,
			None => panic!(
				/* Return the source file name, line number error occurred in this source file,
				and line_num that this grapheme boundary exists on. */
				"{}::left_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
				file!(),
				line!(),
				line_num
			),
		},
		Err(_) => 0,
	};
	// Get the difference in the positions
	let diff = editor.text_position - loc;
	// Update editor text position
	editor.text_position -= diff;
	// Move the screen cursor
	match diff > 1 {
		// If there is a non ascii character there, the screen cursor needs to move two spaces
		true => editor.cursor_position[0] -= 2,
		// Otherwise, move one space
		false => editor.cursor_position[0] -= 1,
	}
}

// Logic for moving left if not at the beginning of the line
fn left_not_beginning_of_line(editor: &mut EditorSpace, line_num: usize) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If the next char isn't a tab, move normally
	if line.graphemes(true).nth(editor.text_position - 1) != Some("\t") {
		left_not_tab(editor, line_num, &line);
	// Otherwise, move by the number of tab spaces
	} else {
		editor.text_position -= 1;
		editor.cursor_position[0] -= editor.config.tab_width;
	}
}

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
	// If the x position is before the start of the line, return false
	if editor.text_position == 0 {
		return false;
	}
	true
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't move before the beginning of the line
	if check_cursor_begin_line(editor) {
		// Move left if not at the beginning of the line (move normally)
		left_not_beginning_of_line(editor, line_num);
	} else {
		// Move to above line
		if line_num > 0 {
			up_arrow(editor);
			end_key(editor);
		} else {
			home_key(editor);
		}
	}
}

// Check the end of line cursor condition
fn check_cursor_end_line(editor: &mut EditorSpace, line_num: usize) -> bool {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If the x position is beyond the end of the line, return false
	if editor.text_position >= line.len()
		|| editor.text_position >= editor.width.1 - editor.width.0 - 2
	{
		return false;
	}
	true
}

// Logic for moving right in the text for a non-tab char
fn right_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str) {
	// Create a cursor to navigate the grapheme cluster
	let mut cursor = GraphemeCursor::new(editor.text_position, line.len(), true);
	// Get the next location in the text
	let loc = cursor.next_boundary(line, 0);
	// Set the text position
	let loc = match loc {
		Ok(num) => match num {
			Some(num) => num,
			None => panic!(
				/* Return the source file name, line number error occurred in this source file,
				and line_num that this grapheme boundary exists on. */
				"{}::right_arrow: line {}. Invalid grapheme boundary for `line_num = {}`",
				file!(),
				line!(),
				line_num
			),
		},
		Err(_) => line.len(),
	};
	// Get the difference in the positions
	let diff = loc - editor.text_position;
	// Update editor text position
	editor.text_position += diff;
	// Move the screen cursor
	match diff > 1 {
		// If there is a non ascii character there, the screen cursor needs to move two spaces
		true => editor.cursor_position[0] += 2,
		// Otherwise, move one space
		false => editor.cursor_position[0] += 1,
	}
}

// Logic for moving right in the text before reaching the end of the line
fn right_not_end_of_line(editor: &mut EditorSpace, line_num: usize) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If not a tab character, move normally
	if line.graphemes(true).nth(editor.text_position) != Some("\t") {
		// Move right for non-tab chars
		right_not_tab(editor, line_num, &line);
	// Otherwise, move the number of tab spaces
	} else {
		editor.text_position += 1;
		editor.cursor_position[0] += editor.config.tab_width;
	}
}

/* If the last line of the file is an empty line, using
file_length = editor.file_length - 1 will cause the cursor to
stop at the second to last line, so file_length = editor.file_length
must be used. */
fn get_correct_file_length(editor: &mut EditorSpace) -> usize {
	// Last line that the cursor can move to
	let mut file_length = editor.file_length - 1;
	// Check if last line is empty
	if editor.blocks.as_ref().unwrap().blocks_list
		[editor.blocks.as_ref().unwrap().blocks_list.len() - 1]
		.content
		.last()
		.unwrap()
		.clone() == *""
	{
		file_length = editor.file_length;
	}
	// Return the file length
	file_length
}

// Logic for moving right in the text when at the end of the line (move to next line)
fn right_end_of_line(editor: &mut EditorSpace, line_num: usize) {
	// Last line that the cursor can move to
	let file_length = get_correct_file_length(editor);

	// Move to next line
	if line_num < file_length {
		down_arrow(editor);
		home_key(editor);
	} else {
		end_key(editor);
	}
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// Move right normally
		right_not_end_of_line(editor, line_num);
	// If the cursor goes beyond the end of the line
	} else {
		// Move right at the end of the line (move to next line)
		right_end_of_line(editor, line_num);
	}
}

// Logic for moving up without scrolling
fn up_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the prev line
	editor.cursor_position[1] -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.cursor_position[0];
	// Move cursor to beginning of line
	home_key(editor);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor);
	}
}

// Logic for moving up while scrolling
fn up_with_scroll(editor: &mut EditorSpace) {
	// Scroll up
	editor.scroll_offset -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.cursor_position[0];
	// Move cursor to beginning of line
	home_key(editor);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor);
	}
}

// Logic for loading new blocks while moving up
fn up_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the head
	blocks.as_mut().unwrap().push_head(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;

	// Update scroll offset
	editor.scroll_offset += editor.blocks.as_ref().unwrap().get_head().len - 1;
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace) {
	// The current line number
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Ensure that the cursor doesn't move above the editor block
	if editor.cursor_position[1] > 0 {
		// Move up without scrolling
		up_no_scroll(editor);
	// If the cursor moves beyond the bound
	} else if editor.scroll_offset > 0 {
		// Move up and scroll
		up_with_scroll(editor);
	// If moving before the start of the block, insert a new head
	} else if line_num < editor.blocks.as_ref().unwrap().starting_line_num + 1 && line_num > 0 {
		// Move up and load blocks
		up_load_blocks(editor);
	}
}

// Logic for moving down without scrolling
fn down_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the next line
	editor.cursor_position[1] += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.cursor_position[0];
	// Move cursor to beginning of line
	home_key(editor);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor);
	}
}

// Logic for loading blocks when moving down
fn down_load_blocks(editor: &mut EditorSpace) {
	// Clone the blocks
	let mut blocks = editor.blocks.clone();
	// Insert a new block at the tail (and remove head if necessary)
	blocks.as_mut().unwrap().push_tail(editor, true).unwrap();
	// Set this blocks to the editor
	editor.blocks = blocks;
}

// Logic for moving down while scrolling
fn down_with_scroll(editor: &mut EditorSpace) {
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
	// Save current position
	let position = editor.cursor_position[0];
	// Move cursor to beginning of line
	home_key(editor);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor);
	}
}

// The main control flow of the down arrow key
fn down_conditions(editor: &mut EditorSpace) {
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];
	// Ensure that the cursor doesn't move below the editor block
	if cursor_line_num < (editor.height.1 - editor.height.0) - 3 {
		// Move down without scrolling
		down_no_scroll(editor);
	// If the cursor goes below the bound
	} else {
		// Move down and scroll
		down_with_scroll(editor);
	}
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Last line that the cursor can move to
	let file_length = get_correct_file_length(editor);

	// Ensure that the cursor doesn't move beyond the end of the file
	if line_num < file_length {
		// Following proper control flow for moving down
		down_conditions(editor);
	}
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace) {
	// Move to beginning of line
	editor.text_position = 0;
	editor.cursor_position[0] = 0;
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	while check_cursor_end_line(editor, line_num) {
		right_arrow(editor);
	}
}

fn load_blocks(editor: &mut EditorSpace) -> Blocks {
	// Clone the editor blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	// The block number of the head and tail blocks respectively
	let (head_block, tail_block) = (blocks.head_block, blocks.tail_block);

	// Load in all blocks in the file that aren't currently in the Blocks
	for i in 0..blocks.max_blocks {
		if i >= head_block && i <= tail_block {
			continue;
		} else if i < head_block {
			match blocks.push_head(editor, false) {
				Ok(_) => (),
				Err(err) => {
					panic!("{}", err);
				}
			}
		} else if i > tail_block {
			match blocks.push_tail(editor, false) {
				Ok(_) => (),
				Err(err) => {
					panic!("{}", err);
				}
			}
		}
	}

	// Return the blocks
	blocks
}

// Recreate an existing file (for saving)
fn recreate_file(filename: &str) -> File {
	// Create a new blank version of the file
	File::create(filename).unwrap();
	// Open the file in read-write mode
	let file = match OpenOptions::new().read(true).write(true).open(filename) {
		Ok(file) => file,
		Err(err) => panic!("{}", err),
	};
	file
}

// Save the contents of the contents vector to the given file
fn save_file(filename: &str, contents: Vec<String>) -> File {
	// Open the file in read-write mode
	let mut file = recreate_file(filename);
	// Get the number of lines
	let len = contents.len();

	// Write lines to the file
	for (idx, line) in contents.iter().enumerate() {
		// If not last line, add a newline char
		if idx < len - 1 {
			match writeln!(&file, "{}", line) {
				Ok(_) => (),
				Err(err) => panic!("{}", err),
			}
		// If last line, don't add newline char
		} else {
			match write!(&file, "{}", line) {
				Ok(_) => (),
				Err(err) => panic!("{}", err),
			}
		}
	}
	// Flush the file buffer
	file.flush().unwrap();
	// Return the file
	file
}

// Update the editor's scroll offset and blocks after saving
fn post_save_editor_update(editor: &mut EditorSpace, blocks: &mut Blocks) {
	// Get the line number of the first line of the widget
	let line_num = editor.get_line_num(0);
	// Might need to add a new head block
	if line_num < blocks.starting_line_num {
		blocks.push_head(editor, false).unwrap();
	}
	// Reset scroll offset
	editor.scroll_offset = line_num - blocks.starting_line_num;
	// Set the editor blocks to this new Blocks
	editor.blocks = Some(blocks.clone());
}

// Save key combo functionality
pub fn save_key_combo(editor: &mut EditorSpace, in_debug_mode: bool, debug_filename: &str) {
	// Load in all the blocks
	let blocks = load_blocks(editor);
	// Get all the lines of the Blocks in one vector
	let mut contents: Vec<String> = Vec::new();
	for block in blocks.clone().blocks_list {
		contents.par_extend(block.content)
	}

	/* Write to different files based on if this function is in
	debug mode. */
	match in_debug_mode {
		// If in debug mode, write to debug_filename
		true => _ = save_file(debug_filename, contents),
		// If not in debug mode, write to the regular file
		false => editor.file = save_file(&editor.filename, contents),
	}

	// Get the block number and line number of the current location
	let (block_num, _) = match blocks.get_location(editor.get_line_num(editor.cursor_position[1])) {
		Ok((block, line)) => (block, line),
		Err(err) => panic!("{}::save_key_combo: line = {} | {}", file!(), line!(), err),
	};
	// Construct a new Blocks for the newly saved file
	let mut blocks = match Blocks::new(editor, block_num) {
		Ok(block) => block,
		Err(err) => panic!(
			"{}::save_key_combo: line = {}. Couldn't initialize Blocks for block_num = {} | {}",
			file!(),
			line!(),
			block_num,
			err
		),
	};
	// Update the editor's scroll offset and Blocks
	post_save_editor_update(editor, &mut blocks);
}
