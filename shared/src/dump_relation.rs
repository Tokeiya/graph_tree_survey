use std::io::{Write,Result as IoResult};

pub trait DumpRelation {
	fn dump_relation<T:Write>(&self,writer:&mut T)-> IoResult<()>;
}