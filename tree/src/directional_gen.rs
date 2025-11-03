use super::simple_tree::Tree;
use rand::Rng;

const LIMIT: usize = 16usize;
const MASK: u64 = 0x03;
const SHIFT: u64 = 0x02;

pub struct DirectionalGenerator<T> {
	rng: T,
	cache: u64,
	cnt: usize,
}

impl<T: Rng> DirectionalGenerator<T> {
	pub fn new(rng: T) -> Self {
		todo!()
	}

	pub fn generate_with_offset(&mut self, offset: usize) -> usize {
		todo!()
	}

	fn fill_cache(&mut self) {
		self.cache = self.rng.next_u64();
		self.cnt = 0;
	}

	pub fn generate(&mut self) -> u64 {
		if self.cnt >= LIMIT {
			self.fill_cache();
		}

		let ret = self.cache & MASK;
		self.cache >>= SHIFT;
		self.cnt += 1;
		ret
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::*;
	use rand::RngCore;

	mock! {
		 Rnd{}

		impl RngCore for Rnd {
			fn next_u32(&mut self) -> u32;
			fn next_u64(&mut self) -> u64;
			fn fill_bytes(&mut self, dst: &mut [u8]);
		}
	}

	fn gen_mock() -> MockRnd {}
}
