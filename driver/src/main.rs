use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;

fn main() {
	let mut accum = 0u64;

	for i in 0..32u64 {
		accum |= i & 3;
		accum <<= 2;
	}

	println!("{accum}");

	println!("accum: {}", accum);
	for i in 0..32 {
		let c = accum & 0x03;
		accum >>= 2;
		println!("{}:{} {}", i, c, accum);
	}
}
