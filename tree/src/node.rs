#[derive(PartialEq, Eq, Hash)]
pub struct Node(usize);

impl Node {
	pub fn value(&self) -> usize {
		self.0
	}
}

impl From<usize> for Node {
	fn from(value: usize) -> Self {
		Node(value)
	}
}
