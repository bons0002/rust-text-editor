use super::{
	editing_keys, navigation_keys, Blocks, ClipboardProvider, EditorSpace, IndexedParallelIterator,
	IntoParallelIterator, ParallelIterator, UnicodeSegmentation,
};

// Copy a selection of text to the clipboard
pub fn copy_to_clipboard(editor: &mut EditorSpace) {
	// Start of the highlighted selection
	let start = (editor.selection.start[0], editor.selection.start[1]);
	// End of the highlighted selection
	let end = (editor.selection.end[0], editor.selection.end[1]);
	// Create a copy of the text blocks
	let mut blocks = editor.blocks.as_ref().unwrap().clone();

	// Copy the lines of text in the selection into a vector
	let lines = match copy_subroutines::copy_lines(editor, start, end, &mut blocks) {
		Ok(lines) => lines,
		Err(err) => panic!(
			"{}:{}::copy_to_clipboard | Couldn't copy selection | {}",
			file!(),
			line!(),
			err
		),
	};

	// Write to the clipboard
	editor
		.clipboard
		.as_mut()
		.unwrap()
		.set_contents(lines.into_par_iter().collect::<String>())
		.unwrap();
}

// Paste text from the clipboard
pub fn paste_from_clipboard(editor: &mut EditorSpace) {
	// Delete selection to paste over
	if !editor.selection.is_empty {
		editor.delete_selection();
	}
	// The text content of the clipboard
	let text = paste_subroutines::get_clipboard_content(editor);
	// Split the current line of text about the cursor
	let (mut before_cursor, after_cursor) = paste_subroutines::split_line(editor);

	// For a multiline selection to be pasted
	if text.len() > 1 {
		paste_subroutines::multiline_paste(editor, &mut before_cursor, after_cursor, text);
	// Single line clipboard content
	} else {
		paste_subroutines::singleline_paste(editor, &mut before_cursor, after_cursor, text);
	}
}

// Call the copy function and delete the selection
pub fn cut(editor: &mut EditorSpace) {
	// Get the current editor state
	let state = editor.get_unredo_state();
	// Add a new undo state
	editor.unredo_stack.auto_update(state, true);

	// Copy the selection to the clipboard
	copy_to_clipboard(editor);
	// Delete the selection
	editor.delete_selection();
}

/*
=================================================
			Copy Function Subroutines
=================================================
*/

// Subroutines for copying to the clipboard
mod copy_subroutines {
	use super::{
		Blocks, EditorSpace, IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
		UnicodeSegmentation,
	};
	use std::io::Error;

	pub fn copy_lines(
		editor: &mut EditorSpace,
		start: (usize, usize),
		end: (usize, usize),
		blocks: &mut Blocks,
	) -> Result<Vec<String>, Error> {
		// Get the lines of text
		let mut lines = Vec::new();
		// Iterate through the lines of the selection
		for line_num in start.1..end.1 + 1 {
			// Ensure the blocks are valid
			if line_num % editor.height == 0 {
				blocks.check_blocks(editor);
			}
			// Get the indices of the graphemes
			let indices = &blocks
				.get_some_line(line_num)?
				.graphemes(true)
				.map(String::from)
				.collect::<Vec<String>>();

			// Get the line of the selection
			let line = match copy_line(start, end, line_num, blocks, indices) {
				Ok(line) => line,
				Err(err) => panic!(
					"{}:{}::copy_lines | Couldn't copy line {} | {}",
					file!(),
					line!(),
					line_num,
					err
				),
			};

			// Add a newline on all but the last line
			if line_num != end.1 {
				lines.push(line + "\n");
			} else {
				lines.push(line);
			}
		}

		Ok(lines)
	}

	// Retrieve a line from the selection to add to the list of lines to be copies
	fn copy_line(
		start: (usize, usize),
		end: (usize, usize),
		line_num: usize,
		blocks: &mut Blocks,
		indices: &Vec<String>,
	) -> Result<String, Error> {
		// If only one line
		if start.1 == end.1 {
			Ok(one_line_selection(indices, start.0, end.0))
		// If first line
		} else if line_num == start.1 {
			Ok(first_line_selection(indices, start.0))
		// If last line
		} else if line_num == end.1 {
			Ok(last_line_selection(indices, end.0))
		// If middle line
		} else {
			Ok(String::from(&blocks.get_some_line(line_num)?))
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
}

/*
==================================================
			Paste Function Subroutines
==================================================
*/

// Subroutines for pasting from clipboard
mod paste_subroutines {
	use super::{
		super::super::key_functions, editing_keys, navigation_keys, ClipboardProvider, EditorSpace,
		UnicodeSegmentation,
	};

	// Get the text on the line before and after the cursor
	pub fn split_line(editor: &mut EditorSpace) -> (String, String) {
		// The current line of text
		let line = editor.blocks.as_ref().unwrap().get_current_line();
		// The current line of text before the text position
		let before_cursor = String::from(&line[..editor.text_position]);
		// The current line of text after the text position
		let after_cursor = String::from(&line[editor.text_position..]);

		(before_cursor, after_cursor)
	}

	// Get the text content of the clipboard (and the length of the text)
	pub fn get_clipboard_content(editor: &mut EditorSpace) -> Vec<String> {
		// Get the text stored in the clipboard
		let text = editor.clipboard.as_mut().unwrap().get_contents().unwrap();

		// The text from the clipboard as a text vector
		text.split('\n').map(String::from).collect::<Vec<String>>()
	}

	// Paste a single line from the clipboard
	pub fn singleline_paste(
		editor: &mut EditorSpace,
		before_cursor: &mut String,
		after_cursor: String,
		text: Vec<String>,
	) {
		// Append the clipboard content to the text before the cursor
		*before_cursor += text[0].as_str();
		// Create the full line of text (after paste)
		let text = before_cursor.to_owned() + &after_cursor;

		// Store the original editor width
		let original_width = editor.width;
		// Make the editor width HUGE
		editor.width = usize::MAX;

		// Update the current line with this new text
		editor.blocks.as_mut().unwrap().update_current_line(text);

		// Reset editor width
		editor.width = original_width;

		// Move cursor to new location
		realign_cursor(editor, before_cursor.to_string());
	}

	// Paste multiple lines from the clipboard
	pub fn multiline_paste(
		editor: &mut EditorSpace,
		before_cursor: &mut String,
		after_cursor: String,
		text: Vec<String>,
	) {
		// Append the first line of the clipboard to the text before the cursor
		*before_cursor += text[0].as_str();
		// Prepend the last line of the clipboard to the text after the cursor
		let after_cursor = text.last().unwrap().to_owned() + &after_cursor;

		// Store the original editor width
		let original_width = editor.width;
		// Make the editor width HUGE
		editor.width = usize::MAX;

		// Update the current line of text with the part before the cursor
		editor
			.blocks
			.as_mut()
			.unwrap()
			.update_current_line(before_cursor.to_string());

		// Move cursor to new location
		realign_cursor(editor, before_cursor.to_string());

		// Paste the multiline clipboard content
		paste_loop(editor, text, after_cursor);

		// Reset editor width
		editor.width = original_width;
	}

	// Move to the new location after updating the first line of the mutliline paste
	fn realign_cursor(editor: &mut EditorSpace, before_cursor: String) {
		// Length of the text before the cursor
		let before_len = before_cursor.graphemes(true).count();

		// Move to the beginning of the line
		navigation_keys::home_key(editor, true);
		// Move to the new location on the line after the update
		while editor.text_position < before_len && key_functions::check_cursor_end_line(editor) {
			navigation_keys::right_arrow(editor, true);
		}
	}

	// Loop through the multiline clipboard content and paste to the editor
	fn paste_loop(editor: &mut EditorSpace, text: Vec<String>, after_cursor: String) {
		// Loop through the lines of the clipboard content
		for (idx, line) in text.clone().into_iter().enumerate() {
			// Skip the first line (already done)
			if idx == 0 {
				continue;
			// For the last line of the clipboard
			} else if idx == text.len() - 1 {
				last_line_paste(editor, &text, &after_cursor);
			// All lines in the middle
			} else {
				// Add a new line
				editing_keys::enter_key(editor);
				// Update this new line
				editor.blocks.as_mut().unwrap().update_current_line(line);
				// Move to the end of this new line
				navigation_keys::end_key(editor, true);
			}
		}
	}

	// Paste the last line of the clipboard content
	fn last_line_paste(editor: &mut EditorSpace, text: &[String], after_cursor: &str) {
		// Length of the text after the cursor
		let after_len = text.last().unwrap().graphemes(true).count();

		// Add a newline
		editing_keys::enter_key(editor);
		// Update the text of the newline
		editor
			.blocks
			.as_mut()
			.unwrap()
			.update_current_line(after_cursor.to_string());
		// Move to the beginning of the line
		navigation_keys::home_key(editor, true);
		// Move to the new location after updating the line
		while editor.text_position < after_len && key_functions::check_cursor_end_line(editor) {
			navigation_keys::right_arrow(editor, true);
		}
	}
}
