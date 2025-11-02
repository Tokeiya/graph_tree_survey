use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgumentError {
	#[error("Invalid argument: {0}{detail}",
        detail = .1.as_deref()
                    .map(|s| format!(" ({s})"))  // Someなら " (補足)" を作る
                    .unwrap_or_default()         // Noneなら空文字
	)]
	InvalidArgument(String, Option<String>),

	#[error("Argument out of range: {0}")]
	ArgumentOutOfRange(String),
}
