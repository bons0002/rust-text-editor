use super::{
    check_cursor_end_line,
    end_key,
    Config,
    EditorSpace
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
pub fn move_cursor_line(
    editor: &mut EditorSpace,
    config: &Config,
    op: Operation,
    pos_operand: usize,
    cursor_pos_operand: usize,
) {
    // Add to the index (move to the next line)
    if op == Operation::ADD {
        add_branch(editor, config, pos_operand, cursor_pos_operand);
    } else {
        // Subtract from the index (move to previous line)
        sub_branch(editor, config, pos_operand, cursor_pos_operand);
    }
}

// The logic of the ADD branch of move_cursor_line
fn add_branch(
    editor: &mut EditorSpace,
    config: &Config,
    text_position_operand: usize,
    cursor_position_operand: usize,
) {
    // Line number of current line in the block
	let line_num = editor.text_position[1];
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];

    // The location of the next line in the text
    let next_text_line = line_num + text_position_operand;
    // Location of next line for screen cursor
    let next_cursor_line = cursor_line_num + cursor_position_operand;

    // Ensure that the cursor doesn't move beyond the end of the next line
    if check_cursor_end_line(editor, next_text_line) {
        // Get the x position on the next line (takes tabs into account)
        let next_text_position = calc_next_line_pos(editor, config, next_text_line);
        // Set position in text
        editor.text_position = [next_text_position, next_text_line];
        // Set screen cursor
        editor.cursor_position[1] = next_cursor_line;
    } else {
        // After end of line
        // Set cursor to beginning of line
        editor.text_position = [1, next_text_line];
        editor.cursor_position[1] = next_cursor_line;
        // Move cursor to end of line
        end_key(editor, config);
    }
}

// The logic of the SUB branch of move_cursor_line
fn sub_branch(
    editor: &mut EditorSpace,
    config: &Config,
    text_position_operand: usize,
    cursor_position_operand: usize,
) {
    // Line number of current line in the block
	let line_num = editor.text_position[1];
	// Line number of the screen number
	let cursor_line_num = editor.cursor_position[1];

    // The location of the previous line in the text
    let prev_text_line = line_num - text_position_operand;
    // Location of prev line for screen cursor
    let prev_cursor_line = cursor_line_num - cursor_position_operand;

    // Ensure that the cursor doesn't move beyond the end of the previous line
    if check_cursor_end_line(editor, prev_text_line) {
        // Get the x position on the previous line (takes tabs into account)
        let next_text_position = calc_next_line_pos(editor, config, prev_text_line);
        // Set position in text
        editor.text_position = [next_text_position, prev_text_line];
        // Set screen cursor
        editor.cursor_position[1] = prev_cursor_line;
    } else {
        // After end of line
        // Set cursor to beginning of line
        editor.text_position = [0, prev_text_line];
        editor.cursor_position[1] = prev_cursor_line;
        // Move cursor to end of line
        end_key(editor, config);
    }
}

// Calculate the x position of the cursor on the next line (accounting for tab character)
fn calc_next_line_pos(editor: &mut EditorSpace, config: &Config, alt_line_num: usize) -> usize {
    // Position on the current line
	let line_position = editor.text_position[0];
	// Line number of current line in the block
	let line_num = editor.text_position[1];

    // Count the number of tab characters up to the current position on the current line
    let curr_tab_chars = editor.block[line_num][0..line_position]
        .matches('\t')
        .count() as isize;
    // Count the number of tab characters up to the current position on the next line
    let next_tab_chars = editor.block[alt_line_num][0..line_position]
        .matches('\t')
        .count() as isize;
    // Difference in the number of tab chars between the two lines
    let diff = curr_tab_chars - next_tab_chars;
    // Calculate the position in the text when moving to the next line
    // This is done to account for tabs on the next line and adjusting accordingly
    let next_pos_0 = line_position as isize + (config.tab_width - 1) as isize * diff;
    // If the resulting position is non-negative, return it
    if next_pos_0 >= 0 {
        return next_pos_0 as usize;
    }
    // Otherwise, return 1
    0
}
