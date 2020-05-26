use super::*;

use std::collections::HashMap;

/// A named module holding multiple items.
///
/// An AST node that takes a name and zero or more items.
#[derive(Clone, Debug)]
pub struct Module(pub HashMap<String, Item>);

impl From<ast::Module> for Module {
	fn from(m: ast::Module) -> Self {
		let mut map = HashMap::new();
		for item in m.items {
			match item {
				ast::Item::Proc(p) => map.insert(p.name.clone(), Item::Proc(p.into())),
				ast::Item::Mod(m) => map.insert(m.name.clone(), Item::Mod(m.into())),
				//ast::Item::Fn(f) => map.insert(f.name, Item::Fn(f.into())),
				_ => unimplemented!()
			};
		}
		Module(map)
	}
}

impl From<Vec<ast::Item>> for Module {
	fn from(items: Vec<ast::Item>) -> Self {
		let mut map = HashMap::new();
		for item in items {
			match item {
				ast::Item::Proc(p) => map.insert(p.name.clone(), Item::Proc(p.into())),
				ast::Item::Mod(m) => map.insert(m.name.clone(), Item::Mod(m.into())),
				//ast::Item::Fn(f) => map.insert(f.name, Item::Fn(f.into())),
				_ => unimplemented!()
			};
		}
		Module(map)
	}
}

impl Module {
	pub fn add_internal_procedure(&mut self, name: String, fore: Proc, back: Proc) {
		self.0.insert(name, Item::InternProc(fore, back));
	}
}