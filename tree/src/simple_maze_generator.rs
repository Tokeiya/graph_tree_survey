use crate::directional_gen::DirectionalGenerator;
use crate::simple_tree::Tree;
use rand::Rng;
use rand::seq::index::sample;
use rand_distr::Distribution;
use shared::argument_error::ArgumentError;
use std::collections::VecDeque;

pub struct SimpleMazeGenerator<T> {
	rnd: T,
	boost_threshold: usize,
	threshold: usize,
}

impl<T> SimpleMazeGenerator<T> {
	pub fn boost_threshold(&self) -> usize {
		self.boost_threshold
	}

	pub fn threshold(&self) -> usize {
		self.threshold
	}
}

impl<T: Rng> SimpleMazeGenerator<T> {
	pub fn try_new(
		rng: T,
		boost_threshold: usize,
		threshold: usize,
	) -> Result<Self, ArgumentError> {
		if boost_threshold > threshold {
			Err(ArgumentError::InvalidArgument(
				"boost_threshold must be less than threshold".to_string(),
				None,
			))
		} else {
			Ok(Self {
				rnd: rng,
				boost_threshold,
				threshold,
			})
		}
	}

	pub fn generate(&mut self) -> Tree {
		let mut tree = Tree::new();
		let mut cnt = 0usize;

		let dist = rand_distr::Uniform::new(0f64, 5.0f64).unwrap();

		let mut dir = DirectionalGenerator::new(&mut self.rnd);
		let mut queue = VecDeque::<usize>::new();

		queue.push_back(tree.add_node());

		loop {
			if cnt >= self.threshold {
				break;
			}

			if let Some(parent) = queue.pop_front() {
				if self.boost_threshold > cnt {
					let direction = dir.generate_with_offset(1, &mut self.rnd);
					for _ in 0..direction {
						let child = tree.add_node();
						queue.push_back(child);

						let a = dist.sample(&mut self.rnd);
						tree.add_edge(parent, child, dist.sample(&mut self.rnd))
					}

					cnt += direction as usize;
				}
			} else {
				break;
			}
		}

		tree
	}
}
