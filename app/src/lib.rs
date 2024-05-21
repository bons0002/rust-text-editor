use std::io::{self, stdout};

use crossterm::{
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::*,
};

// TEMP IMPORTS
use std::thread;
use std::time::Duration;

pub fn run() -> io::Result<()> {
    // Put stdout into raw mode (turn off canonical mode)
    enable_raw_mode()?;
    // Switches the terminal to an alternate screen
    stdout().execute(EnterAlternateScreen)?;
    // Show the cursor
    stdout().execute(Show)?;

    // Draw the terminal widgets
    // Temporarily not handling errors
    let _ = draw_terminal();

    // Turn off raw mode for stdout (enable canonical mode)
    disable_raw_mode()?;
    // Exit the alternate screen
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn draw_terminal() -> io::Result<()> {
    // Create a new terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|frame| {
            ui(frame);
        })?;
        thread::sleep(Duration::from_secs(5));
        break;
    }

    Ok(())
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Block::bordered(),
        frame.size()
    );
}