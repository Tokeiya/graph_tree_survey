use crate::edge::Edge;
use std::collections::HashMap;

pub struct Tree(Vec<Vec<Edge>>);

impl Tree {
	pub fn new() -> Tree {
		Tree(Vec::new())
	}

	pub fn add_node(&mut self) -> usize {
		self.0.push(Vec::new());
		self.0.len() - 1
	}

	pub fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
		self.0[from].push(Edge::new(to, cost));
		self.0[to].push(Edge::new(from, cost));
	}

	pub fn length(&self) -> usize {
		self.0.len()
	}

	pub fn edges(&self) -> &[Vec<Edge>] {
		&self.0
	}
}
