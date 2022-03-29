#![feature(let_else)]
#![allow(dead_code)]

use crate::compiler::files;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::debug::errors::CompileErr;
use crate::debug::NewDebugTree;

pub mod debug;
pub mod compiler;

fn main() -> Result<(), CompileErr> {
	files::init();
	files::load_file("datapack.mccs");

	let mut lexer = Lexer::new(files::ref_file(0));
	let tokens = lexer.lex();

	// for t in &tokens {
	// 	println!("{:?}", t);
	// }

	let mut parser = Parser::new(tokens);
	let datapack = parser.parse();
	println!("{:?}", datapack.debug_tree());

	Ok(())
}
