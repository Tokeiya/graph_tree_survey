use std::rc::{Rc,Weak};


pub struct Node {
	id:usize,
	parent:Option<Weak<Node>>,
	children:Vec<Rc<Node>>
}

impl Node{
	pub fn new(id:usize,parent:Option<Rc<Node>>)->Self{
		Self{
			id,
			parent:parent.map(|p|Rc::downgrade(&p)),
			children:Vec::new()
		}
	}
	
	pub fn id(&self)->usize{
		self.id
	}
	pub fn parent(&self)->Option<Weak<Node>>{
		self.parent.clone()
	}
	pub fn children(&self)->&[Rc<Node>]{
		&self.children
	}
	pub fn add_child(&mut self,child:Rc<Node>){
		self.children.push(child);
	}
}


