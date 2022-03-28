use std::fmt::Debug;
use crate::compiler::tokens::*;

#[derive(Debug)]
pub struct TokenStream {
	pub index: usize,
	pub saved_index: Option<usize>,
	pub tokens: Vec<Token>,
}
impl TokenStream {
	// expects last token to be `Token::End`
	pub fn new(tokens:Vec<Token>) -> Self {
		assert!(tokens.len() > 0);
		assert!(tokens.last().unwrap().is_end());
		TokenStream {
			tokens,
			saved_index: None,
			index: 0,
		}
	}
	pub fn save(&mut self) {
		self.saved_index = Some(self.index);
	}
	pub fn load_save(&mut self) {
		self.index = self.saved_index.expect("no saved index");
	}

	pub fn current(&self) -> &Token {
		&self.tokens[self.index]
	}
	pub fn next(&mut self) -> &Token {
		let result = &self.tokens[self.index];
		if !result.is_end() { self.index += 1; }
		result
	}
}
