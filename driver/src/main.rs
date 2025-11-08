use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;
use std::collections::VecDeque;
fn main() {
	let rng = rand::rng();
	let mut generator =
		tree::simple_maze_generator::SimpleMazeGenerator::try_new(rng, 3, 100).unwrap();

	let maze = generator.generate();
}
