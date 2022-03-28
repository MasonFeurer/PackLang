use std::fmt::{Debug, Display, Formatter};
use crate::debug::bash_tools::*;
use crate::compiler::files::FileRef;
use crate::debug::errors::{CompileResult, error, ErrorInfo, Help};

#[derive(Clone, Debug)]
pub enum Token {
	Ident(Ident),
	Str(Str),
	Int(Int),
	Sep(Sep),
	Symbol(Symbol),
	End(End),
	Group(Group),
}
impl Token {
	pub fn ident(value:String, scope:SrcScope) -> Self {
		Self::Ident(Ident { value, scope })
	}
	pub fn string(value:String, scope:SrcScope) -> Self {
		Self::Str(Str { value, scope })
	}
	pub fn int(value:i32, scope:SrcScope) -> Self {
		Self::Int(Int { value, scope })
	}
	pub fn sep(value:char, scope:SrcScope) -> Self {
		Self::Sep(Sep { value, scope })
	}
	pub fn symbol(value:String, scope:SrcScope) -> Self {
		Self::Symbol(Symbol { value, scope })
	}
	pub fn end(scope:SrcScope) -> Self {
		Self::End(End { scope })
	}
	pub fn group(delimiter:Delimiter, tokens:Vec<Token>, scope:SrcScope) -> Self {
		Self::Group(Group { delimiter, tokens, scope })
	}

	pub fn scope(&self) -> &SrcScope {
		match self {
			Self::Ident(e) => &e.scope,
			Self::Str(e) => &e.scope,
			Self::Int(e) => &e.scope,
			Self::Sep(e) => &e.scope,
			Self::Symbol(e) => &e.scope,
			Self::End(e) => &e.scope,
			Self::Group(e) => &e.scope,
		}
	}

	pub fn is_ident(&self) -> bool {
		if let Self::Ident(_) = self { true } else { false }
	}
	pub fn is_string(&self) -> bool {
		if let Self::Str(_) = self { true } else { false }
	}
	pub fn is_int(&self) -> bool {
		if let Self::Int(_) = self { true } else { false }
	}
	pub fn is_sep(&self) -> bool {
		if let Self::Sep(_) = self { true } else { false }
	}
	pub fn is_symbol(&self) -> bool {
		if let Self::Symbol(_) = self { true } else { false }
	}
	pub fn is_end(&self) -> bool {
		if let Self::End(_) = self { true } else { false }
	}
	pub fn is_group(&self) -> bool {
		if let Self::Group(_) = self { true } else { false }
	}

	pub fn as_ident(&self) -> Option<&Ident> {
		if let Self::Ident(e) = self { Some(e) } else { None }
	}
	pub fn as_string(&self) -> Option<&Str> {
		if let Self::Str(e) = self { Some(e) } else { None }
	}
	pub fn as_int(&self) -> Option<&Int> {
		if let Self::Int(e) = self { Some(e) } else { None }
	}
	pub fn as_sep(&self) -> Option<&Sep> {
		if let Self::Sep(e) = self { Some(e) } else { None }
	}
	pub fn as_symbol(&self) -> Option<&Symbol> {
		if let Self::Symbol(e) = self { Some(e) } else { None }
	}
	pub fn as_end(&self) -> Option<&End> {
		if let Self::End(e) = self { Some(e) } else { None }
	}
	pub fn as_group(&self) -> Option<&Group> {
		if let Self::Group(e) = self { Some(e) } else { None }
	}

	pub fn as_symbol_w(&self, value:&str) -> Option<&Symbol> {
		if let Self::Symbol(e) = self {
			if e.value.as_str() == value { Some(e) } else { None }
		} else { None }
	}
	pub fn as_sep_w(&self, value:char) -> Option<&Sep> {
		if let Self::Sep(e) = self {
			if e.value == value { Some(e) } else { None }
		} else { None }
	}
	pub fn as_ident_w(&self, value:&str) -> Option<&Ident> {
		if let Self::Ident(e) = self {
			if e.value.as_str() == value { Some(e) } else { None }
		} else { None }
	}
	
	pub fn as_ok(&self) -> CompileResult<Self> {
		match self {
			Self::Ident(_) => CompileResult::Ok(self.clone()),
			Self::Str(_) => CompileResult::Ok(self.clone()),
			Self::Int(_) => CompileResult::Ok(self.clone()),
			Self::Sep(_) => CompileResult::Ok(self.clone()),
			Self::Symbol(_) => CompileResult::Ok(self.clone()),
			Self::End(_) => CompileResult::Ok(self.clone()),
			Self::Group(_) => CompileResult::Ok(self.clone()),
		}
	}

	pub fn expect_non_end(&self, more:&str, help:Option<Help>) -> CompileResult<Token> {
		match self {
			Token::End(_) => Self::invalid_token(self, more, help),
			_ => self.as_ok()
		}
	}
	pub fn expect_ident(&self, more:&str, help:Option<Help>) -> CompileResult<Ident> {
		match self {
			Token::Ident(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_string(&self, more:&str, help:Option<Help>) -> CompileResult<Str> {
		match self {
			Token::Str(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_int(&self, more:&str, help:Option<Help>) -> CompileResult<Int> {
		match self {
			Token::Int(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_sep(&self, more:&str, help:Option<Help>) -> CompileResult<Sep> {
		match self {
			Token::Sep(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_symbol(&self, more:&str, help:Option<Help>) -> CompileResult<Symbol> {
		match self {
			Token::Symbol(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_end(&self, more:&str, help:Option<Help>) -> CompileResult<End> {
		match self {
			Token::End(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}
	pub fn expect_group(&self, more:&str, help:Option<Help>) -> CompileResult<Group> {
		match self {
			Token::Group(e) => e.as_ok(),
			_ => Self::invalid_token(self, more, help),
		}
	}

	pub fn expect_ident_w(&self, value:&str, more:&str, help:Option<Help>) -> CompileResult<Ident> {
		match self {
			Self::Ident(e) => {
				if e.value.as_str() == value { e.as_ok() }
				else { Self::invalid_token(self, more, help) }
			}
			_ => Self::invalid_token(self, more, help)
		}
	}
	pub fn expect_sep_w(&self, value:char, more:&str, help:Option<Help>) -> CompileResult<Sep> {
		match self {
			Self::Sep(e) => {
				if e.value == value { e.as_ok() }
				else { Self::invalid_token(self, more, help) }
			}
			_ => Self::invalid_token(self, more, help)
		}
	}
	pub fn expect_group_w(&self, delimiter:Delimiter, more:&str, help:Option<Help>) -> CompileResult<Group> {
		match self {
			Self::Group(e) => {
				if e.delimiter == delimiter { e.as_ok() }
				else { Self::invalid_token(self, more, help) }
			}
			_ => Self::invalid_token(self, more, help)
		}
	}

	fn invalid_token<T: Debug>(token:&Token, more:&str, help:Option<Help>) -> CompileResult<T> {
		CompileResult::Err(error(
			token.scope(),
			ErrorInfo {
				cause: format!("invalid token: {}", token).as_str(),
				more, help
			}
		))
	}

	pub fn display(&self) -> String {
		match self {
			Self::Ident(e) => e.display(),
			Self::Str(e) => e.display(),
			Self::Int(e) => e.display(),
			Self::Sep(e) => e.display(),
			Self::Symbol(e) => e.display(),
			Self::End(e) => e.display(),
			Self::Group(e) => e.display(),
		}
	}
}
impl Display for Token {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct Ident {
	pub value: String,
	pub scope: SrcScope,
}
impl Ident {
	pub fn display(&self) -> String {
		format!("`{}`", self.value)
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Ident {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct Str {
	pub value: String,
	pub scope: SrcScope,
}
impl Str {
	pub fn display(&self) -> String {
		format!("\"{}\"", self.value)
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Str {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct Int {
	pub value: i32,
	pub scope: SrcScope,
}
impl Int {
	pub fn display(&self) -> String {
		self.value.to_string()
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Int {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct Sep {
	pub value: char,
	pub scope: SrcScope,
}
impl Sep {
	pub fn display(&self) -> String {
		format!("`{}`", self.value)
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Sep {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct Symbol {
	pub value: String,
	pub scope: SrcScope,
}
impl Symbol {
	pub fn display(&self) -> String {
		format!("`{}`", self.value)
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Symbol {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug)]
pub struct End {
	pub scope: SrcScope,
}
impl End {
	pub fn display(&self) -> String {
		format!(
			"{}end{}",
			Fmt::DecorColor(BOLD,LIGHT_WHITE), Fmt::Reset
		)
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for End {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Delimiter {
	Parentheses,
	Brackets,
	CurlyBrackets,
	AngledBrackets,
}
impl Delimiter {
	pub const fn opening(&self) -> char {
		match self {
			Self::Parentheses => '(',
			Self::Brackets => '[',
			Self::CurlyBrackets => '{',
			Self::AngledBrackets => '<',
		}
	}
	pub const fn closing(&self) -> char {
		match self {
			Self::Parentheses => ')',
			Self::Brackets => ']',
			Self::CurlyBrackets => '}',
			Self::AngledBrackets => '>',
		}
	}
}

#[derive(Clone, Debug)]
pub struct Group {
	pub delimiter: Delimiter,
	pub tokens: Vec<Token>,
	pub scope: SrcScope,
}
impl Group {
	pub fn display(&self) -> String {
		let mut string = String::new();
		string.push(self.delimiter.opening());
		for token in &self.tokens {
			string.push_str(token.display().as_str());
			string.push(' ');
		}
		string.push(self.delimiter.closing());
		string
	}
	pub fn as_ok(&self) -> CompileResult<Self> {
		CompileResult::Ok(self.clone())
	}
}
impl Display for Group {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.display().as_str())
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SrcScope {
	pub start: usize,
	pub end: usize,
	pub file: FileRef,
}
impl SrcScope {
	// NOTE: expects `other` to appear after `self` in the source
	pub fn join(&self, other:&Self) -> Self {
		assert_eq!(self.file, other.file);
		SrcScope {
			start: self.start,
			end: other.end,
			file: self.file,
		}
	}

	pub fn lines(&self) -> Vec<usize> {
		let source = self.file.source();

		let mut lines:Vec<usize> = Vec::new();
		let mut line:usize = 0;
		for (i, c) in source.chars().enumerate() {
			if i >= self.end { break }
			if i == self.start { lines.push(line) }
			if c == '\n' {
				line += 1;
				if i > self.start {
					lines.push(line)
				}
			}
		}
		lines
	}
	pub fn len(&self) -> usize {
		self.end-self.start
	}
}
