use super::{
	navigation_keys::right_arrow, Blocks, ClipboardProvider, EditorSpace, IndexedParallelIterator,
	IntoParallelIterator, ParallelIterator, UnicodeSegmentation,
};

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
		if line_num % editor.height == 0 {
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
