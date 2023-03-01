use std::{
	iter::{Flatten, Peekable},
	str::Chars,
};

#[derive(Debug, PartialEq)]
pub enum Token {
	Right,
	Left,
	Inc,
	Dec,
	Out,
	In,
	JumpRight,
	JumpLeft,
}

#[derive(Debug, PartialEq)]
enum OutToken {
	Token(Token),
	WS,
}

#[derive(Debug)]
struct Lexer<'a> {
	code: Chars<'a>,
}

impl<'a> Lexer<'a> {
	fn new(code: Chars<'a>) -> Self {
		Self { code }
	}

	fn tokenize(ch: char) -> Option<Token> {
		use Token::*;

		match ch {
			'?' => Some(Right),
			'!' => Some(Left),
			'냥' => Some(Inc),
			'냐' => Some(Dec),
			'.' => Some(Out),
			',' => Some(In),
			'~' => Some(JumpRight),
			'-' => Some(JumpLeft),
			_ => None,
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Vec<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		self.code
			.next()
			.map(Self::tokenize)
			.map(|v| v.map_or(Vec::new(), |tok| vec![tok]))
	}
}

struct Formatter<'a> {
	token_stream: Peekable<Flatten<Lexer<'a>>>,
}

impl<'a> From<Lexer<'a>> for Formatter<'a> {
	fn from(lexer: Lexer<'a>) -> Self {
		Self {
			token_stream: lexer.flatten().peekable(),
		}
	}
}

impl<'a> Iterator for Formatter<'a> {
	type Item = Vec<OutToken>;

	fn next(&mut self) -> Option<Self::Item> {
		use Token::*;

		let Some(current) = self.token_stream.next() else {
            return None;
        };
		let Some(next) = self.token_stream.peek() else {
            return Some(vec![OutToken::Token(current)]);
        };

		if match (&current, next) {
			(Right | Left, Right | Left) => false,
			(Right | Left, _)
			| (Out | In | JumpRight | JumpLeft, Inc | Dec) => true,
			_ => false,
		} {
			Some(vec![
				OutToken::Token(current),
				OutToken::WS,
			])
		} else {
			Some(vec![OutToken::Token(current)])
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn lex() {
		use Token::*;

		let code = "냥냥냥냥냥 냥냥냥~? 냥냥냥냥~? 냥냥? 냥냥냥? 냥냥냥? 냥!!!! 냐-? 냥? 냥? 냐?? 냥~! -! 냐-?? .? 냐냐냐. ";
		let lexer = Lexer::new(code.chars());

		assert_eq!(
			lexer.flatten().collect::<Vec<Token>>(),
			[
				Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, JumpRight, Right, Inc,
				Inc, Inc, Inc, JumpRight, Right, Inc, Inc, Right, Inc, Inc,
				Inc, Right, Inc, Inc, Inc, Right, Inc, Left, Left, Left, Left,
				Dec, JumpLeft, Right, Inc, Right, Inc, Right, Dec, Right,
				Right, Inc, JumpRight, Left, JumpLeft, Left, Dec, JumpLeft,
				Right, Right, Out, Right, Dec, Dec, Dec, Out
			],
		)
	}

	#[test]
	fn format() {
		use Token::*;

		let code = "냥~?냥냥??냥냥-???-!-??.?냐.";
		let rep = Formatter::from(Lexer::new(code.chars()));

		assert_eq!(
			rep.flatten().collect::<Vec<OutToken>>(),
			[
				OutToken::Token(Inc),
				OutToken::Token(JumpRight),
				OutToken::Token(Right),
				OutToken::WS,
				OutToken::Token(Inc),
				OutToken::Token(Inc),
				OutToken::Token(Right),
				OutToken::Token(Right),
				OutToken::WS,
				OutToken::Token(Inc),
				OutToken::Token(Inc),
				OutToken::Token(JumpLeft),
				OutToken::Token(Right),
				OutToken::Token(Right),
				OutToken::Token(Right),
				OutToken::WS,
				OutToken::Token(JumpLeft),
				OutToken::Token(Left),
				OutToken::WS,
				OutToken::Token(JumpLeft),
				OutToken::Token(Right),
				OutToken::Token(Right),
				OutToken::WS,
				OutToken::Token(Out),
				OutToken::Token(Right),
				OutToken::WS,
				OutToken::Token(Dec),
				OutToken::Token(Out),
			],
		)
	}
}
