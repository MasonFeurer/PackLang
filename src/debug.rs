use std::fmt::{Debug, Formatter};
use crate::compiler::parser::{Datapack, FunArgs, Function, Inline, InlineArg, Scoreboard};
use crate::compiler::tokens::{Group, Token};

pub mod bash_tools;
pub mod errors;

impl<T: NewDebugTree> NewDebugTree for Option<T> {
	fn debug_tree(&self) -> DebugTree {
		match self {
			Self::None => DebugTree::new("None"),
			Self::Some(t) => t.debug_tree(),
		}
	}
}

impl NewDebugTree for Datapack {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("Datapack")
			.node_debug("name", &self.name)
			.list("functions", &self.functions)
			.list("scoreboards", &self.scoreboards)
			.list("inlines", &self.inlines)
	}
}
impl NewDebugTree for Function {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("Function")
			.node_debug("name", &self.name)
			.node("args", &self.args)
			.node_debug("at", &self.at)
			.node("block", &self.block)
	}
}
impl NewDebugTree for Scoreboard {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("Scoreboard")
			.node_debug("name", &self.name)
			.node_debug("objective", &self.objective)
	}
}
impl NewDebugTree for Inline {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("Inline")
			.list("args", &self.args)
			.node("block", &self.block)
	}
}
impl NewDebugTree for InlineArg {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("InlineArg")
			.node_debug("name", &self.name)
			.node_debug("type", &self.ty)
	}
}
impl NewDebugTree for FunArgs {
	fn debug_tree(&self) -> DebugTree {
		DebugTree::new("FunArgs")
			.list_debug("input", &self.input)
			.list_debug("output", &self.output)
			.node_debug("objective", &self.objective)
	}
}
impl NewDebugTree for Group {
	fn debug_tree(&self) -> DebugTree {
		let value = format!("Group<{:?}>", self.delimiter);
		DebugTree::new(value.as_str())
			.list("tokens", &self.tokens)
	}
}
impl NewDebugTree for Token {
	fn debug_tree(&self) -> DebugTree {
		if let Token::Group(group) = self {
			group.debug_tree()
		} else {
			DebugTree::new(format!("{:?}", self).as_str())
		}
	}
}

pub struct DebugTree {
	field: Option<String>,
	value: String,
	sub: Vec<DebugTree>,
}
impl DebugTree {
	pub fn new(value:&str) -> Self {
		DebugTree {
			field: None,
			value: value.to_owned(),
			sub: Vec::new(),
		}
	}
	pub fn field(field:&str, value:&str) -> Self {
		DebugTree {
			field: Some(field.to_owned()),
			value: value.to_owned(),
			sub: Vec::new(),
		}
	}

	pub fn node<T>(mut self, field:&str, item:&T) -> Self
	where T: NewDebugTree {
		let mut node = item.debug_tree();
		node.field = Some(field.to_owned());
		self.sub.push(node);
		self
	}
	pub fn node_debug<T>(mut self, field:&str, item:&T) -> Self
		where T: Debug {
		self.sub.push(Self::field(field, format!(
			"{:?}", item
		).as_str()));
		self
	}
	pub fn list<T>(mut self, field:&str, list:&Vec<T>) -> Self
	where T: NewDebugTree {
		let mut node = Self::field(field, "List");
		for item in list {
			node.sub.push(item.debug_tree());
		}
		self.sub.push(node);
		self
	}
	pub fn list_debug<T>(mut self, field:&str, list:&Vec<T>) -> Self
		where T: Debug {
		let mut node = Self::field(field, "List");
		for item in list {
			node.sub.push(Self::new(
				format!("{:?}", item).as_str()
			));
		}
		self.sub.push(node);
		self
	}

	fn fmt_indent(&self, f:&mut Formatter<'_>, indent:usize) -> std::fmt::Result {
		const INDENT:&'static str = " ";
		f.write_str(INDENT.repeat(indent).as_str())?;
		if let Some(field) = &self.field {
			f.write_str(field.as_str())?;
			f.write_str(": ")?;
		}
		f.write_str(self.value.as_str())?;
		f.write_str("\n")?;
		for sub in &self.sub {
			sub.fmt_indent(f, indent+1)?;
		}
		Ok(())
	}
}
impl Debug for DebugTree {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.fmt_indent(f, 0)
	}
}
pub enum DebugValue {
	Tree(Box<DebugTree>),
	Item(String),
}
pub trait NewDebugTree {
	fn debug_tree(&self) -> DebugTree;
}
