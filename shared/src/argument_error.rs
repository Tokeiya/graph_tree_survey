use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgumentError {
	#[error("Invalid argument: {0} {1}")]
	InvalidArgument(String, Option<String>),
}
