use super::{
	check_cursor_end_line,
	Config,
	EditorSpace,
	end_key,
};

// Tracks which operation to use in move_cursor_line
pub enum Operation {
	ADD,
	SUB,
}

// Implement a comparator for the operation
impl PartialEq for Operation {
	// Check whether the two enums are the same value
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ADD, Self::ADD) => true,
			(Self::SUB, Self::SUB) => true,
			_ => false,
		}
	}
}

// Move the cursor to the next or previous line (dependant on the operation)
pub fn move_cursor_line(editor: &mut EditorSpace, config: &Config, op: Operation, pos_operand: usize, cursor_pos_operand: usize) {
	// Add to the index (move to the next line)
	if op == Operation::ADD {
		add_branch(editor, config, pos_operand, cursor_pos_operand);
	} else {	// Subtract from the index (move to previous line)
		sub_branch(editor, config, pos_operand, cursor_pos_operand);
	}
}

// The logic of the ADD branch of move_cursor_line
fn add_branch(editor: &mut EditorSpace, config: &Config, pos_operand: usize, cursor_pos_operand: usize) {
	// The location of the next line in the text
	let idx_pos = editor.pos.1 + pos_operand;
	// Location of next line for screen cursor
	let idx_raw = editor.cursor_pos.1 + cursor_pos_operand;

	// Ensure that the cursor doesn't move beyond the end of the next line
	if check_cursor_end_line(editor, idx_pos) {
		// Get the x position on the next line (takes tabs into account)
		let next_pos_0 = calc_next_line_pos(editor, config, idx_pos);
		// Set position in text
		editor.pos = (next_pos_0, idx_pos);
		// Set screen cursor
		editor.cursor_pos = (editor.cursor_pos.0, idx_raw);
	} else {	// After end of line
		// Set cursor to beginning of line
		editor.pos = (1, idx_pos);
		editor.cursor_pos = (editor.cursor_pos.0, idx_raw);
		// Move cursor to end of line
		end_key(editor, config);
	}
}

// The logic of the SUB branch of move_cursor_line
fn sub_branch(editor: &mut EditorSpace, config: &Config, pos_operand: usize, cursor_pos_operand: usize) {
	// The location of the previous line in the text
	let idx_pos = editor.pos.1 - pos_operand;
	// Location of prev line for screen cursor
	let idx_raw = editor.cursor_pos.1 - cursor_pos_operand;
	
	// Ensure that the cursor doesn't move beyond the end of the previous line
	if check_cursor_end_line(editor, idx_pos) {
		// Get the x position on the previous line (takes tabs into account)
		let next_pos_0 = calc_next_line_pos(editor, config, idx_pos);
		// Set position in text
		editor.pos = (next_pos_0, idx_pos);
		// Set screen cursor
		editor.cursor_pos = (editor.cursor_pos.0, idx_raw);
	} else {	// After end of line
		// Set cursor to beginning of line
		editor.pos = (1, idx_pos);
		editor.cursor_pos = (editor.cursor_pos.0, idx_raw);
		// Move cursor to end of line
		end_key(editor, config);
	}
}

// Calculate the x position of the cursor on the next line (accounting for tab character)
fn calc_next_line_pos(editor: &mut EditorSpace, config: &Config, idx_pos: usize) -> usize {
	// Count the number of tab characters up to the current position on the current line
	let curr_tab_chars = editor.content[editor.pos.1][0..(editor.pos.0 - 1)].matches('\t').count() as isize;
	// Count the number of tab characters up to the current position on the next line
	let next_tab_chars = editor.content[idx_pos][0..(editor.pos.0 - 1)].matches('\t').count() as isize;
	// Difference in the number of tab chars between the two lines
	let diff = curr_tab_chars - next_tab_chars;
	// Calculate the position in the text when moving to the next line
	// This is done to account for tabs on the next line and adjusting accordingly
	let next_pos_0 = editor.pos.0 as isize + (config.tab_width - 1) as isize * diff;
	// If the resulting position is non-negative, return it
	if next_pos_0 >= 1 {
		return next_pos_0 as usize;
	}
	// Otherwise, return 1
	1
}