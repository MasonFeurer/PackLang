use std::fmt::{Debug};
use crate::compiler::token_stream::{TokenIter, TokenList};
use crate::compiler::tokens::{Delimiter, Group, Ident, Sep, SrcScope, Symbol, Token};
use crate::debug::errors::{Help, CompileResult, ErrorInfo, error};

pub struct Parser {
	pub tokens: TokenIter,
}
impl Parser {
	pub fn new(tokens:Vec<Token>) -> Self {
		Parser {
			tokens: TokenIter::new(tokens),
		}
	}
	pub fn parse(&mut self) -> Datapack {
		self.tokens.next().expect_ident_w(
			"datapack",
			Some("datapack declaration"),
			Some(Help {
				msg: "declare datapack at top of file".to_owned(),
				source: "datapack some_pack;".to_owned(),
				line: 0
			})
		).fatal();
		let name = self.tokens.next().expect_ident(
			Some("datapack declaration"),
			Some(Help {
				msg: "declare datapack at top of file".to_owned(),
				source: "datapack some_pack;".to_owned(),
				line: 0
			})
		).fatal();

		self.tokens.next().expect_sep_w(
			';',
			Some("datapack declaration"),
			None,
		).fatal();

		let mut datapack = Datapack::empty(name);

		let mut next = self.tokens.next().clone();
		while !next.is_end() {
			let ident = next.expect_ident(
				Some("top-level"),
				None,
			).fatal();

			match ident.value.as_str() {
				"function" => {
					let function = self.parse_function(ident);
					datapack.functions.push(function);
				},
				"scoreboard" => {
					let scoreboard = self.parse_scoreboard(ident);
					datapack.scoreboards.push(scoreboard);
				},
				"inline" => {
					let inline = self.parse_inline(ident);
					datapack.inlines.push(inline);
				}
				_ => error(ident.scope, ErrorInfo {
					cause: ident.value.as_str(),
					pointer: "unknown item declaration",
					context: Some("top-level"),
					help: None,
				}).fatal()
			}
			next = self.tokens.next().clone();
		}
		datapack
	}

	pub fn parse_inline(&mut self, keyword:Ident) -> Inline {
		// get name
		let name = self.tokens.next().expect_ident(
			Some("inline declaration"), None,
		).fatal();

		// get args
		let group = self.tokens.next().expect_group_w(
			Delimiter::Parentheses,
			Some("inline declaration"), None,
		).fatal();
		// parse args
		let mut args = Vec::new();
		group.tokens.iter_tokens(|token, iter| {
			// get arg name
			let name = token.expect_ident(
				Some("inline argument declaration"), None,
			).fatal();

			// get colon
			let colon = iter.next().expect_sep_w(
				':',
				Some("inline argument declaration"), None
			).fatal();

			// get type
			let ty_ident = iter.next().expect_ident(
				Some("inline argument declaration"), None,
			).fatal();
			let ty = InlineArgType::parse(
				&ty_ident, Some("inline argument declaration"), None,
			).fatal();

			// consume separator
			let next = iter.next();
			let err = error(next.scope(), ErrorInfo {
				cause: "invalid token",
				pointer: format!(
					"expected End() or Sep(','), found {:#1?}", next
				).as_str(),
				context: Some("inline argument declaration"),
				help: None
			});
			let sep = match next {
				Token::End(_) => None,
				Token::Sep(sep) => Some(sep.clone()),
				_ => err.fatal()
			};

			// done
			args.push(InlineArg {
				name, colon: colon.scope, ty,
				ty_scope: ty_ident.scope, sep
			});
		});

		// get block
		let block = self.tokens.next().expect_group_w(
			Delimiter::CurlyBrackets,
			Some("inline declaration"), None
		).fatal();

		// done
		Inline {
			keyword,
			name,
			args,
			parens: group.scope,
			block
		}
	}

	pub fn parse_scoreboard(&mut self, keyword:Ident) -> Scoreboard {
		let objective = self.tokens.next().expect_ident(
			Some("scoreboard declaration"),
			None,
		).fatal();
		let name = self.tokens.next().expect_ident(
			Some("scoreboard declaration"),
			None,
		).fatal();
		let semi_colon = self.tokens.next().expect_sep_w(
			';',
			Some("scoreboard declaration"),
			None,
		).fatal().scope;
		Scoreboard { keyword, objective, name, semi_colon }
	}

	pub fn parse_function(&mut self, keyword:Ident) -> Function {
		let name = self.tokens.next().expect_ident(
			Some("function declaration"),
			None,
		).fatal();

		// get function args
		let group = self.tokens.next().expect_group_w(
			Delimiter::Parentheses,
			Some("scoreboard declaration"),
			None,
		).fatal();

		// parse function args
		let mut input = Vec::new();
		let mut output = Vec::new();
		let mut arrow = None;
		let mut colon = None;
		let mut objective = None;

		group.tokens.iter_tokens(|token, _iter| {
			if let Some(ident) = token.as_ident() {
				if colon.is_some() {
					if objective.is_some() {
						error(ident.scope, ErrorInfo {
							cause: "illegal identifier, only 1 identifier after the colon",
							pointer: "an objective has already been defined",
							context: Some("function args declaration"),
							help: None
						}).fatal()
					}
					objective = Some(ident.clone());
				}
				else if arrow.is_some() {
					output.push(ident.clone());
				} else { input.push(ident.clone()); }
			}
			if let Some(symbol) = token.as_symbol_w("->") {
				arrow = Some(symbol.scope);
			}
			if let Some(sep) = token.as_sep_w(':') {
				colon = Some(sep.scope);
			}
		});
		let args = FunArgs {
			parens: group.scope, input,
			arrow, output, colon, objective
		};

		// check for at mod
		let mut at:Option<At> = None;
		let mut next = self.tokens.next().clone();
		if let Some(ident) = next.as_ident_w("at") {
			// parse at mod
			let loc_token = self.tokens.next();
			let loc = AtLoc::parse(
				ident,
				Some("function declaration"),
				None
			).fatal();
			at = Some(At {
				ident: ident.clone(),
				loc,
				loc_scope: loc_token.as_ident().unwrap().scope
			});
			next = self.tokens.next().clone();
		}
		// get block
		let block = next.expect_group_w(
			Delimiter::CurlyBrackets,
			Some("function declaration"),
			None
		).fatal();

		Function {
			keyword, name, args, block, at
		}
	}

	pub fn parse_function_block(block:&Group) -> Vec<Statement> {
		let mut statements:Vec<Statement> = Vec::new();

		block.tokens.iter_tokens(|token, iter| {
			if let Some(ident) = token.as_ident_w("unsafe") {
				let block = iter.next().expect_group_w(
					Delimiter::CurlyBrackets,
					Some("unsafe block declaration"),
					None,
				).fatal();
				let mut commands = Vec::new();
				let mut command = UnsafeCmd::new();

				block.tokens.iter_tokens(|token, _iter| {
					if token.as_sep_w(';').is_some() {
						if !command.tokens.is_empty() {
							commands.push(command.clone());
							command = UnsafeCmd::new();
						}
					} else {
						command.tokens.push(token.clone());
					}
				});

				statements.push(Statement::Unsafe(Unsafe {
					ident: ident.clone(),
					block,
					commands,
				}))
			}

			if let Some(path) = iter.get_path(Some("")) {
				let group = iter.next().expect_group_w(
					Delimiter::Parentheses,
					Some("function call declaration"),
					None,
				).fatal();
				let mut args = Vec::new();
				let mut arg = Arg::new();
				group.tokens.iter_tokens(|token, _iter| {
					if let Some(sep) = token.as_sep_w(',') {
						assert!(!arg.tokens.is_empty());
						arg.sep = Some(sep.clone());
						args.push(arg.clone());
						arg = Arg::new();
					}
					arg.tokens.push(token);
				});

				statements.push(Statement::Call(Call {
					path, group, args,
				}));
			}

			if let Some(_sep) = token.as_sep_w('|') {

			}
		});
		statements
	}
}

#[derive(Debug)]
pub struct Datapack {
	pub name: Ident,
	pub functions: Vec<Function>,
	pub scoreboards: Vec<Scoreboard>,
	pub inlines: Vec<Inline>,
}
impl Datapack {
	pub fn empty(name:Ident) -> Self {
		Datapack {
			name,
			functions: Vec::new(),
			scoreboards: Vec::new(),
			inlines: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Function {
	pub keyword: Ident,
	pub name: Ident,
	pub args: FunArgs,
	pub block: Group,
	pub at: Option<At>,
}
#[derive(Debug, Clone)]
pub struct Scoreboard {
	pub keyword: Ident,
	pub objective: Ident,
	pub name: Ident,
	pub semi_colon: SrcScope,
}
#[derive(Debug, Clone)]
pub struct Inline {
	pub keyword: Ident,
	pub name: Ident,
	pub args: Vec<InlineArg>,
	pub parens: SrcScope,
	pub block: Group,
}

#[derive(Debug, Clone)]
pub struct FunArgs {
	pub parens: SrcScope,
	pub input: Vec<Ident>,
	pub arrow: Option<SrcScope>,
	pub output: Vec<Ident>,
	pub colon: Option<SrcScope>,
	pub objective: Option<Ident>,
}
#[derive(Debug, Clone)]
pub struct InlineArg {
	pub name: Ident,
	pub colon: SrcScope,
	pub ty: InlineArgType,
	pub ty_scope: SrcScope,
	pub sep: Option<Sep>,
}
#[derive(Debug, Clone)]
pub enum Statement {
	Unsafe(Unsafe),
	Call(Call),
	Pipe(Pipe),
	If(If),
	At(At),
}

#[derive(Debug, Clone)] pub struct Unsafe {
	pub ident: Ident,
	pub block: Group,
	pub commands: Vec<UnsafeCmd>
}
#[derive(Debug, Clone)] pub struct Call {
	pub path: Path,
	pub group: Group,
	pub args: Vec<Arg>,
}
#[derive(Debug, Clone)] pub struct Pipe {
	pub symbol: Sep,
	pub name: Ident,
	pub equals: Symbol,
	pub call: Call,
}
#[derive(Debug, Clone)] pub struct If {
	pub call: Call,
	pub statements: Vec<Statement>,
}
#[derive(Debug, Clone)] pub struct At {
	pub ident: Ident,
	pub loc: AtLoc,
	pub loc_scope: SrcScope,
}

//
#[derive(Debug, Clone)] pub struct UnsafeCmd {
	pub tokens: Vec<Token>,
	pub sep: Option<Sep>,
}
impl UnsafeCmd {
	pub fn new() -> Self { UnsafeCmd {
		tokens: Vec::new(),
		sep: None,
	}}
}
#[derive(Debug, Clone)]
pub struct Arg {
	pub tokens: Vec<Token>,
	pub sep: Option<Sep>,
}
impl Arg {
	pub fn new() -> Self { Arg {
		tokens: Vec::new(),
		sep: None,
	}}
}
#[derive(Debug, Clone)]
pub struct Path {
	parts: Vec<Ident>,
	seps: Vec<Sep>,
}
impl Path {
	pub fn new(part:Ident) -> Self {
		let mut path = Path { parts: Vec::new(), seps: Vec::new() };
		path.parts.push(part);
		path
	}
	pub fn push_part(&mut self, sep:Sep, part:Ident) {
		self.seps.push(sep);
		self.parts.push(part);
	}
}

// Enums
#[derive(Debug, Clone)]
pub enum AtLoc {
	Player,
	Players,
	Entity,
	Entities,
	Any,
}
impl AtLoc {
	pub fn parse(
		ident:&Ident, context:Option<&str>, help:Option<Help>
	) -> CompileResult<Self> {
		match ident.value.as_str() {
			"player" => CompileResult::Ok(Self::Player),
			"players" => CompileResult::Ok(Self::Players),
			"entity" => CompileResult::Ok(Self::Entity),
			"entities" => CompileResult::Ok(Self::Entities),
			"any" => CompileResult::Ok(Self::Any),
			_ => CompileResult::Err(error(ident.scope, ErrorInfo {
				cause: "invalid `location`",
				pointer: "expected one of `player`, `players`, `entity`, `entities`, or `any`",
				context, help
			})),
		}
	}
}

#[derive(Debug, Clone)]
pub enum InlineArgType {
	Target,
	Objective,
	Score,
	Int,
}
impl InlineArgType {
	pub fn parse(
		ident:&Ident, context:Option<&str>, help:Option<Help>
	) -> CompileResult<Self> {
		match ident.value.as_str() {
			"target" => CompileResult::Ok(Self::Target),
			"objective" => CompileResult::Ok(Self::Objective),
			"score" => CompileResult::Ok(Self::Score),
			"int" => CompileResult::Ok(Self::Int),
			_ => CompileResult::Err(error(ident.scope, ErrorInfo {
				cause: "invalid `location`",
				pointer: "expected one of `target`, `objective`, `score`, or `int`",
				context, help
			})),
		}
	}
}
