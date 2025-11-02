use thiserror::Error;

fn disp(arg_name: &str, detail: &Option<String>) -> String {
	if let Some(detail) = detail {
		format!("{} InvalidArgument:{}", arg_name, detail)
	} else {
		format!("{} InvalidArgument", arg_name)
	}
}

#[derive(Error, Debug)]
pub enum ArgumentError {
	#[error("Invalid argument: {0}{detail}", detail = disp(.0, .1))]
	InvalidArgument(String, Option<String>),

	#[error("Argument out of range: {0}")]
	ArgumentOutOfRange(String),
}
