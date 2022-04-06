use std::fmt::Debug;
use crate::lexer::tokens::Token;
use crate::parser::Path;

pub trait TokenList {
	fn as_list(&self) -> &Vec<Token>;

	fn iter_tokens<F: FnMut(Token, &mut TokenIter)>(&self, mut f:F) {
		let mut iter = TokenIter::new(self.as_list().clone());
		let mut next = iter.next().clone();
		while !next.is_end() {
			f(next, &mut iter);
			next = iter.next().clone();
		}
	}
}
impl TokenList for Vec<Token> {
	#[inline(always)] fn as_list(&self) -> &Vec<Token> { self }
}

#[derive(Debug)]
pub struct TokenIter {
	pub index: usize,
	pub tokens: Vec<Token>,
}
impl TokenIter {
	// expects last token to be `Token::End`
	pub fn new(tokens:Vec<Token>) -> Self {
		assert!(tokens.last().unwrap().is_end());
		TokenIter {
			tokens,
			index: 0,
		}
	}

	pub fn current(&self) -> &Token {
		&self.tokens[self.index]
	}
	pub fn prev(&self) -> Option<&Token> {
		self.tokens.get(self.index-1)
	}
	pub fn next(&mut self) -> &Token {
		let result = &self.tokens[self.index];
		if !result.is_end() { self.index += 1; }
		result
	}

	pub fn get_path(&mut self, context:Option<&str>) -> Option<Path> {
		let first_token = self.prev().unwrap().clone();
		let Some(first) = first_token.as_ident() else { return None };
		let mut path = Path::new(first.clone());
		while let Some(token) = self.next().as_non_end() {
			let sep = token.expect_sep(
				context, None,
			).fatal();
			let part = self.next().expect_ident(
				context, None,
			).fatal();
			path.push_part(sep, part);
		}
		Some(path)
	}
}

pub fn traverse_tokens<F: Fn(&mut Token)>(tokens:&mut Vec<Token>, f:&F) {
	let mut i = 0;
	while i < tokens.len() {
		f(&mut tokens[i]);
		if let Token::Group(group) = &mut tokens[i] {
			traverse_tokens(&mut group.tokens, f);
		}
		i += 1;
	}
}
