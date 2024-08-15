use ratatui::style::Color;

// Contains the colors of the editor
#[derive(Clone)]
pub struct Theme {
	// Entire app's foreground (text) color
	pub app_fg: Color,
	// Entire app's background color
	pub app_bg: Color,
	// Editor space's highlighted line's foreground (text) color
	pub line_highlight_fg_color: Color,
	// Editor space's highlighted line's background (highlight) color
	pub line_highlight_bg_color: Color,
	// Color of the highlighted selection of text
	pub selection_highlight: Color,
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
			line_highlight_fg_color: Color::White,
			line_highlight_bg_color: Color::DarkGray,
			selection_highlight: Color::Blue,
			tab_fg: Color::White,
			tab_bg: Color::Blue,
		}
	}

	// Light theme based on the existing terminal theme
	pub fn light_terminal() -> Self {
		Theme {
			app_fg: Color::Black,
			app_bg: Color::White,
			line_highlight_fg_color: Color::Black,
			line_highlight_bg_color: Color::Gray,
			selection_highlight: Color::LightBlue,
			tab_fg: Color::Black,
			tab_bg: Color::LightBlue,
		}
	}
}
