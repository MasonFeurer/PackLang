use crate::compiler::files::FileRef;
use crate::compiler::tokens::{Delimiter, SrcScope, Token};
use crate::debug::errors::{error, ErrorInfo};

pub struct Lexer {
	file: FileRef,
	chars: Vec<char>,
	index: usize,
}

impl Lexer {
	pub fn new(file:FileRef) -> Self {
		let chars:Vec<char> = file.source().chars().collect();
		Lexer {
			file,
			chars,
			index: 0,
		}
	}

	pub fn lex(&mut self) -> Vec<Token> {
		let mut tokens:Vec<Token> = Vec::new();
		let num_chars = self.chars.len();

		while self.index < num_chars {
			let c = self.chars[self.index];
			if let Some(token) = self.next(c) {
				tokens.push(token);
			}
		};
		tokens.push(Token::end(self.len_scope(1)));
		tokens
	}

	fn next(&mut self, c:char) -> Option<Token> {
		let num_chars = self.chars.len();
		if c == '"' {
			let opening_pos = self.index;
			let mut string = String::new();
			let mut found_closing = false;
			self.index += 1;
			while self.index < num_chars {
				let c = self.chars[self.index];
				self.index += 1;
				if c == '"' {
					found_closing = true;
					break;
				}
				string.push(c);
			}
			let scope = self.scope(opening_pos, self.index);
			if !found_closing {
				error(&scope, ErrorInfo {
					cause: "missing closing quote",
					more: "opening quote is missing a matching closing quote",
					help: None,
				}).fatal();
			}
			return Some(Token::string(string, scope));
		}
		if is_sep(c) {
			self.index += 1;
			let token = Token::sep(
				c, self.len_scope(1)
			);
			return Some(token);
		}
		if is_symbol(c) {
			let mut value = String::new();
			value.push(c);
			self.index += 1;
			while self.index < num_chars {
				let nc = self.chars[self.index];
				if !is_symbol(nc) { break }
				value.push(nc);
				self.index += 1;
			}
			let scope = self.len_scope(value.len());
			return Some(Token::symbol(value, scope));
		}
		if starts_num(c) {
			let mut value = String::new();
			value.push(c);
			self.index += 1;
			while self.index < num_chars {
				let nc = self.chars[self.index];
				if is_ident_sep(nc) { break }
				value.push(nc);
				self.index += 1;
			}
			let scope = self.len_scope(value.len());
			let value = match value.parse() {
				Ok(value) => value,
				Err(err) => error(&scope, ErrorInfo {
					cause: "invalid integer literal",
					more: format!("failed to parse integer, reason: {}", err).as_str(),
					help: None,
				}).fatal()
			};
			return Some(Token::int(value, scope));
		}
		if starts_ident(c) {
			let mut value = String::new();
			value.push(c);
			self.index += 1;
			while self.index < num_chars {
				let nc = self.chars[self.index];
				if !in_ident(nc) { break }
				value.push(nc);
				self.index += 1;
			}
			let scope = self.len_scope(value.len());
			return Some(Token::ident(value, scope));
		}
		if c.is_whitespace() {
			self.index += 1;
			return None;
		}

		if let Some(delimiter) = delimiter(c) {
			let opening_pos = self.index;
			let Some(tokens) = self.get_group_tokens(delimiter.closing()) else {
				let scope = self.scope(opening_pos, opening_pos+1);
				error(&scope, ErrorInfo {
					cause: "missing closing delimiter",
					more: "opening delimiter doesn't have a matching closing delimiter",
					help: None,
				}).fatal();
			};
			let scope = self.scope(opening_pos, self.index);
			return Some(Token::group(delimiter, tokens, scope))
		}

		self.index += 1;
		error(&self.len_scope(1), ErrorInfo {
			cause: "illegal character",
			more: "this character does not start a token",
			help: None,
		}).fatal();
	}

	fn get_group_tokens(&mut self, until:char) -> Option<Vec<Token>> {
		let mut tokens:Vec<Token> = Vec::new();
		let num_chars = self.chars.len();

		let mut found_closing = false;
		self.index += 1;
		while self.index < num_chars {
			let c = self.chars[self.index];
			if c == until {
				found_closing = true;
				self.index += 1;
				break;
			}
			if let Some(token) = self.next(c) {
				tokens.push(token);
			}
		};
		if !found_closing { return None }
		tokens.push(Token::end(self.len_scope(1)));
		Some(tokens)
	}

	fn len_scope(&self, len:usize) -> SrcScope {
		SrcScope { file: self.file, start: self.index-len, end: self.index }
	}
	fn scope(&self, start:usize, end:usize) -> SrcScope {
		SrcScope { file: self.file, start, end }
	}
}

fn is_ident_sep(c:char) -> bool {
	c.is_whitespace() || is_sep(c) || is_symbol(c) || is_delimiter(c)
}
fn is_sep(c:char) -> bool {
	c == ',' || c == ':' || c == '@' || c == ';' ||
	c == '|'
}
fn is_symbol(c:char) -> bool {
	c == '+' || c == '-' || c == '*' || c == '/' ||
	c == '%' || c == '=' || c == '.' || c == '>' ||
	c == '<'
}
fn is_delimiter(c:char) -> bool {
	match c {
		'(' | ')' | '{' | '}' | '[' | ']' | '<' | '>' => true,
		_ => false,
	}
}
fn delimiter(c:char) -> Option<Delimiter> {
	match c {
		'(' => Some(Delimiter::Parentheses),
		'{' => Some(Delimiter::CurlyBrackets),
		'[' => Some(Delimiter::Brackets),
		'<' => Some(Delimiter::AngledBrackets),
		_ => None,
	}
}
fn starts_num(c:char) -> bool {
	c.is_digit(10)
}
fn in_num(c:char) -> bool {
	c.is_digit(10) || c == '_'
}
fn starts_ident(c:char) -> bool {
	c.is_alphabetic() || c == '_' || c == '$'
}
fn in_ident(c:char) -> bool {
	c.is_alphabetic() || c == '_' || c.is_digit(10)
}
