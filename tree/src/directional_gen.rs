use rand::Rng;

pub struct DirectionalGenerator<T> {
	rng: T,
	cache: u64,
	cnt: usize,
}

impl<T: Rng> DirectionalGenerator<T> {
	pub fn new(rng: T) -> Self {
		todo!()
	}
}
