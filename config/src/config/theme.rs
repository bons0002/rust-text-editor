use ratatui::style::Color;

// Contains the colors of the editor
pub struct Theme {
    // Entire app's foreground (text) color
    pub app_fg: Color,
    // Entire app's background color
    pub app_bg: Color,
    // Editor space's highlighted line's foreground (text) color
    pub editor_highlight_fg_color: Color,
    // Editor space's highlighted line's background (highlight) color
    pub editor_highlight_bg_color: Color,
    // Selected tab's foreground (text) color
    pub tab_fg: Color,
    // Selected tab's backgound (highlight) color
    pub tab_bg: Color,
}

impl Theme {
    // Dark theme based on the existing terminal theme
    pub fn dark_terminal() -> Self {
        Theme {
            app_fg: Color::White,
            app_bg: Color::Black,
            editor_highlight_fg_color: Color::White,
            editor_highlight_bg_color: Color::DarkGray,
            tab_fg: Color::White,
            tab_bg: Color::Blue,
        }
    }

	// Light theme based on the existing terminal theme
	pub fn light_terminal() -> Self {
		Theme {
			app_fg: Color::Black,
			app_bg: Color::White,
			editor_highlight_fg_color: Color::Black,
			editor_highlight_bg_color: Color::Gray,
			tab_fg: Color::Black,
			tab_bg: Color::LightBlue,
		}
	}
}
