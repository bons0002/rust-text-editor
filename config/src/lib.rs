pub mod config {
	use std::default::Default;

	use crossterm::cursor::SetCursorStyle;

	// Contains color settings
	mod theme;

	// Contains user configuration for the app
	pub struct Config {
		// The cursor style for the editor
		pub cursor_style: SetCursorStyle,
		// The number of spaces used to represent a tab character
		pub tab_width: usize,
		// The color theme of the editor
		pub theme: theme::Theme,
	}

	impl Default for Config {
		// Create a new default config
		fn default() -> Self {
			Config {
				// Use the terminal's default cursor
				cursor_style: SetCursorStyle::DefaultUserShape,
				// Set the number of spaces for a tab to 4
				tab_width: 4,
				// Set the theme as a default dark theme based on the terminal theme
				theme: theme::Theme::light_terminal(),
			}
		}
	}
}
