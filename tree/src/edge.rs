use crate::node::Node;

pub struct Edge {
	from: usize,
	to: usize,
	cost: f64,
}

impl Edge {
	pub fn new(from: usize, to: usize, cost: f64) -> Edge {
		Edge { from, to, cost }
	}

	pub fn from(&self) -> usize {
		self.from
	}
	pub fn to(&self) -> usize {
		self.to
	}
	pub fn cost(&self) -> f64 {
		self.cost
	}
}
