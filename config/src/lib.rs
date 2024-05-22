use crossterm::cursor::SetCursorStyle;

pub struct Config {
    pub cursor_style: SetCursorStyle,
    pub tab_width: u16,
}

impl Config {
    pub fn new(cursor_style: SetCursorStyle, tab_width: u16) -> Self {
        Config {
            cursor_style,
            tab_width,
        }
    }
}