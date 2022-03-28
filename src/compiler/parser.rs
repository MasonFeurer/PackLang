use crate::compiler::token_stream::TokenStream;
use crate::compiler::tokens::{Delimiter, Group, Ident, Sep, SrcScope, Symbol, Token};
use crate::debug::errors::{Help, CompileResult, ErrorInfo, error};

pub struct Parser {
	pub tokens: TokenStream,
}
impl Parser {
	pub fn new(tokens:Vec<Token>) -> Self {
		Parser {
			tokens: TokenStream::new(tokens),
		}
	}
	pub fn parse(&mut self) -> Datapack {
		self.tokens.next().expect_ident_w(
			"datapack",
			"expected datapack declaration",
			Some(Help {
				msg: "declare datapack at top of file".to_owned(),
				source: "datapack some_pack;".to_owned(),
				line: 0
			})
		).fatal();
		let name = self.tokens.next().expect_ident(
			"expected datapack name",
			Some(Help {
				msg: "declare datapack at top of file".to_owned(),
				source: "datapack some_pack;".to_owned(),
				line: 0
			})
		).fatal();

		self.tokens.next().expect_sep_w(
			';',
			"expected semi-colon to finish statement",
			None,
		).fatal();

		let mut datapack = Datapack::empty(name);

		let mut next = self.tokens.next().clone();
		while !next.is_end() {
			let ident = next.expect_ident(
				"expected item declaration",
				None,
			).fatal();

			match ident.value.as_str() {
				"function" => {
					let function = self.parse_function(next.scope().start);
					datapack.functions.push(function);
				}
				_ => error(&ident.scope, ErrorInfo {
					cause: ident.value.as_str(),
					more: "expected top-level item declaration",
					help: None,
				}).fatal()
			}
			next = self.tokens.next().clone();
		}
		datapack
	}

	fn parse_function(&mut self, start:usize) -> Function {
		let name = self.tokens.next().expect_ident(
			"expected identifier for function name",
			None,
		).fatal();

		let mut args:Option<FunctionArgs> = None;
		let mut at:Option<At> = None;

		let mut next = self.tokens.next().clone();
		// check for function args
		if let Token::Group(group) = &next {
			if group.delimiter == Delimiter::Parentheses {
				// parse function args
				let mut tokens = TokenStream::new(group.clone().tokens);
				let mut input = Vec::new();
				let mut output = Vec::new();
				let mut arrow = None;
				let mut colon = None;
				let mut objective = None;

				loop {
					let next = tokens.next();
					if next.is_end() { break }
					if let Some(ident) = next.as_ident() {
						if colon.is_some() {
							assert!(objective.is_none()); // TODO
							objective = Some(ident.clone());
						}
						else if arrow.is_some() {
							output.push(ident.clone());
						} else { input.push(ident.clone()); }
					}
					if let Some(symbol) = next.as_symbol_w("->") {
						arrow = Some(symbol.clone());
					}
					if let Some(sep) = next.as_sep_w(':') {
						colon = Some(sep.clone());
					}
				}

				args = Some(FunctionArgs {
					group: group.clone(),
					input, arrow, output, colon, objective
				});
				next = self.tokens.next().clone();
			}
		}
		// check for at mod
		if let Some(ident) = next.as_ident_w("at") {
			// parse at mod
			let loc_token = self.tokens.next();
			let loc = expect_loc(
				loc_token,
				"expected `player`, `players`, `entity`, `entities`, or `any`",
				None
			).fatal();
			at = Some(At {
				scope: ident.scope.join(loc_token.scope()),
				at: ident.clone(),
				loc,
				loc_ident: loc_token.as_ident().unwrap().clone()
			});
			next = self.tokens.next().clone();
		}
		// expect block
		let block = next.expect_group_w(
			Delimiter::CurlyBrackets,
			"expected (), `at`, or {}",
			None
		).fatal();
		let statements = self.parse_function_block(&block);

		Function {
			scope: SrcScope {
				start,
				end: block.scope.end,
				file: block.scope.file.clone(),
			},
			block, name, args, at, statements,
		}
	}

	fn parse_function_block(&mut self, block:&Group) -> Vec<Statement> {
		let mut statements:Vec<Statement> = Vec::new();
		let mut tokens = TokenStream::new(block.tokens.clone());

		let mut next = tokens.next().clone();
		while !next.is_end() {
			// TODO
		}

		statements
	}
}

#[derive(Debug)]
pub struct Datapack {
	pub name: Ident,
	pub functions: Vec<Function>,
}

impl Datapack {
	pub fn empty(name:Ident) -> Self {
		Datapack {
			name,
			functions: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Function {
	pub scope: SrcScope,
	pub block: Group,
	pub name: Ident,
	pub args: Option<FunctionArgs>,
	pub at: Option<At>,
	pub statements: Vec<Statement>,
}
#[derive(Debug, Clone)]
pub struct FunctionArgs {
	pub group: Group,
	pub input: Vec<Ident>,
	pub arrow: Option<Symbol>,
	pub output: Vec<Ident>,
	pub colon: Option<Sep>,
	pub objective: Option<Ident>,
}
#[derive(Debug, Clone)]
pub struct At {
	pub scope: SrcScope,
	pub at: Ident,
	pub loc: Loc,
	pub loc_ident: Ident,
}
#[derive(Debug, Clone)]
pub enum Loc {
	Player,
	Players,
	Entity,
	Entities,
	Any,
}
#[derive(Debug, Clone)]
pub enum Statement {
	Unsafe(Unsafe),
	FunctionCall(FunctionCall),
	If(If),
}
#[derive(Debug, Clone)]
pub struct Unsafe {
	pub commands: Vec<UnsafeCmdCall>
}
#[derive(Debug, Clone)]
pub struct UnsafeCmdCall {
	pub tokens: Vec<Token>,
}
#[derive(Debug, Clone)]
pub struct FunctionCall {

}
#[derive(Debug, Clone)]
pub struct If {
	condition: ConditionalCall,
	statements: Vec<Statement>,
}
#[derive(Debug, Clone)]
pub struct ConditionalCall {
	name: Ident,

}


pub fn expect_loc(token:&Token, more:&str, help:Option<Help>) -> CompileResult<Loc> {
	let err = CompileResult::Err(error(
	token.scope(), ErrorInfo {
		cause: "invalid location",
		more, help
	}));
	if let Token::Ident(ident) = token {
		match ident.value.as_str() {
			"player" => CompileResult::Ok(Loc::Player),
			"players" => CompileResult::Ok(Loc::Players),
			"entity" => CompileResult::Ok(Loc::Entity),
			"entities" => CompileResult::Ok(Loc::Entities),
			"any" => CompileResult::Ok(Loc::Any),
			_ => err,
		}
	}
	else { err }
}


fn display_tokens(tokens:&Vec<Token>, sep:&str) -> String {
	let mut string = String::new();
	for token in tokens {
		string.push_str(token.display().as_str());
		string.push_str(sep);
	}
	string
}
fn sep_tokens(tokens:&Vec<Token>, sep:char) -> Vec<Vec<Token>> {
	let mut parts:Vec<Vec<Token>> = Vec::new();
	let mut current:Vec<Token> = Vec::new();

	for token in tokens {
		if let Token::Sep(s) = token {
			if s.value == sep && !current.is_empty() {
				parts.push(current.clone());
				current.clear();
				continue;
			}
		}
		current.push(token.clone());
	}
	if !current.is_empty() { parts.push(current) }
	parts
}
