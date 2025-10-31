use std::fmt::Display;
use std::rc::{Rc, Weak};

struct Node<T: Display> {
	value: T,
	parent: Option<Weak<Node<T>>>,
	children: Vec<Rc<Node<T>>>,
}

impl<T: Display> Node<T> {
	fn new(value: T, parent: Option<Weak<Node<T>>>) -> Node<T> {
		Node {
			value,
			parent,
			children: Vec::new(),
		}
	}

	fn add_child(&mut self, child: Rc<Node<T>>) -> usize {
		self.children.push(child);
		self.children.len()
	}

	fn parent(&self) -> Option<Weak<Node<T>>> {
		self.parent.clone()
	}

	fn children(&self) -> &[Rc<Node<T>>] {
		&self.children
	}
}
