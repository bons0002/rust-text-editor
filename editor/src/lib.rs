pub mod editor {

    use std::{
        fs::{self, File},
        iter,
        path::Path,
        time::Duration,
    };
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{
        style::Style,
        text::{Line, Text},
        widgets::Paragraph,
    };

    use config::config::Config;
    use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};

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
        // Sets the amount to scroll the text
        pub scroll_offset: (u16, u16),
        // TEMP bool to break the main loop
        pub break_loop: bool,
    }

    impl EditorSpace {
        pub fn new(filename: String, config: &Config) -> Self {
            // Check if a file exists, if not create it
            if !Path::new(&filename).exists() {
                File::create(&filename).unwrap();
            }
            EditorSpace {
                // Read in the contents of the file
                content: Self::parse_file(&filename, config),
                filename,
                width: (0,0),
                height: (0,0),
                raw_pos: (0,0),
                pos: (0, 0),
                start_cursor_set: false,
                scroll_offset: (0,0),
                break_loop: false,
            }
        }

        // Parse the specified file to a vector of strings (each element representing a line) as a string for the raw data
        fn parse_file(filename: &String, config: &Config) -> Vec<String> {
            // Read the file to a string
            let content = fs::read_to_string(&filename)
                    .expect("Couldn't read file");

            // Vector containing text lines
            let mut result: Vec<String> = Vec::new();
            // String of spaces of length tab_width. Used to replace space indentation with tab indentation
            let tab_spaces = &iter::repeat(" ").take(config.tab_width).collect::<String>();

            if !content.is_empty() {
                let line_split: Vec<&str> = content.split('\n').collect();
                // Split the string into lines
                let mut lines: Vec<String> = Vec::new();
                // Convert this split lines into a vector of strings
                lines.par_extend(line_split
                    .into_par_iter()    // Parallel iterator
                    .map(|line| {   // Operation
                        let temp = String::from(line);
                        temp
                    }));
                
                // Add each line (with space indentation replaced with tabs) to the vector of strings
                result.par_extend(lines
                    .into_par_iter()
                    .map(|line| {
                        line.replace(tab_spaces, "\t")
                    }));
                // Return the vector and raw string
                return result;
            }
            // If there is no text in the file being opened, push an empty line to the vector
            result.push(String::from(""));
            return result;
        }

        // Set the starting position of the editing space cursor
        pub fn set_starting_pos(&mut self, start: (usize, usize), width: usize, height: usize) {
            // Set the bounds of the block
            self.width = (start.0, start.0 + width);
            self.height = (start.1, start.1 + height);

            // Set the cursor to the beginning of the block
            self.pos = (1,0);
            self.raw_pos = (self.width.0 + 1, self.height.0 + 1);

            // Flag that cursor has been initialized
            self.start_cursor_set = true;
        }

        // Return the vector as a paragraph
        pub fn get_paragraph(&self, config: &Config) -> Paragraph {
            // Vector to store the lines
            let mut lines: Vec<Line> = Vec::new();
            let content = &self.content;
            
            // Start tab with a vertical line
            let mut tab_char = String::from("\u{023D0}");
            // Iterator to create a string of tab_width - 1 number of spaces
            tab_char.push_str(&iter::repeat(" ").take(config.tab_width - 1).collect::<String>());
            
            // Create a vector of Lines (in parallel)
            lines.par_extend(content.into_par_iter().map(|part| {
                Line::from(
                    // Display tabs as a series of spaces
                    part.replace(
                        '\t',
                        tab_char.as_str(),
                    )
                )
            }));

            // Highlight the selected line
            lines[self.pos.1] = lines[self.pos.1].clone().style(Style::default()
                .fg(config.theme.editor_highlight_fg_color)
                .bg(config.theme.editor_highlight_bg_color)
            );

            // Return a paragraph from the lines
            Paragraph::new(Text::from(lines))
                .scroll(self.scroll_offset)
        }

        // Get the key pressed
        pub fn handle_input(&mut self, config: &Config) {
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
                                key_functions::tab_key(self, config);
                            }
                            // If backspace was pressed, remove the previous character
                            KeyCode::Backspace => {
                                key_functions::backspace(self, config);
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
