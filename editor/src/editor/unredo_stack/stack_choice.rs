// Choice of which stack within the UnRedoStack to use
#[allow(unused)]
pub enum StackChoice {
	Undo,
	Redo,
}

impl PartialEq for StackChoice {
	// Check that two choice values are the same
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(Self::Undo, Self::Undo) | (Self::Redo, Self::Redo)
		)
	}
}
