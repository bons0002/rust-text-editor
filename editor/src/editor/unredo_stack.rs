use super::{key_functions::highlight_selection::Selection, Blocks};
use stack::Stack;

pub mod stack_choice;
use stack_choice::StackChoice;

// The number of characters that need to be entered in order to add a new undo state
const UNDO_PERIOD: usize = 20;

/* Undo or Redo state. Formatted as (stored position, text position,
cursor position, scroll offset, Blocks, Selection) */
pub type UnRedoState = (usize, usize, [usize; 2], usize, Blocks, Selection);

// Controls both the undo and redo stack simultaneously
pub struct UnRedoStack {
	// The undo stack
	undo_stack: Stack<UnRedoState>,
	// The redo stack
	redo_stack: Stack<UnRedoState>,
	// The counter towards progress of updating the stack
	counter: usize,
}

impl UnRedoStack {
	// Create a new undo/redo stack
	pub fn new() -> Self {
		Self {
			undo_stack: Stack::new(),
			redo_stack: Stack::new(),
			counter: 0,
		}
	}

	// Automatically update the undo and redo stacks, and the cached state
	pub fn auto_update(&mut self, state: UnRedoState, force: bool) {
		self.counter += 1;
		// If the stack is empty, push to it
		if self.undo_stack.is_empty() {
			self.counter = 0;
			self.undo_stack.push(state);
		/* If the counter has reached the required value and the top of the stack is different from
		the current cached Blocks, update both stacks */
		} else if self.counter >= UNDO_PERIOD || force {
			// Reset the counter
			self.counter = 0;
			// Push the cached state to the undo stack
			self.undo_stack.push(state);
			// Clear the redo stack
			self.redo_stack.clear();
		}
	}

	// Pop the top undo state and return it. Also push to the redo stack
	pub fn undo(&mut self, editor_state: UnRedoState) -> UnRedoState {
		match self.undo_stack.pop() {
			// If the stack wasn't empty
			Some(state) => {
				// Push the passed editor_state (editor's current state) to the redo stack
				self.redo_stack.push(editor_state);
				// Return the popped state to set the EditorSpace's current state
				state.clone()
			}
			// If the stack was empty
			None => {
				// Return the EditorSpace's current state so it doesn't change
				editor_state
			}
		}
	}

	// Used for debugging
	#[allow(unused)]
	pub fn len(&self, stack: StackChoice) -> usize {
		// Return the length of the appropriate stack
		match stack {
			StackChoice::Undo => self.undo_stack.len(),
			StackChoice::Redo => self.redo_stack.len(),
		}
	}
}
