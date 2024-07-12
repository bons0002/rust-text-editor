use super::*;

// Keep track of which movement key is used
pub enum Movement {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	HOME,
	END,
}

impl Movement {
	// Uses a movement key based on the value of the enum
	pub fn take_movement(&self, editor: &mut EditorSpace, config: &Config) {
		match self {
			Self::UP => up_arrow(editor, config),
			Self::DOWN => down_arrow(editor, config),
			Self::LEFT => left_arrow(editor, config),
			Self::RIGHT => right_arrow(editor, config),
			Self::HOME => home_key(editor),
			Self::END => end_key(editor, config),
		};
	}
}

// Implement equality for the Movement enum
impl PartialEq for Movement {
	// Check whether the two enums are the same value
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UP, Self::UP) => true,
            (Self::DOWN, Self::DOWN) => true,
			(Self::LEFT, Self::LEFT) => true,
			(Self::RIGHT, Self::RIGHT) => true,
			(Self::HOME, Self::HOME) => true,
			(Self::END, Self::END) => true,
            _ => false,
        }
    }
}