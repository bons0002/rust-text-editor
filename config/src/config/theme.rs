use ratatui::style::Color;

pub struct Theme {
    pub editor_fg: Color,
    pub editor_bg: Color,
    pub editor_highlight_fg_color: Color,
    pub editor_highlight_bg_color: Color,
    pub tab_fg: Color,
    pub tab_bg: Color,
}

impl Theme {
    pub fn dark_mode() -> Self {
        Theme {
            editor_fg: Color::White,
            editor_bg: Color::Black,
            editor_highlight_fg_color: Color::White,
            editor_highlight_bg_color: Color::DarkGray,
            tab_fg: Color::White,
            tab_bg: Color::Blue,
        }
    }
}