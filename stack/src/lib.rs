use std::collections::LinkedList;

// Simple stack data structure using a doubly linked list
#[derive(Default, Debug)]
pub struct Stack<T> {
	// Doubly linked list containing all items on the stack
	items: LinkedList<T>,
}

impl<T> Stack<T> {
	// Construct a new Stack
	pub fn new() -> Self {
		// Return a newly created stack
		Stack {
			items: LinkedList::new(),
		}
	}

	// Push a new item to the top of the stack
	pub fn push(&mut self, item: T) {
		/* Push the item to the back of the linked list.
		(It's doubly linked so push_back should have O(1) time) */
		self.items.push_back(item);
	}

	// Remove and return the top item on the stack
	pub fn pop(&mut self) -> Option<T> {
		// Remove the last element of the list (Should be O(1) time)
		self.items.pop_back()
	}

	// Remove all items from the stack
	pub fn clear(&mut self) {
		// Set the items list to a new list (making the old one go out of scope)
		self.items = LinkedList::new();
	}

	// Return a reference to the top item in the Stack
	pub fn top(&self) -> Option<&T> {
		self.items.front()
	}

	// Check if the stack is empty
	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	// Only used for debugging
	pub fn len(&self) -> usize {
		self.items.len()
	}
}

#[cfg(test)]
mod tests {
	use crate::Stack;

	#[test]
	// Test pushing to and popping from the stack
	fn push_pop_stack() {
		let mut stack = Stack::new();

		// Push 64 items to the stack
		for i in 0..64 {
			stack.push(i);
		}

		// Pop all items from the stack and collect them in a vector
		let mut actual = Vec::new();
		for _i in 0..64 {
			actual.push(stack.pop().unwrap());
		}

		// Check that the stack is empty
		assert_eq!(stack.pop(), None);

		// Check that the items were popped correctly
		let expected = (0..64).rev().collect::<Vec<i32>>();
		assert_eq!(actual, expected);
	}

	#[test]
	// Test clearing a populated stack
	fn clear_stack() {
		let mut stack = Stack::default();

		// Push 128 arbitrary strings
		for i in 0..128 {
			stack.push(format!("item: {}", i));
		}
		// Clear the stack
		stack.clear();
		// Check that the stack is empty
		assert_eq!(stack.pop(), None);
	}
}
