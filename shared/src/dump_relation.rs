use std::io::{Result as IoResult, Write};

pub trait DumpRelation {
	fn dump_relation<T: Write>(&self, writer: &mut T) -> IoResult<()>;
}
