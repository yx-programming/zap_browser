use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
	pub children: Vec<Node>,
	node_type: NodeType,
}

pub enum NodeType {
	Text(String),
	Element(ElementData),
}

pub struct ElementData {
	name: String,
	attributes: AttrMap,
}

pub fn text(data: String) -> Node {
	Node {
		children: vec![],
		node_type: NodeType::Text(data),
	}
}

pub fn elem(name: String, attr: AttrMap, children: Vec<Node>) -> Node {
	Node {
		children,
		node_type: NodeType::Element(ElementData {
			name,
			attributes: attr,
		}),
	}
}
