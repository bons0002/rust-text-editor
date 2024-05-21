use std::{
    fs::{self, File},
    path::Path,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Editor {
    // Name of file opened in current editor frame
    pub filename: String,
    // The raw text of the file
    pub raw_content: String,
    // Text content of current frame
    pub content: Vec<String>,
    // Position in the raw string
    pub raw_pos: (u16, u16),
    // Position of cursor in current vector
    pub pos: (u16, u16),
    // Track if the starting cursor position has already been set
    pub start_cursor_set: bool,
}

impl Editor {
    pub fn new(filename: String) -> Editor {
        // Check if a file exists, if not create it
        if !Path::new(&filename).exists() {
            File::create(&filename).unwrap();
        }
        // Get the file as a vector of string (for lines) and a raw string
        let result = Self::parse_file(&filename);
        Editor {
            // Read in the contents of the file
            content: result.0,
            raw_content: result.1,
            filename,
            raw_pos: (0,0),
            pos: (0, 0),
            start_cursor_set: false,
        }
    }

    // Parse the specified file to a vector of strings (each element representing a line) as a string for the raw data
    fn parse_file(filename: &String) -> (Vec<String>, String) {
        // Read the file to a string
        let content = fs::read_to_string(&filename)
                .expect("Couldn't read file");
        // Copy file string
        let raw_content = content.clone();

        // Split the string into lines
        let lines = content.lines();
        // Add the lines to a vector
        let mut result: Vec<String> = Vec::new();
        for line in lines {
            result.push(String::from(line));
        }
        // Return the vector and raw string
        return (result, raw_content);
    }

    // Set the starting position of the editing space cursor
    pub fn set_starting_pos(&mut self, start: (u16, u16)) {
        let text_pos = (
            (self.content[self.content.len() - 1].len() + 1) as u16,
            (self.content.len()) as u16,
        );
        self.raw_pos = (start.0 + text_pos.0, start.1 + text_pos.1);
        self.pos = (text_pos.0, text_pos.1 - 1);
    }

    // Get the key pressed
pub fn handle_input(&mut self) {
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
                    //let last_idx = self.content.len();
                    let idx = self.pos.1 as usize;
                    self.content[idx].push(code);
                    self.raw_content.push(code);
                }
                // If Enter was pressed, insert newline
                KeyCode::Enter => {
                    let idx = self.pos.1 as usize;
                    // Add newline to the row
                    self.content[idx].push('\n');
                    // Insert new row
                    self.content.push(String::from(""));
                    // Add newline to the string
                    self.raw_content.push('\n');
                }
                _ => (),
            }
        },

        // Uppercase letters (Using shift)
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::SHIFT,
            ..
        }) => {
            match code {
                KeyCode::Char(code) => {
                    let idx = self.pos.1 as usize;
                    self.content[idx].push(code.to_ascii_uppercase());
                    self.raw_content.push(code.to_ascii_uppercase());
                }
                _ => ()
            }
        }

        _ => (),
    }
}

}

#[cfg(test)]
mod tests {
    use super::*;
}

