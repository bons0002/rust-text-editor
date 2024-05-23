pub mod config {

    use crossterm::cursor::SetCursorStyle;

    // Contains color settings
    mod theme;

    // Contains user configuration for the app
    pub struct Config {
        pub cursor_style: SetCursorStyle,
        pub tab_width: usize,
        pub theme: theme::Theme,
    }

    impl Config {
        // Create a new config
        pub fn default() -> Self {
            Config {
                cursor_style: SetCursorStyle::DefaultUserShape,
                tab_width: 4,
                theme: theme::Theme::dark_mode(),
            }
        }
    }
}