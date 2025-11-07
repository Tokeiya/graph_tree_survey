use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;

fn main() {
	let mut accum = 0u64;

	for i in 0..32 {
		accum <<= 2;
		accum |= i & 3;
	}

	println!("{:x}", accum);

	for i in 0..32 {
		println!("{}:{}", i, accum & 3);
		accum >>= 2;
	}
}
