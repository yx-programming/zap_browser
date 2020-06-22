pub mod dom;

fn main() {
	let mut root = dom::elem(String::from("html"), dom::AttrMap::new(), vec![]);
	let mut body = dom::elem(String::from("body"), dom::AttrMap::new(), vec![]);
	body.children.push(dom::text(String::from("Well I hope this works...")));
	root.children.push(body);
}
