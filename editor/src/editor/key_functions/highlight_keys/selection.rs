// Structure that keeps track of the highlighted selection of text
#[derive(Clone, Debug)]
pub struct Selection {
	// The start point of the selection
	pub start: [usize; 2],
	// The endpoint of the selection
	pub end: [usize; 2],
	// Flag to track if selection is empty or not
	pub is_empty: bool,
	// Store the original position of the cursor before highlighting
	pub original_cursor_position: (usize, usize),
	// Store the original position in the text before highlighting
	pub original_text_position: (usize, usize),
	// Store the original scroll offset of the text
	pub original_scroll_offset: usize,
}

impl Selection {
	// Create a new Selection struct
	pub fn new() -> Self {
		Selection {
			start: [0, 0],
			end: [0, 0],
			is_empty: true,
			original_cursor_position: (0, 0),
			original_text_position: (0, 0),
			original_scroll_offset: 0,
		}
	}
}

impl PartialEq for Selection {
	// Check that two selections are equal
	fn eq(&self, other: &Self) -> bool {
		self.start == other.start
			&& self.end == other.end
			&& self.is_empty == other.is_empty
			&& self.original_cursor_position == other.original_cursor_position
			&& self.original_text_position == other.original_text_position
			&& self.original_scroll_offset == other.original_scroll_offset
	}
}
