use super::{
	navigation_keys::right_arrow, Blocks, ClipboardProvider, EditorSpace, IndexedParallelIterator,
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
	// Delete a selection to paste over
	if !editor.selection.is_empty {
		editor.delete_selection();
	}
	// The text content of the clipboard (and the length of the text)
	let (text, text_length) = paste_subroutines::get_clipboard_content(editor);
	// Get the current line number
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Get the text on the current line before and after the cursor
	let (before_cursor, after_cursor) = match paste_subroutines::split_line(editor, line_num) {
		Ok(line) => line,
		Err(err) => {
			panic!(
				"{}:{}::paste_from_clipboard | Couldn't split line {} | {}",
				file!(),
				line!(),
				line_num,
				err
			)
		}
	};
	// The number of lines in the clipboard text vector
	let num_lines = text.len();

	// Loop through the lines of the clipboard content
	for (idx, mut line) in text.into_iter().enumerate() {
		paste_subroutines::paste_loop(
			editor,
			&before_cursor,
			&after_cursor,
			&mut line,
			line_num,
			num_lines,
			idx,
		);
	}

	// Move to the end of the paste
	for _i in 0..text_length {
		right_arrow(editor, true);
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
	use super::{ClipboardProvider, EditorSpace, UnicodeSegmentation};
	use std::io::Error;

	// The content of the loop in the paste function
	pub fn paste_loop(
		editor: &mut EditorSpace,
		before_cursor: &str,
		after_cursor: &str,
		line: &mut String,
		line_num: usize,
		num_lines: usize,
		idx: usize,
	) {
		// First line
		if idx == 0 {
			let _ = paste_first_line(
				editor,
				before_cursor,
				after_cursor,
				line,
				line_num,
				num_lines,
			);
		// Rest of the lines
		} else {
			paste_rest(editor, after_cursor, line, line_num, num_lines, idx);
		}
	}

	// Get the text on the line before and after the cursor
	pub fn split_line(
		editor: &mut EditorSpace,
		line_num: usize,
	) -> Result<(String, String), Error> {
		// The current line of text
		let line = editor.blocks.as_ref().unwrap().get_some_line(line_num)?;
		// The current line of text before the text positio, line_numn
		let before_cursor = String::from(&line[..editor.text_position]);
		// The current line of text after the text position
		let after_cursor = String::from(&line[editor.text_position..]);

		Ok((before_cursor, after_cursor))
	}

	// Get the text content of the clipboard (and the length of the text)
	pub fn get_clipboard_content(editor: &mut EditorSpace) -> (Vec<String>, usize) {
		// Get the text stored in the clipboard
		let text = editor.clipboard.as_mut().unwrap().get_contents().unwrap();

		(
			// The text from the clipboard as a text vector
			text.split('\n').map(String::from).collect::<Vec<String>>(),
			// The length of the text in the clipboard
			text.graphemes(true).count(),
		)
	}

	// Paste the first line from the clipboard
	fn paste_first_line(
		editor: &mut EditorSpace,
		before_cursor: &str,
		after_cursor: &str,
		line: &str,
		line_num: usize,
		num_lines: usize,
	) -> Result<(), Error> {
		// Concat the current line before the cursor with the first line of the clipboard content
		let mut new_line = before_cursor.to_owned() + line;
		// If only one line in the clipboard, append the after_cursor string
		if num_lines == 1 {
			new_line.push_str(after_cursor);
		}
		// Update the line in the Blocks with this new line
		editor
			.blocks
			.as_mut()
			.unwrap()
			.update_some_line(new_line, line_num)?;

		Ok(())
	}

	// Paste the rest of the lines after the first one
	fn paste_rest(
		editor: &mut EditorSpace,
		after_cursor: &str,
		line: &mut String,
		line_num: usize,
		num_lines: usize,
		idx: usize,
	) {
		// Append after_cursor to the line if at the last line
		if idx == num_lines - 1 {
			line.push_str(after_cursor);
		}
		// Add a new line of text to the Blocks
		editor
			.blocks
			.as_mut()
			.unwrap()
			.insert_full_line(line.to_string(), line_num + idx)
			.unwrap();
		// Update the file length
		editor.file_length += 1;
	}
}
