use super::simple_tree::Tree;
use rand::Rng;

const LIMIT: usize = 16usize;
const MASK: u64 = 0x03;
const SHIFT: u64 = 0x02;

pub struct DirectionalGenerator<'a, T> {
	rng: &'a mut T,
	cache: u64,
	cnt: usize,
}

impl<'a, T: Rng> DirectionalGenerator<'a, T> {
	pub fn new(rng: &'a mut T) -> Self {
		let c = rng.next_u64();
		Self {
			rng,
			cache: c,
			cnt: 0,
		}
	}

	pub fn generate_with_offset(&mut self, offset: u64) -> u64 {
		self.generate() + offset
	}

	fn fill_cache(&mut self) {
		let a = self.rng.next_u64();

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

	fn gen_mock() -> MockRnd {
		let mut mock = MockRnd::new();

		mock.expect_next_u64().return_const(0x1b1b1b1b1b1b1b1bu64);

		mock.expect_next_u32().never();
		mock.expect_fill_bytes().never();

		mock
	}

	#[test]
	fn new_test() {
		let mut mock = gen_mock();
		let fixture = DirectionalGenerator::new(&mut mock);

		assert_eq!(fixture.cnt, 0);
		assert_eq!(fixture.cache, 0x1b1b1b1b1b1b1b1bu64);
	}

	#[test]
	fn generate_test() {
		let mut mock = gen_mock();
		let mut fixture = DirectionalGenerator::new(&mut mock);

		let mut actual = 0u64;

		for i in 0..32 {
			actual |= fixture.generate() << (i * 2);
		}

		assert_eq!(actual, 0x1b1b1b1b1b1b1b1bu64);
	}

	#[test]
	fn offset_test() {
		let mut mock = gen_mock();
		let mut fixture = DirectionalGenerator::new(&mut mock);
		let expected = [4u64, 3, 2, 1];

		for i in 0..16 {
			let actual = fixture.generate_with_offset(1);
			assert_eq!(actual, expected[i & 3]);
		}
	}
}
