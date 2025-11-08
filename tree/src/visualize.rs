use crate::simple_tree::Tree;
use std::fs::write;
use std::io::Write;

pub fn write_mermaid(root: &Tree, writer: &mut impl Write) {
	writer.write_all(b"graph TD\n").unwrap();

	let data = root.edges();

	for node in 0..data.len() {
		for edge in data[node].iter() {
			if node < edge.to() {
				writer
					.write_fmt(format_args!(
						"{}[{}] -- {:.2} --- {}[{}]\n",
						node,
						node,
						edge.cost(),
						edge.to(),
						edge.to()
					))
					.unwrap()
			}
		}
	}
}
