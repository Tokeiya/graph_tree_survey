use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;

fn main() {
	let mut accum =0;
	
	for i in 0..16u64 {
		accum|=
	}

	for i in 0..16 {
		let c = accum & 0x03;
		accum >>= 2;
		println!("{}:{} {}", i, c, accum);
	}
}
