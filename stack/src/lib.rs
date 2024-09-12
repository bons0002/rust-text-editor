use std::collections::LinkedList;

// Simple stack data structure using a doubly linked list
#[derive(Default)]
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
}
