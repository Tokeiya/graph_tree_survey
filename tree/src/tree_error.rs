use thiserror::Error;

#[derive(Error,Debug)]
pub enum TreeError{
	#[error("Node with id {0} already exists")]
	NodeExists(usize),
	#[error("Node with id {0} does not exist")]
	NodeNotFound(usize),
}