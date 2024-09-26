use super::{Blocks, EditorSpace, File, OpenOptions, ParallelExtend};
use std::io::Write;

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
fn post_save_editor_update(editor: &mut EditorSpace) {
	// Get the current line number
	let line_num = editor.get_line_num(editor.cursor_position[1]);
	// Construct a block from that line number
	let mut blocks = Blocks::from_line(editor, line_num).unwrap();

	// Get the line number of the first line of the widget
	let line_num = editor.get_line_num(0);
	// Might need to add a new head block
	while line_num < blocks.starting_line_num {
		blocks.push_head(editor, true).unwrap();
	}
	// Reset scroll offset
	editor.scroll_offset = line_num - blocks.starting_line_num;
	// Set the editor blocks to this new Blocks
	editor.blocks = Some(blocks.clone());
}

// Save key combo functionality
pub fn save_key_combo(editor: &mut EditorSpace, in_debug_mode: bool, debug_filename: &str) {
	// Load in all the blocks in the file
	let mut blocks = editor.blocks.as_ref().unwrap().clone();
	blocks.load_all_blocks(editor);

	// Get all the lines of the Blocks in one vector
	let mut contents: Vec<String> = Vec::new();
	for block in blocks.blocks_list {
		contents.par_extend(block.content)
	}

	// Write to different files based on if this function is in debug mode
	match in_debug_mode {
		// If in debug mode, write to debug_filename
		true => _ = save_file(debug_filename, contents),
		// If not in debug mode, write to the regular file
		false => editor.file = save_file(&editor.filename, contents),
	}

	// Update the editor's scroll offset and Blocks
	post_save_editor_update(editor);
}
