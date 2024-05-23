pub mod editor {

    use std::{
        fs::{self, File},
        path::Path, 
        time::Duration,
    };
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use config;

    // Module containing all the functionality of each key. Called in handle_input
    mod key_functions;

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
    }
}


#[cfg(test)]
mod tests {
    //use super::*;
}
