// Implementation of the module `key_functions` defined in `src/lib.rs` module `editor`

use std::io::Write;
use super::{EditorSpace, File, Config};

// Functionality of pressing a normal character key
pub fn char_key(editor: &mut EditorSpace, code: char) {
    // Insert the character
    editor.content[editor.pos.1].insert(editor.pos.0 - 1, code);
    // Move cursor
    editor.pos = (editor.pos.0 + 1, editor.pos.1);
    editor.raw_pos = (editor.raw_pos.0 + 1, editor.raw_pos.1);
}

// Functionality of pressing the enter key
pub fn enter_key(editor: &mut EditorSpace) {
    // Get the current cursor position
    let loc = (editor.pos.0, editor.pos.1);
    let line = &editor.content[loc.1];
    
    // Get the rest of the line after the cursor
    let after_cursor = get_after_cursor(line, loc.0);

    // Insert new row
    editor.content.insert(loc.1 + 1, String::from(after_cursor));
    // Remove the rest of the old row after the enter
    editor.content[loc.1].truncate(loc.0 - 1);

    // Reset cursor to beginning of line
    editor.pos = (1, loc.1 + 1);
    editor.raw_pos = (editor.width.0 + 1, editor.raw_pos.1 + 1);
}

pub fn backspace(editor: &mut EditorSpace, config: &Config) {
    let line = editor.content[editor.pos.1].clone();
        // Remove empty line
        if editor.pos.0 <= 1 {  // If cursor at beginning of line, move to above line
            if editor.content.len() > 1 {
                // Get the text from the rest of the line after the cursor
                let after_cursor = get_after_cursor(&line, editor.pos.0);
                // Remove the current line
                editor.content.remove(editor.pos.1);
                // Move up one line
                up_arrow(editor, config);
                end_key(editor, config);
                // Append the rest of the line to the previous line (where the cursor is moving to)
                editor.content[editor.pos.1].push_str(after_cursor);
            }
        } else {
            // Move cursor left
            left_arrow(editor, config);
            // Remove one character
            editor.content[editor.pos.1].remove(editor.pos.0 - 1);
        }
}

// Get the rest of the text on the line after the cursor
fn get_after_cursor(line: &String, loc: usize) -> &str {
    // Get the rest of the line and store 
    &line[loc - 1..]
}

// Functionality for the tab key
pub fn tab_key(editor: &mut EditorSpace, config: &Config) {
    // Insert tab character
    editor.content[editor.pos.1].insert(editor.pos.0 - 1, '\t');
    // Move cursor
    editor.pos = (editor.pos.0 + 1, editor.pos.1);
    editor.raw_pos = (editor.raw_pos.0 + config.tab_width, editor.raw_pos.1);
}

// Left arrow key functionality
pub fn left_arrow(editor: &mut EditorSpace, config: &Config) {
    // If the cursor doesn't move before the beginning of the editor block
    if check_cursor_begin_line(editor) {
        // If the next char isn't a tab, move normally
        if editor.content[editor.pos.1].chars().nth(editor.pos.0 - 2) != Some('\t') {
            editor.pos = (editor.pos.0 - 1, editor.pos.1);
            editor.raw_pos = (editor.raw_pos.0 - 1, editor.raw_pos.1);
        } else {    // Otherwise, move by the number of tab spaces
            editor.pos = (editor.pos.0 - 1, editor.pos.1);
            editor.raw_pos = (editor.raw_pos.0 - config.tab_width, editor.raw_pos.1);
        }
    } else { // Otherwise
        editor.pos = (1, editor.pos.1);
        editor.raw_pos = (editor.width.0 + 1, editor.raw_pos.1);
    }
}

// Right arrow key functionality
pub fn right_arrow(editor: &mut EditorSpace, config: &Config) {
    // Count the number of tab characters
    let tab_chars = editor.content[editor.pos.1].matches('\t').count() * (config.tab_width - 1);

    // If the cursor doesn't go beyond the end of the line
    if check_cursor_end_line(editor, editor.pos.1) {
        // If not a tab character, move normally
        if editor.content[editor.pos.1].chars().nth(editor.pos.0 - 1) != Some('\t') {
            editor.pos = (editor.pos.0 + 1, editor.pos.1);
            editor.raw_pos = (editor.raw_pos.0 + 1, editor.raw_pos.1);
        } else {    // Otherwise, move the number of tab spaces
            editor.pos = (editor.pos.0 + 1, editor.pos.1);
            editor.raw_pos = (editor.raw_pos.0 + config.tab_width, editor.raw_pos.1);
        }
    } else { // Otherwise
        editor.pos = (editor.content[editor.pos.1].len() + 1, editor.pos.1);
        // Raw cursor must take into account the end of the line plus the number of tabs
        editor.raw_pos = (editor.width.0 + editor.content[editor.pos.1].len() + 1 + tab_chars, editor.raw_pos.1);
    }
}

// Up arrow key functionality
pub fn up_arrow(editor: &mut EditorSpace, config: &Config) {
    // Ensure that the cursor doesn't move above the editor block
    if editor.raw_pos.1 > editor.height.0 + 1 {
        // Location of line above
        let idx_pos = editor.pos.1 - 1;
        let idx_raw = editor.raw_pos.1 - 1;
        // Count the number of tab characters
        let tab_chars = editor.content[idx_pos].matches('\t').count() * (config.tab_width - 1);

        // Check that the cursor doesn't move beyond the end of the above line
        // Cursor before end of line
        if check_cursor_end_line(editor, idx_pos) {
            editor.pos = (editor.pos.0, idx_pos);
            editor.raw_pos = (editor.raw_pos.0, idx_raw);
        } else {    // After end of line
            editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
            editor.raw_pos = (editor.width.0 + editor.content[idx_pos].len() + 1 + tab_chars, idx_raw);
        }
    } else if editor.scroll_offset.0 > 0 {    // If the cursor moves beyond the bound, scroll up
        // Scroll up
        editor.scroll_offset = (editor.scroll_offset.0 - 1, editor.scroll_offset.1);
        // Location of line below
        let idx_pos = editor.pos.1 - 1;
        // Check that the cursor doesn't move beyond the end of the above line
        // Cursor before end of line
        if check_cursor_end_line(editor, editor.pos.1 - 1) {
            editor.pos = (editor.pos.0, idx_pos);
        } else {
            editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
        }
    }
}

// Down arrow key functionality
pub fn down_arrow(editor: &mut EditorSpace, config: &Config) {
    // Ensure that the cursor doesn't move below the editor block
    if editor.pos.1 < ((editor.height.1 - editor.height.0) - 3) {
        // Location of line below
        let idx_pos = editor.pos.1 + 1;
        let idx_raw = editor.raw_pos.1 + 1;
        // Count the number of tab characters
        let tab_chars = editor.content[idx_pos].matches('\t').count() * (config.tab_width - 1);

        // Check that the cursor doesn't move beyond the end of the next line
        if check_cursor_end_line(editor, idx_pos) {
            editor.pos = (editor.pos.0, idx_pos);
            editor.raw_pos = (editor.raw_pos.0, idx_raw);
        } else {    // After end of line
            editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
            editor.raw_pos = (editor.width.0 + editor.content[idx_pos].len() + 1 + tab_chars, idx_raw);
        }
    }
    else if editor.scroll_offset.0 < editor.content.len() as u16 {  // If the cursor goes below the bound, scroll down
        // Scroll down
        editor.scroll_offset = (editor.scroll_offset.0 + 1, editor.scroll_offset.1);
        // Location of line below
        let idx_pos = editor.pos.1 + 1;
        // Check that the cursor doesn't move beyond the end of the above line
        // Cursor before end of line
        if check_cursor_end_line(editor, editor.pos.1 + 1) {
            editor.pos = (editor.pos.0, idx_pos);
        } else {
            editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
        }
    }
}

// Check the end of line cursor condition
fn check_cursor_end_line(editor: &mut EditorSpace, idx: usize) -> bool {
    // If the x position is beyond the end of the line, return false
    if editor.pos.0 > editor.content[idx].chars().count() {
        return false;
    }
    true
}

// Check the beginning of line cursor condition
fn check_cursor_begin_line(editor: &mut EditorSpace) -> bool {
    // If the x position is before the start of the line, return false
    if editor.pos.0 <= 1 {
        return false;
    }
    true
}

// Home key functionality
pub fn home_key(editor: &mut EditorSpace) {
    // Move to beginning of line
    editor.pos = (1, editor.pos.1);
    editor.raw_pos = (editor.width.0 + 1, editor.raw_pos.1)
}

// End key functionality
pub fn end_key(editor: &mut EditorSpace, config: &Config) {
    // Count the number of tab characters
    let tab_chars = editor.content[editor.pos.1].matches('\t').count() * (config.tab_width - 1);

    // Move to end of line
    editor.pos = (editor.content[editor.pos.1].len() + 1, editor.pos.1);
    editor.raw_pos = (editor.width.0 + editor.content[editor.pos.1].len() + 1 + tab_chars, editor.raw_pos.1);
}

// Save key combo functionality
pub fn save_key_combo(editor: &mut EditorSpace) {
    // Create a blank file
    let mut file = match File::create(&editor.filename) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file"),
    };
    // Write the content to the new file
    for line in &editor.content {
        write!(file, "{}\n", line).unwrap();
    }
}