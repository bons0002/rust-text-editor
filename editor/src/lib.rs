pub mod editor {

    use std::{
        fs::{self, File},
        path::Path, 
        time::Duration,
    };
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use config;

    pub struct EditorSpace {
        // Name of file opened in current editor frame
        pub filename: String,
        // Text content of current frame
        pub content: Vec<String>,
        // Horizontal bounds of the editor block
        width: (usize, usize),
        // Vertical bounds of the editor block
        height: (usize, usize),
        // Position on the screen (frontend cursor)
        pub raw_pos: (usize, usize),
        // Position of cursor in the actual data vector (backend cursor)
        pub pos: (usize, usize),
        // Track if the starting cursor position has already been set
        pub start_cursor_set: bool,
        // TEMP bool to break the main loop
        pub break_loop: bool,
    }

    impl EditorSpace {
        pub fn new(filename: String) -> Self {
            // Check if a file exists, if not create it
            if !Path::new(&filename).exists() {
                File::create(&filename).unwrap();
            }
            EditorSpace {
                // Read in the contents of the file
                content: Self::parse_file(&filename),
                filename,
                width: (0,0),
                height: (0,0),
                raw_pos: (0,0),
                pos: (0, 0),
                start_cursor_set: false,
                break_loop: false,
            }
        }

        // Parse the specified file to a vector of strings (each element representing a line) as a string for the raw data
        fn parse_file(filename: &String) -> Vec<String> {
            // Read the file to a string
            let content = fs::read_to_string(&filename)
                    .expect("Couldn't read file");

            // Vector containing text lines
            let mut result: Vec<String> = Vec::new();

            if !content.is_empty() {
                // Split the string into lines
                let lines = content.lines();
                // Add the lines to a vector
                for line in lines {
                    result.push(String::from(line));
                }
                // Return the vector and raw string
                return result;
            }
            // If there is no text in the file being opened, push an empty line to the vector
            result.push(String::from(""));
            return result;
        }

        // Set the starting position of the editing space cursor
        pub fn set_starting_pos(&mut self, config: &config::Config, start: (usize, usize), width: usize, height: usize) {
            // Position of visible text in frame
            let text_pos = (
                (self.content[self.content.len() - 1].len() + 1),
                (self.content.len()),
            );
            // Position of cursor in frame
            self.raw_pos = (start.0 + text_pos.0, start.1 + text_pos.1);
            // Position of cursor in actual text data
            self.pos = (text_pos.0, text_pos.1 - 1);
            
            // Set the bounds of the block
            self.width = (start.0, start.0 + width);
            self.height = (start.1, start.1 + height);

            // Make sure to move cursor to end of line
            key_functions::end_key(self, config);

            // Flag that cursor has been initialized
            self.start_cursor_set = true;
        }

        // Return the vector as a paragraph
        pub fn get_paragraph(&self, tab_width: usize) -> String {
            let mut spaces = String::from("");
            for _i in 0..tab_width {
                spaces.push(' ');
            }
            self.content.join("\n").replace('\t', &spaces)
        }

        // Get the key pressed
        pub fn handle_input(&mut self, config: &config::Config) {
            // Non-blocking read
            if event::poll(Duration::from_millis(50)).unwrap() {
                // Read input
                match event::read().unwrap() {

                    // Return the character if only a key (without moodifier key) is pressed
                    Event::Key(KeyEvent {
                        code,
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => {
                        // Return the key
                        match code {
                            // If normal character, insert that character
                            KeyCode::Char(code) => {
                                key_functions::char_key(self, code);
                            }
                            // If Enter was pressed, insert newline
                            KeyCode::Enter => {
                                key_functions::enter_key(self);
                            }
                            // If tab was pressed, insert tab character
                            KeyCode::Tab => {
                                key_functions::tab_key(self, config)
                            }
                            // Left arrow moves cursor left
                            KeyCode::Left => {
                                key_functions::left_arrow(self, config);
                            }
                            // Right arrow moves cursor right
                            KeyCode::Right => {
                                key_functions::right_arrow(self, config);
                            }
                            // Up arrow move cursor up one line
                            KeyCode::Up => {
                                key_functions::up_arrow(self, config);
                            }
                            // Down arrow move cursor down one line
                            KeyCode::Down => {
                                key_functions::down_arrow(self, config);
                            }
                            // Home button moves to beginning of line
                            KeyCode::Home => {
                                key_functions::home_key(self);
                            }
                            // End button move to end of line
                            KeyCode::End => {
                                key_functions::end_key(self, config);
                            }

                            _ => (),
                        
                        }
                    },

                    // Shift modifier key
                    Event::Key(KeyEvent {
                        code,
                        modifiers: KeyModifiers::SHIFT,
                        ..
                    }) => {
                        match code {
                            // Uppercase characters
                            KeyCode::Char(code) => {
                                key_functions::char_key(self, code.to_ascii_uppercase());
                            }
                            _ => ()
                        }
                    }

                    // Control modified keys
                    Event::Key(KeyEvent {
                        code,
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        match code {
                            // Save the frame to the file
                            KeyCode::Char('s') => {
                                key_functions::save_key_combo(self);
                            }
                            // Break the loop to end the program
                            KeyCode::Char('c') => {
                                self.break_loop = true;
                            }
                            _ => (),
                        }
                    }

                    _ => (),
                }
            }
        }

        // Check the end of line cursor condition
        fn check_cursor_end_line(&mut self, idx: usize) -> bool {
            // If the x position is beyond the end of the line, return false
            if self.pos.0 as usize > self.content[idx].chars().count() {
                return false;
            }
            true
        }

        // Check the beginning of line cursor condition
        fn check_cursor_begin_line(&mut self) -> bool {
            // If the x position is before the start of the line, return false
            if self.pos.0 <= 1 {
                return false;
            }
            true
        }
    }

    // Module containing all the functionality of each key. Called in handle_input
    mod key_functions {

        use std::io::Write;
        use super::{EditorSpace, File, config};

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
            // String to store everything on the current line after the cursor
            let  mut after_cursor = "";
            if loc.0 < editor.content[loc.1].len() {
                after_cursor = &editor.content[loc.1][loc.0 - 1..];
            }

            // Insert new row
            editor.content.insert(editor.pos.1 + 1, String::from(after_cursor));
            // Remove the rest of the old row after the enter
            editor.content[loc.1].truncate(loc.0 - 1);

            // Reset cursor to beginning of line
            editor.pos = (1, editor.pos.1 + 1);
            editor.raw_pos = (editor.width.0 + 1, editor.raw_pos.1 + 1);
        }

        // Functionality for the tab key
        pub fn tab_key(editor: &mut EditorSpace, config: &config::Config) {
            // Insert tab character
            editor.content[editor.pos.1].insert(editor.pos.0 - 1, '\t');
            // Move cursor
            editor.pos = (editor.pos.0 + 1, editor.pos.1);
            editor.raw_pos = (editor.raw_pos.0 + config.tab_width, editor.raw_pos.1);
        }

        // Left arrow key functionality
        pub fn left_arrow(editor: &mut EditorSpace, config: &config::Config) {
            // If the cursor doesn't move before the beginning of the editor block
            if editor.check_cursor_begin_line() {
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
        pub fn right_arrow(editor: &mut EditorSpace, config: &config::Config) {
            // Count the number of tab characters
            let tab_chars = editor.content[editor.pos.1].matches('\t').count() * (config.tab_width - 1);

            // If the cursor doesn't go beyond the end of the line
            if editor.check_cursor_end_line(editor.pos.1) {
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
        pub fn up_arrow(editor: &mut EditorSpace, config: &config::Config) {
            // Ensure that the cursor doesn't move above the editor block
            if editor.pos.1 > 0 {
                // Location of line above
                let idx_pos = editor.pos.1 - 1;
                let idx_raw = editor.raw_pos.1 - 1;
                // Count the number of tab characters
                let tab_chars = editor.content[idx_pos].matches('\t').count() * (config.tab_width - 1);

                // Check that the cursor doesn't move beyond the end of the above line
                // Cursor before end of line
                if editor.check_cursor_end_line(idx_pos) {
                    editor.pos = (editor.pos.0, idx_pos);
                    editor.raw_pos = (editor.raw_pos.0, idx_raw);
                } else {    // After end of line
                    editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
                    editor.raw_pos = (editor.width.0 + editor.content[idx_pos].len() + 1 + tab_chars, idx_raw);
                }
            }
        }

        // Down arrow key functionality
        pub fn down_arrow(editor: &mut EditorSpace, config: &config::Config) {
            // Ensure that the cursor doesn't move below the editor block
            if editor.pos.1 < editor.content.len() - 1 {
                // Location of line below
                let idx_pos = editor.pos.1 + 1;
                let idx_raw = editor.raw_pos.1 + 1;
                // Count the number of tab characters
                let tab_chars = editor.content[idx_pos].matches('\t').count() * (config.tab_width - 1);

                // Check that the cursor doesn't move beyond the end of the next line
                if editor.check_cursor_end_line(idx_pos) {
                    editor.pos = (editor.pos.0, idx_pos);
                    editor.raw_pos = (editor.raw_pos.0, idx_raw);
                } else {    // After end of line
                    editor.pos = (editor.content[idx_pos].len() + 1, idx_pos);
                    editor.raw_pos = (editor.width.0 + editor.content[idx_pos].len() + 1 + tab_chars, idx_raw);
                }
            }
        }

        // Home key functionality
        pub fn home_key(editor: &mut EditorSpace) {
            // Move to beginning of line
            editor.pos = (1, editor.pos.1);
            editor.raw_pos = (editor.width.0 + 1, editor.raw_pos.1)
        }

        // End key functionality
        pub fn end_key(editor: &mut EditorSpace, config: &config::Config) {
            // Count the number of tab characters
            let tab_chars = editor.content[editor.pos.1].matches('\t').count() * (config.tab_width - 1);

            // Move to end of line
            editor.pos = (editor.content[editor.pos.1].len() + 1, editor.pos.1);
            editor.raw_pos = (editor.width.0 + editor.content[editor.pos.1].len() + 1 + tab_chars, editor.raw_pos.1);
        }

        // Save key combo functionality
        pub fn save_key_combo(editor: &mut EditorSpace) {
            let mut file = match File::options()
                .read(false)
                .write(true)
                .open(&editor.filename) {
                    Ok(file) => file,
                    Err(_) => File::create(&editor.filename).unwrap(),
                };
            for line in &editor.content {
                write!(file, "{}\n", line).unwrap();
            }
        }

    }
}


#[cfg(test)]
mod tests {
    //use super::*;
}
