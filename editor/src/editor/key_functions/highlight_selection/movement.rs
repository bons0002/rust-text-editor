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
	pub fn take_movement(&self, editor: &mut EditorSpace) {
		match self {
			Self::Up => up_arrow(editor),
			Self::Down => down_arrow(editor),
			Self::Left => left_arrow(editor, true),
			Self::Right => right_arrow(editor, true),
			Self::Home => home_key(editor, true),
			Self::End => end_key(editor, true),
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
