use super::*;

// Keep track of which movement key is used
pub enum Movement {
	Up,
	Down,
	Left,
	Right,
	Home,
	End,
}

impl Movement {
	// Uses a movement key based on the value of the enum
	pub fn take_movement(&self, editor: &mut EditorSpace, config: &Config) {
		match self {
			Self::Up => up_arrow(editor, config),
			Self::Down => down_arrow(editor, config),
			Self::Left => left_arrow(editor, config),
			Self::Right => right_arrow(editor, config),
			Self::Home => home_key(editor),
			Self::End => end_key(editor, config),
		};
	}
}

// Implement equality for the Movement enum
impl PartialEq for Movement {
	// Check whether the two enums are the same value
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(Self::Up, Self::Up)
				| (Self::Down, Self::Down)
				| (Self::Left, Self::Left)
				| (Self::Right, Self::Right)
				| (Self::Home, Self::Home)
				| (Self::End, Self::End)
		)
	}
}
