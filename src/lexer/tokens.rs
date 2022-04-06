use std::fmt::{Debug, Formatter};
use crate::debug::errors::{CompileResult, error, ErrorInfo, Help};
use crate::files::FileRef;

#[derive(Clone)]
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

	pub fn scope(&self) -> SrcScope {
		match self {
			Self::Ident(e) => e.scope,
			Self::Str(e) => e.scope,
			Self::Int(e) => e.scope,
			Self::Sep(e) => e.scope,
			Self::Symbol(e) => e.scope,
			Self::End(e) => e.scope,
			Self::Group(e) => e.scope,
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

	pub fn as_non_end(&self) -> Option<&Token> {
		if let Self::End(_) = self { None } else { Some(self) }
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

	fn invalid<T: Debug>(
		&self,
		expected:&str,
		context:Option<&str>,
		help:Option<Help>
	) -> CompileResult<T> {
		CompileResult::Err(error(
			self.scope(), ErrorInfo {
				cause: "Invalid token",
				pointer: format!("expected {}, found {:#1?}", expected, self).as_str(),
				context,
				help
			}
		))
	}

	pub fn expect_non_end(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Token> {
		match self {
			Token::End(_) => self.invalid("non End()", context, help),
			_ => CompileResult::Ok(self.clone())
		}
	}
	pub fn expect_ident(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Ident> {
		match self {
			Token::Ident(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("Ident()", context, help),
		}
	}
	pub fn expect_string(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Str> {
		match self {
			Token::Str(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("Str()", context, help),
		}
	}
	pub fn expect_int(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Int> {
		match self {
			Token::Int(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("int", context, help),
		}
	}
	pub fn expect_sep(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Sep> {
		match self {
			Token::Sep(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("Sep()", context, help),
		}
	}
	pub fn expect_symbol(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Symbol> {
		match self {
			Token::Symbol(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("Symbol()", context, help),
		}
	}
	pub fn expect_end(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<End> {
		match self {
			Token::End(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("End()", context, help),
		}
	}
	pub fn expect_group(&self, context:Option<&str>, help:Option<Help>) -> CompileResult<Group> {
		match self {
			Token::Group(e) => CompileResult::Ok(e.clone()),
			_ => self.invalid("Group", context, help),
		}
	}

	pub fn expect_ident_w(&self, value:&str, context:Option<&str>, help:Option<Help>) -> CompileResult<Ident> {
		let err = self.invalid(format!(
			"Word(\"{}\")", value
		).as_str(), context, help);
		match self {
			Self::Ident(e) => {
				if e.value.as_str() == value { CompileResult::Ok(e.clone()) }
				else { err }
			}
			_ => err
		}
	}
	pub fn expect_sep_w(&self, value:char, context:Option<&str>, help:Option<Help>) -> CompileResult<Sep> {
		let err = self.invalid(format!(
			"Sep('{}')", value
		).as_str(), context, help);
		match self {
			Self::Sep(e) => {
				if e.value == value { CompileResult::Ok(e.clone()) }
				else { err }
			}
			_ => err
		}
	}
	pub fn expect_group_w(&self, delimiter:Delimiter, context:Option<&str>, help:Option<Help>) -> CompileResult<Group> {
		let err = self.invalid(format!(
			"Group<{:?}>", delimiter
		).as_str(), context, help);
		match self {
			Self::Group(e) => {
				if e.delimiter == delimiter { CompileResult::Ok(e.clone()) }
				else { err }
			}
			_ => err
		}
	}
}
impl Debug for Token {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Ident(e) => Debug::fmt(e, f),
			Self::Str(e) => Debug::fmt(e, f),
			Self::Int(e) => Debug::fmt(e, f),
			Self::Sep(e) => Debug::fmt(e, f),
			Self::Symbol(e) => Debug::fmt(e, f),
			Self::End(e) => Debug::fmt(e, f),
			Self::Group(e) => Debug::fmt(e, f),
		}
	}
}

#[derive(Clone)]
pub struct Ident {
	pub value: String,
	pub scope: SrcScope,
}
impl Debug for Ident {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Ident(")?;
		Debug::fmt(&self.value, f)?;
		f.write_str(")")
	}
}

#[derive(Clone)]
pub struct Str {
	pub value: String,
	pub scope: SrcScope,
}
impl Debug for Str {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Str(")?;
		Debug::fmt(&self.value, f)?;
		f.write_str(")")
	}
}

#[derive(Clone)]
pub struct Int {
	pub value: i32,
	pub scope: SrcScope,
}
impl Debug for Int {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Int(")?;
		Debug::fmt(&self.value, f)?;
		f.write_str(")")
	}
}

#[derive(Clone)]
pub struct Sep {
	pub value: char,
	pub scope: SrcScope,
}
impl Debug for Sep {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Sep(")?;
		Debug::fmt(&self.value, f)?;
		f.write_str(")")
	}
}

#[derive(Clone)]
pub struct Symbol {
	pub value: String,
	pub scope: SrcScope,
}
impl Debug for Symbol {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Symbol(")?;
		Debug::fmt(&self.value, f)?;
		f.write_str(")")
	}
}

#[derive(Clone)]
pub struct End {
	pub scope: SrcScope,
}
impl Debug for End {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("End()")
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

#[derive(Clone)]
pub struct Group {
	pub delimiter: Delimiter,
	pub tokens: Vec<Token>,
	pub scope: SrcScope,
}
impl Debug for Group {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		if let Some(_width) = f.width() {
			f.write_str("Group<")?;
			Debug::fmt(&self.delimiter, f)?;
			f.write_str(">(")?;
			f.write_str("...")?;
			f.write_str(")")
		} else {
			f.write_str("Group<")?;
			Debug::fmt(&self.delimiter, f)?;
			f.write_str(">(")?;
			Debug::fmt(&self.tokens, f)?;
			f.write_str(")")
		}
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
	pub fn join(&self, other:Self) -> Self {
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
