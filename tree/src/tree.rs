use std::collections::HashMap;
use crate::edge::Edge;
use crate::node::Node;
use crate::tree_error::TreeError;

pub struct Tree{
	node:HashMap<usize,Node>,
	edge:Vec<Edge>
}

impl Tree{
	pub fn new()->Self{
		Self{
			node:HashMap::new(),
			edge:Vec::new()
		}
	}
	pub fn add_node(&mut self,node:Node){
		self.node.insert(node.id(),node);
	}
	pub fn make_relation(&mut self,parent:usize,child:usize,cost:usize)->Result<(),TreeError>{
		if !self.node.contains_key(&parent){
			return Err(TreeError::NodeNotFound(parent));
		}
		if !self.node.contains_key(&child){
			return Err(TreeError::NodeNotFound(child));
		}
		self.edge.push(Edge::new(parent,child,cost));
		Ok(())
	}
}