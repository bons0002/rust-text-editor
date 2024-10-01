// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`
// Contains the logic for all the keys pressed

use super::{
	blocks::Blocks, ClipboardProvider, EditorSpace, File, IndexedParallelIterator,
	IntoParallelIterator, OpenOptions, ParallelExtend, ParallelIterator, UnicodeSegmentation,
};

use unicode_segmentation::GraphemeCursor;
use unicode_width::UnicodeWidthStr;

// Contains logic for all highlighting keys
pub mod highlight_keys;
// Contains keys for inserting and deleting
pub mod editing_keys;
// Contains the arrow keys, home, end, etc.
pub mod navigation_keys;
// Contains saving logic
pub mod save_key;
// Contains the copy/paste logic
pub mod copy_paste;

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
	// If the x position is before the start of the line, return false
	editor.text_position != 0
}

// Check the end of line cursor condition
pub fn check_cursor_end_line(editor: &mut EditorSpace, line_num: usize) -> bool {
	// The line of text
	let line = match editor.blocks.as_ref().unwrap().get_line(line_num) {
		Ok(line) => line,
		Err(err) => panic!("Couldn't get line {} | {}", line_num, err),
	};
	let num_tabs = line.matches('\t').count();
	// If the cursor is beyond the end of the line, return false
	editor.cursor_position[0]
		< UnicodeWidthStr::width(line.as_str()) + num_tabs * (editor.config.tab_width - 1)
		&& editor.cursor_position[0] < editor.width
}

// Calls the UnRedoStack undo and sets the editor's state
pub fn undo(editor: &mut EditorSpace) {
	// Get the current editor state
	let state = editor.get_unredo_state();
	// Take the undo action and return the new editor state
	let state = editor.unredo_stack.undo(state);
	// Set the new editor's state
	editor.stored_position = state.0;
	editor.text_position = state.1;
	editor.cursor_position = state.2;
	editor.scroll_offset = state.3;
	editor.blocks = Some(state.4);
	editor.selection = state.5;
}
