use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;
use std::collections::VecDeque;

fn main() {
	let mut queue = VecDeque::new();
	queue.push_back(0);
	queue.push_back(1);
	queue.push_back(2);

	println!("{:?}", queue.pop_front());
	queue.push_back(3);

	println!("{:?}", queue.pop_front());
}
