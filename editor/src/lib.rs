use std::fs::{self, File};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Editor {
    // Name of file opened in current editor frame
    pub filename: String,
    // Text content of current frame
    pub content: String,
    // Position of cursor in current frame
    pub pos: (u16, u16),
}

impl Editor {
    pub fn new(filename: String) -> Editor {
        // Check if a file exists, if not create it
        let file = match File::options()
            .read(true)
            .write(true)
            .open(&filename) {
                Ok(file) => file,
                Err(_) => File::create(&filename).unwrap(),
            };
        Editor {
            // Read in the contents of the file
            content: fs::read_to_string(&filename)
                .expect("Unable to read file"),
            filename,
            pos: (0, 0),
        }
    }
}

// Get the key pressed
pub fn handle_input(editor_space: &mut Editor) {
    match event::read().unwrap() {
        // Return the character if only a key (without moodifier key) is pressed
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            // Return the key
            match code {
                KeyCode::Char(code) => editor_space.content.push(code),
                KeyCode::Enter => editor_space.content.push('\n'),
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
                KeyCode::Char(code) => editor_space.content.push(code.to_ascii_uppercase()),
                _ => ()
            }
        }

        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

