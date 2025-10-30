pub struct Edge {
	from:usize,
	to:usize,
	cost:usize
}

impl Edge {
	pub fn new(from:usize,to:usize,cost:usize)->Self{
		Self{
			from,
			to,
			cost
		}
	}
	pub fn from(&self)->usize{
		self.from
	}
	
	pub fn to(&self)->usize{
		self.to
	}
	
	pub fn cost(&self)->usize{
		self.cost
	}
}