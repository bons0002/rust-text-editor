use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

// Get the key pressed
pub fn handle_input(text: &mut String) {
    match event::read().unwrap() {
        // Return the character if only a key (without moodifier key) is pressed
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            // Return the key
            match code {
                KeyCode::Char(code) => text.push(code),
                KeyCode::Enter => text.push('\n'),
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
                KeyCode::Char(code) => text.push(code.to_ascii_uppercase()),
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

