// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`
// Contains the logic for all the keys pressed

use super::blocks::Blocks;
use super::EditorSpace;
use cli_clipboard::ClipboardProvider;
use rayon::iter::{
	IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator,
};
use std::{
	fs::{File, OpenOptions},
	io::Write,
};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

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
	editor.stored_position = editor.cursor_position[0];
	editor.index_position += 1;
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
	editor.stored_position = editor.cursor_position[0];
	editor.index_position += 1;
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
	home_key(editor, true);
}

// Backspace at the beginning of line, moving to the above line
fn backspace_beginning_of_line(editor: &mut EditorSpace) {
	if editor.file_length > 0 {
		// Move up one line
		up_arrow(editor);
		end_key(editor, true);
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
	left_arrow(editor, true);
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// Remove one character
	editor
		.blocks
		.as_mut()
		.unwrap()
		.delete_char_in_line(line_num, editor.index_position)
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
			.delete_char_in_line(line_num, editor.index_position)
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
fn left_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str, will_store_cursor: bool) {
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

	// Get the previous grapheme
	let character = line
		.graphemes(true)
		.nth(editor.index_position - 1)
		.unwrap_or("");
	// Get the width of the current grapheme
	let char_width = UnicodeWidthStr::width(character);
	// Move the screen cursor
	editor.cursor_position[0] -= char_width;
	// Store the cursor position if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}

	// Get the difference in the positions
	let diff = editor.text_position - loc;
	// Update editor text position
	editor.text_position -= diff;
}

// Logic for moving left if not at the beginning of the line
fn left_not_beginning_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If the previous char isn't a tab, move normally
	if line.graphemes(true).nth(editor.index_position - 1) != Some("\t") {
		left_not_tab(editor, line_num, &line, will_store_cursor);
	// Otherwise, move by the number of tab spaces
	} else {
		editor.text_position -= 1;
		editor.cursor_position[0] -= editor.config.tab_width;
		// Store the cursor position if the flag is set
		if will_store_cursor {
			editor.stored_position = editor.cursor_position[0];
		}
	}
	// Update the index position
	editor.index_position -= 1;
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
pub fn left_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't move before the beginning of the line
	if check_cursor_begin_line(editor) {
		// Move left if not at the beginning of the line (move normally)
		left_not_beginning_of_line(editor, line_num, will_store_cursor);
	} else {
		// Move to above line
		if line_num > 0 {
			up_arrow(editor);
			end_key(editor, will_store_cursor);
		} else {
			home_key(editor, will_store_cursor);
		}
	}
}

// Jump one unicode word to the left
pub fn jump_left(editor: &mut EditorSpace, will_highlight: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Line number of current line in the text
	let line = editor.blocks.as_ref().unwrap().get_line(line_num).unwrap();

	// Get the index of the previous word
	let index = line
		.unicode_word_indices()
		.filter(|(idx, _)| *idx < editor.text_position)
		.last()
		.unwrap_or((0, ""))
		.0;

	// Move to the beginning of the previous word
	while editor.text_position > index {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_left(editor);
		// If set to not highlight
		} else {
			left_arrow(editor, true);
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
fn right_not_tab(editor: &mut EditorSpace, line_num: usize, line: &str, will_store_cursor: bool) {
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

	// Get the current grapheme
	let character = line
		.graphemes(true)
		.nth(editor.index_position)
		.unwrap_or("");
	// Get the width of the current grapheme
	let char_width = UnicodeWidthStr::width(character);
	// Move the screen cursor
	editor.cursor_position[0] += char_width;
	// Set the stored cursor position if the flag is set
	if will_store_cursor {
		editor.stored_position = editor.cursor_position[0];
	}

	// Get the difference in the positions
	let diff = loc - editor.text_position;
	// Update editor text position
	editor.text_position += diff;
}

// Logic for moving right in the text before reaching the end of the line
fn right_not_end_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	// If not a tab character, move normally
	if line.graphemes(true).nth(editor.index_position) != Some("\t") {
		// Move right for non-tab chars
		right_not_tab(editor, line_num, &line, will_store_cursor);
	// Otherwise, move the number of tab spaces
	} else {
		editor.text_position += 1; // tabs are width 1
		editor.cursor_position[0] += editor.config.tab_width;
		// Store the cursor if the flag is set
		if will_store_cursor {
			editor.stored_position = editor.cursor_position[0];
		}
	}
	// Update the index position
	editor.index_position += 1;
}

// Logic for moving right in the text when at the end of the line (move to next line)
fn right_end_of_line(editor: &mut EditorSpace, line_num: usize, will_store_cursor: bool) {
	// Last line that the cursor can move to
	let file_length = editor.file_length - 1;

	// Move to next line
	if line_num < file_length {
		down_arrow(editor);
		home_key(editor, will_store_cursor);
	} else {
		end_key(editor, will_store_cursor);
	}
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);

	// If the cursor doesn't go beyond the end of the line
	if check_cursor_end_line(editor, line_num) {
		// Move right normally
		right_not_end_of_line(editor, line_num, will_store_cursor);
	// If the cursor goes beyond the end of the line
	} else {
		// Move right at the end of the line (move to next line)
		right_end_of_line(editor, line_num, will_store_cursor);
	}
}

// Jump one unicode word to the right
pub fn jump_right(editor: &mut EditorSpace, will_highlight: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Line number of current line in the text
	let line = editor.blocks.as_ref().unwrap().get_line(line_num).unwrap();

	// Get the index of the next word
	let index = line
		.unicode_word_indices()
		.find(|(idx, _)| *idx > editor.text_position)
		.unwrap_or((line.len(), ""))
		.0;

	// Move to the beginning of the next word
	while editor.text_position < index {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_right(editor);
		// If set to not highlight
		} else {
			right_arrow(editor, true);
		}
	}
}

// Logic for moving up without scrolling
fn up_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the prev line
	editor.cursor_position[1] -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// Logic for moving up while scrolling
fn up_with_scroll(editor: &mut EditorSpace) {
	// Scroll up
	editor.scroll_offset -= 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
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

// Move the cursor up 10 lines
pub fn jump_up(editor: &mut EditorSpace, will_highlight: bool) {
	// Move up 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_up(editor);
		// If set to not highlight
		} else {
			up_arrow(editor);
		}
	}
}

// Logic for moving down without scrolling
fn down_no_scroll(editor: &mut EditorSpace) {
	// Move the cursor to the next line
	editor.cursor_position[1] += 1;
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Save current position
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
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
	let position = editor.stored_position;
	// Move cursor to beginning of line
	home_key(editor, false);
	// Loop until in correct position
	while editor.cursor_position[0] < position && check_cursor_end_line(editor, line_num) {
		// Move right
		right_arrow(editor, false);
	}
}

// The main control flow of the down arrow key
fn down_conditions(editor: &mut EditorSpace) {
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];
	// Ensure that the cursor doesn't move below the editor block (sub 3 because 2 lines of borders)
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
	let file_length = editor.file_length - 1;

	// Ensure that the cursor doesn't move beyond the end of the file
	if line_num < file_length {
		// Following proper control flow for moving down
		down_conditions(editor);
	}
}

// Move the cursor down 10 lines
pub fn jump_down(editor: &mut EditorSpace, will_highlight: bool) {
	// Move down 10 lines
	for _i in 0..10 {
		// If set to highlight
		if will_highlight {
			highlight_selection::highlight_down(editor);
		// If set to not highlight
		} else {
			down_arrow(editor);
		}
	}
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Move to beginning of line
	editor.text_position = 0;
	editor.cursor_position[0] = 0;
	editor.index_position = 0;
	// Set the stored cursor to the beginning of the line if the flag is set
	if will_store_cursor {
		editor.stored_position = 0;
	}
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace, will_store_cursor: bool) {
	// Line number of current line in the text
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	while check_cursor_end_line(editor, line_num) {
		right_arrow(editor, will_store_cursor);
	}
}

// Move up one page
pub fn page_up(editor: &mut EditorSpace) {
	/* Move up one page.
	Subtract 2 from the upper bound of the height because there are 2 border lines. */
	for _i in editor.height.0..(editor.height.1 - 2) {
		up_arrow(editor);
	}
}

// Move down one page
pub fn page_down(editor: &mut EditorSpace) {
	/* Move down one page.
	Subtract 2 from the upper bound of the height because there are 2 border lines. */
	for _i in editor.height.0..(editor.height.1 - 2) {
		down_arrow(editor);
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

// Get the text content of the clipboard (and the length of the text)
fn get_clipboard_content(editor: &mut EditorSpace) -> (Vec<String>, usize) {
	// Get the text stored in the clipboard
	let text = editor.clipboard.as_mut().unwrap().get_contents().unwrap();

	(
		// The text from the clipboard as a text vector
		text.split('\n').map(String::from).collect::<Vec<String>>(),
		// The length of the text in the clipboard
		text.graphemes(true).count(),
	)
}

// Get the text on the line before and after the cursor
fn split_line(editor: &mut EditorSpace, line_num: usize) -> (String, String) {
	// The current line of text
	let line = editor.blocks.as_ref().unwrap().get_line(line_num).unwrap();
	// The current line of text before the text position
	let before_cursor = String::from(&line[..editor.text_position]);
	// The current line of text after the text position
	let after_cursor = String::from(&line[editor.text_position..]);

	(before_cursor, after_cursor)
}

// Paste text from the clipboard
pub fn paste_from_clipboard(editor: &mut EditorSpace) {
	// Delete a selection to paste over
	if !editor.selection.is_empty {
		editor.delete_selection();
	}
	// The text content of the clipboard (and the length of the text)
	let (text, text_length) = get_clipboard_content(editor);
	// The line number to start pasting to
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Get the text on the current line before and after the cursor
	let (before_cursor, after_cursor) = split_line(editor, line_num);
	// The number of lines in the clipboard text vector
	let num_lines = text.len();

	// Loop through the lines of the clipboard content
	for (idx, mut line) in text.into_iter().enumerate() {
		// First line
		if idx == 0 {
			// Concat the current line before the cursor with the first line of the clipboard content
			let mut new_line = before_cursor.clone() + &line;
			// If only one line in the clipboard, append the after_cursor string
			if num_lines == 1 {
				new_line.push_str(&after_cursor);
			}
			// Update the line in the Blocks with this new line
			editor
				.blocks
				.as_mut()
				.unwrap()
				.update_line(new_line, line_num)
				.unwrap();
		// Rest of the lines
		} else {
			// Append after_cursor to the line if at the last line
			if idx == num_lines - 1 {
				line.push_str(&after_cursor);
			}
			// Add a new line of text to the Blocks
			editor
				.blocks
				.as_mut()
				.unwrap()
				.insert_full_line(line, line_num + idx)
				.unwrap();
			// Update the file length
			editor.file_length += 1;
		}
	}

	// Move to the end of the paste
	for _i in 0..text_length {
		right_arrow(editor, true);
	}
}

// Collect the graphemes of a one line selection into a string
fn one_line_selection(indices: &Vec<String>, start: usize, end: usize) -> String {
	indices
		.into_par_iter()
		.enumerate()
		.filter_map(|(idx, graph)| {
			// Get all graphemes on the line between the two indices
			if idx >= start && idx < end {
				Some(String::from(graph))
			} else {
				None
			}
		})
		.collect::<String>()
}

// Collect the graphemes of the first line of a multiline selection into a string
fn first_line_selection(indices: &Vec<String>, start: usize) -> String {
	indices
		.into_par_iter()
		.enumerate()
		.filter_map(|(idx, graph)| {
			// Get all graphemes on the line after the index
			if idx >= start {
				Some(String::from(graph))
			} else {
				None
			}
		})
		.collect::<String>()
}

// Collect the graphemes of the last line of a multiline seelction into a string
fn last_line_selection(indices: &Vec<String>, end: usize) -> String {
	indices
		.into_par_iter()
		.enumerate()
		.filter_map(|(idx, graph)| {
			// Get all graphemes on the line before the index
			if idx < end {
				Some(String::from(graph))
			} else {
				None
			}
		})
		.collect::<String>()
}

fn copy_loop(
	editor: &mut EditorSpace,
	start: (usize, usize),
	end: (usize, usize),
	blocks: &mut Blocks,
) -> Vec<String> {
	// Get the lines of text
	let mut lines = Vec::new();
	// Iterate through the lines of the selection
	for line_num in start.1..end.1 + 1 {
		let line;
		// Ensure the blocks are valid
		if line_num % (editor.height.1 - editor.height.0) == 0 {
			blocks.check_blocks(editor);
		}
		// Get the indices of the graphemes
		let indices = &blocks
			.get_line(line_num)
			.unwrap()
			.graphemes(true)
			.map(String::from)
			.collect::<Vec<String>>();
		// If only one line
		if start.1 == end.1 {
			line = one_line_selection(indices, start.0, end.0);
		// If first line
		} else if line_num == start.1 {
			line = first_line_selection(indices, start.0);
		// If last line
		} else if line_num == end.1 {
			line = last_line_selection(indices, end.0);
		// If middle line
		} else {
			line = String::from(&blocks.get_line(line_num).unwrap())
		}
		// Add a newline on all but the last line
		if line_num != end.1 {
			lines.push(line + "\n");
		} else {
			lines.push(line);
		}
	}

	lines
}

// Copy a selection of text to the clipboard
pub fn copy_to_clipboard(editor: &mut EditorSpace) {
	// Start of the highlighted selection
	let start = (editor.selection.start[0], editor.selection.start[1]);
	// End of the highlighted selection
	let end = (editor.selection.end[0], editor.selection.end[1]);
	// Create a copy of the text blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();

	// Copy the lines of text in the selection into a vector
	let lines = copy_loop(editor, start, end, &mut blocks);

	// Write to the clipboard
	editor
		.clipboard
		.as_mut()
		.unwrap()
		.set_contents(lines.into_par_iter().collect::<String>())
		.unwrap();
}
