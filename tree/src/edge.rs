use crate::node::Node;

pub struct Edge {
	to: usize,
	cost: f64,
}

impl Edge {
	pub fn new(to: usize, cost: f64) -> Edge {
		Edge { to, cost }
	}

	pub fn to(&self) -> usize {
		self.to
	}
	pub fn cost(&self) -> f64 {
		self.cost
	}
}
