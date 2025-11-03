use mockall::mock;
use rand::Rng;
use rand_core::RngCore;
use shared::argument_error::ArgumentError;

mock! {
		 Rnd{}

		impl RngCore for Rnd {
			fn next_u32(&mut self) -> u32;
			fn next_u64(&mut self) -> u64;
			fn fill_bytes(&mut self, dst: &mut [u8]);
		}
}
fn main() {
	let mut i = -1i64 as u64;

	let mut mock = MockRnd::new();
	mock.expect_next_u64().returning(move || {
		i = i.wrapping_add(1);
		i
	});

	foo(&mut mock);
}

fn foo<T: Rng>(r: &mut T) {
	println!("{}", r.next_u64());
	println!("{}", r.next_u64());
	println!("{}", r.next_u64());
	println!("{}", r.next_u64());
	println!("{}", r.next_u64());
}
