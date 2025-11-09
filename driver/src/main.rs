use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;
use std::collections::VecDeque;
fn main() {
	println!("{}", std::env::current_dir().unwrap().display());

	let rng = rand::rng();
	let mut generator =
		tree::simple_maze_generator::SimpleMazeGenerator::try_new(rng, 3, 30).unwrap();

	let maze = generator.generate();

	let mut file = std::fs::File::create("maze.mmd").unwrap();

	tree::visualize::write_mermaid(&maze, &mut file);

	println!("node:{}", maze.length())
}
