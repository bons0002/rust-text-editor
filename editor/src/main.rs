use std::io::{self, Write};
use std::{thread, time, env};
use std::fs::{self, File};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Change stdout from canonical to raw mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    // Get non blocking input from stdin
    let mut stdin = termion::async_stdin().keys();
    // Clear the terminal
    println!("{clear}{goto}", clear = termion::clear::All, goto = termion::cursor::Goto(1,1));

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open a file in read-write mode
    let mut file = match File::options()
        .read(true)
        .write(true)
        .open(&filename) {
            // Either open the file or create it if it doesn't exist
            Ok(file) => file,
            Err(_) => File::create(&filename).unwrap(),
        };

    // Get contents of file as string
    let mut contents = fs::read_to_string(&filename)
        .expect("Should have been able to read file");
    // Write contents of file to stdout
    write!(stdout, "{}", contents).unwrap();
    // Flush stdout
    stdout.lock().flush().unwrap();

    // Loop the app until ctrl-c is input
    loop {
        // Get the next key from stdin
        let input = stdin.next();
        
        if let Some(Ok(key)) = input {
            // Get the termion key and print the associated character
            match key {
                // Print the char to stdout
                termion::event::Key::Char(key) => {
                    // Append to the contents to be saved
                    contents.push(key);
                    // Write the char to stdout
                    write!(stdout, "{}", key).unwrap();
                    // Flush stdout
                    stdout.lock().flush().unwrap();
                }
                termion::event::Key::Backspace => {
                    contents.pop();
                    // Move the cursor left one position and insert a space (to delete the char)
                    write!(stdout, "{}{}{}", termion::cursor::Left(1), ' ', termion::cursor::Left(1)).unwrap();
                    // Flush stdout
                    stdout.lock().flush().unwrap();
                }
                // Save the file
                termion::event::Key::Ctrl('s') => {
                    file.write_all(contents.as_bytes()).unwrap_or_else(|err| {
                        eprintln!("Unable to write to {}: {}", filename, err);
                    });
                }
                // Quit the editor
                termion::event::Key::Ctrl('c') => {
                    break;
                }
                _ => (),
            }
        }
        thread::sleep(time::Duration::from_millis(1));
    }
}
