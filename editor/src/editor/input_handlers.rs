use super::{
	copy_paste, editing_keys, highlight_keys, key_functions, navigation_keys, save_key,
	EditorSpace, KeyCode,
};

pub fn no_modifiers(editor: &mut EditorSpace, code: KeyCode) {
	// Return the key
	match code {
		// If normal character, insert that character
		KeyCode::Char(code) => editing_keys::char_key(editor, code),
		// If Enter was pressed, insert newline
		KeyCode::Enter => editing_keys::enter_key(editor),
		// If tab was pressed, insert tab character
		KeyCode::Tab => editing_keys::tab_key(editor),
		// If backspace was pressed, remove the previous character
		KeyCode::Backspace => editing_keys::backspace(editor),
		// If delete was pressed, remove the next character
		KeyCode::Delete => editing_keys::delete_key(editor),
		// Left arrow moves cursor left
		KeyCode::Left => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Left arrow functionality
			navigation_keys::left_arrow(editor, true);
		}
		// Right arrow moves cursor right
		KeyCode::Right => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Right arrow functionality
			navigation_keys::right_arrow(editor, true);
		}
		// Up arrow move cursor up one line
		KeyCode::Up => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Up arrow functionality
			navigation_keys::up_arrow(editor);
		}
		// Down arrow move cursor down one line
		KeyCode::Down => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Down arrow functionality
			navigation_keys::down_arrow(editor);
		}
		// Home button moves to beginning of line
		KeyCode::Home => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Home key functionality
			navigation_keys::home_key(editor, true);
		}
		// End button move to end of line
		KeyCode::End => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// End key functionality
			navigation_keys::end_key(editor, true);
		}
		// The Page Up key moves up one `height` of the editor widget
		KeyCode::PageUp => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Move one page up
			navigation_keys::page_up(editor);
		}
		// The Page Down key moves down one `height` of the editor widget
		KeyCode::PageDown => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Move one page down
			navigation_keys::page_down(editor);
		}
		_ => (),
	}
}

pub fn shift_modifier(editor: &mut EditorSpace, code: KeyCode) {
	match code {
		// Uppercase characters
		KeyCode::Char(code) => editing_keys::char_key(editor, code.to_ascii_uppercase()),
		// Right arrow highlight text to the right
		KeyCode::Right => highlight_keys::highlight_right(editor),
		// Left arrow highlight text to the left
		KeyCode::Left => highlight_keys::highlight_left(editor),
		// Up arrow highlights text upwards
		KeyCode::Up => highlight_keys::highlight_up(editor),
		// Down arrow highlights text downwards
		KeyCode::Down => highlight_keys::highlight_down(editor),
		// End key highlights to end of line
		KeyCode::End => highlight_keys::highlight_end(editor),
		// Home key highlights to beginning of line
		KeyCode::Home => highlight_keys::highlight_home(editor),
		// Highlight one page up
		KeyCode::PageUp => highlight_keys::highlight_page_up(editor),
		// Highlight one page down
		KeyCode::PageDown => highlight_keys::highlight_page_down(editor),
		_ => (),
	}
}

pub fn control_modifier(editor: &mut EditorSpace, code: KeyCode, break_loop: &mut bool) {
	match code {
		// Save the frame to the file
		KeyCode::Char('s') => save_key::save_key_combo(editor, false, ""),
		// Break the loop to end the program
		KeyCode::Char('q') => *break_loop = true,
		// Paste text into the editor (if there is a clipboard)
		KeyCode::Char('v') => {
			if editor.clipboard.is_some() {
				copy_paste::paste_from_clipboard(editor)
			}
		}
		// Copy text from the editor and write it to the clipboard
		KeyCode::Char('c') => {
			if editor.clipboard.is_some() {
				copy_paste::copy_to_clipboard(editor)
			}
		}
		// Undo a change
		KeyCode::Char('z') => {
			key_functions::undo(editor);
		}
		// Jump to the next word
		KeyCode::Right => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Jump to the next word
			navigation_keys::jump_right(editor, false);
		}
		// Jump to the previous word
		KeyCode::Left => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Jump to the previous word
			navigation_keys::jump_left(editor, false);
		}
		// Jump up 10 lines
		KeyCode::Up => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Jump up 10 lines
			navigation_keys::jump_up(editor, false);
		}
		// Jump down 10 lines
		KeyCode::Down => {
			// Clear the highlighted selection of text
			editor.selection.is_empty = true;
			// Jump down 10 lines
			navigation_keys::jump_down(editor, false);
		}
		_ => (),
	}
}

pub fn control_and_shift_modifiers(editor: &mut EditorSpace, code: KeyCode) {
	match code {
		// Highlight the entire unicode word to the right
		KeyCode::Right => navigation_keys::jump_right(editor, true),
		// Highlight the entire unicode word to the left
		KeyCode::Left => navigation_keys::jump_left(editor, true),
		// Highlight upwards 10 lines
		KeyCode::Up => navigation_keys::jump_up(editor, true),
		// Highlight down 10 lines
		KeyCode::Down => navigation_keys::jump_down(editor, true),
		_ => (),
	}
}
