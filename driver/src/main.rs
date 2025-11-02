use shared::argument_error::ArgumentError;

fn main() {
	let err = ArgumentError::InvalidArgument("hoge".to_string(), Some("fuga".to_string()));

	println!("{:?}", err);
}
