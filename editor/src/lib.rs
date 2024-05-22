use std::{
    fs::{self, File},
    io::Write, 
    path::Path, 
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use config;

pub struct Editor {
    // Name of file opened in current editor frame
    pub filename: String,
    // Text content of current frame
    pub content: Vec<String>,
    // Horizontal bounds of the editor block
    width: (usize, usize),
    // Vertical bounds of the editor block
    height: (usize, usize),
    // Position in the raw string
    pub raw_pos: (usize, usize),
    // Position of cursor in current vector
    pub pos: (usize, usize),
    // Track if the starting cursor position has already been set
    pub start_cursor_set: bool,
    // TEMP bool to break the main loop
    pub break_loop: bool,
}

impl Editor {
    pub fn new(filename: String) -> Editor {
        // Check if a file exists, if not create it
        if !Path::new(&filename).exists() {
            File::create(&filename).unwrap();
        }
        Editor {
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
    pub fn set_starting_pos(&mut self, start: (usize, usize), width: usize, height: usize) {
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
        // Flag that cursor has been initialized
        self.start_cursor_set = true;
    }

    // Return the vector as a paragraph
    pub fn get_paragraph(&self) -> String {
        self.content.join("\n").replace('\t', "    ")
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
                            // Insert the character
                            self.content[self.pos.1].insert(self.pos.0 - 1, code);
                            // Move cursor
                            self.pos = (self.pos.0 + 1, self.pos.1);
                            self.raw_pos = (self.raw_pos.0 + 1, self.raw_pos.1);
                        }
                        // If Enter was pressed, insert newline
                        KeyCode::Enter => {
                            let loc = (self.pos.0, self.pos.1);
                            let  mut after_cursor = "";
                            if loc.0 < self.content[loc.1].len() {
                                after_cursor = &self.content[loc.1][loc.0..];
                            }
                            // Insert new row
                            self.content.insert(self.pos.1 + 1, String::from(after_cursor));
                            // Reset cursor to beginning of line
                            self.pos = (1, self.pos.1 + 1);
                            self.raw_pos = (self.width.0 + 1, self.raw_pos.1 + 1);
                        }
                        // If tab was pressed, insert tab character
                        KeyCode::Tab => {
                            // Insert tab character
                            self.content[self.pos.1].insert(self.pos.0 - 1, '\t');
                            // Move cursor
                            self.pos = (self.pos.0 + 1, self.pos.1);
                            self.raw_pos = (self.raw_pos.0 + config.tab_width, self.raw_pos.1);
                        }
                        // Left arrow moves cursor left
                        KeyCode::Left => {
                            // If the cursor doesn't move before the beginning of the editor block
                            if self.check_cursor_begin_line() {
                                self.pos = (self.pos.0 - 1, self.pos.1);
                                self.raw_pos = (self.raw_pos.0 - 1, self.raw_pos.1);
                            } else { // Otherwise
                                self.pos = (0, self.pos.1);
                                self.raw_pos = (self.width.0 + 1, self.raw_pos.1);
                            }
                        }
                        // Right arrow moves cursor right
                        KeyCode::Right => {
                            // Count the number of tab characters
                            let tab_chars = self.content[self.pos.1 as usize].matches('\t').count() * (config.tab_width - 1);

                            // If the cursor doesn't go beyond the end of the line
                            if self.check_cursor_end_line(self.pos.1 as usize) {
                                self.pos = (self.pos.0 + 1, self.pos.1);
                                self.raw_pos = (self.raw_pos.0 + 1, self.raw_pos.1);
                            } else { // Otherwise
                                self.pos = (self.content[self.pos.1].len() + 1, self.pos.1);
                                self.raw_pos = (self.width.0 + self.content[self.pos.1].len() + 1 + tab_chars, self.raw_pos.1);
                            }
                        }
                        // Up arrow move cursor up one line
                        KeyCode::Up => {
                            // Ensure that the cursor doesn't move above the editor block
                            if self.pos.1 > 0 {
                                // Location of line above
                                let idx_pos = self.pos.1 - 1;
                                let idx_raw = self.raw_pos.1 - 1;
                                // Count the number of tab characters
                                let tab_chars = self.content[idx_pos as usize].matches('\t').count() * (config.tab_width - 1);

                                // Check that the cursor doesn't move beyond the end of the above line
                                // Cursor before end of line
                                if self.check_cursor_end_line(self.pos.1 - 1) {
                                    self.pos = (self.pos.0, idx_pos);
                                    self.raw_pos = (self.raw_pos.0, idx_raw);
                                } else {    // After end of line
                                    self.pos = (self.content[idx_pos].len() + 1, idx_pos);
                                    self.raw_pos = (self.width.0 + self.content[idx_pos].len() + 1 + tab_chars, idx_raw);
                                }
                            }
                        }
                        // Down arrow move cursor down one line
                        KeyCode::Down => {
                            // Ensure that the cursor doesn't move below the editor block
                            if self.pos.1 < self.content.len() {

                            }
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
                            // Index of the line
                            let idx = self.pos.1;
                            // Insert at the location
                            let loc = self.pos.0;
                            // Make sure there isn't overflow
                            if loc >= self.content[idx].len() {
                                self.content[idx].push(code.to_ascii_uppercase());
                            } else {
                                self.content[idx].insert(loc - 1, code.to_ascii_uppercase());
                            }
                            // Move cursor
                            self.pos = (self.pos.0 + 1, self.pos.1);
                            self.raw_pos = (self.raw_pos.0 + 1, self.raw_pos.1);
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
                            let mut file = match File::options()
                                .read(false)
                                .write(true)
                                .open(&self.filename) {
                                    Ok(file) => file,
                                    Err(_) => File::create(&self.filename).unwrap(),
                                };
                            for line in &self.content {
                                write!(file, "{}\n", line).unwrap();
                            }
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
        if self.pos.0 <= 0 {
            return false;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    //use super::*;
}
