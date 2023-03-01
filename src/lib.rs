use std::{iter::Peekable, str::Chars, vec::IntoIter};

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
	Span,
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
			' ' | '\t'..='\r' => Some(Span),
			_ => None,
		}
	}
}

impl<'a> From<Lexer<'a>> for Vec<Token> {
	fn from(mut lexer: Lexer<'a>) -> Self {
		let mut v: Vec<Token> = vec![];
		let mut is_span = false;

		while let Some(ch) = lexer.code.next() {
			let Some(token) = Lexer::tokenize(ch) else {
                continue;
            };

			is_span = match token {
				Token::Span => {
					if !is_span {
						v.push(Token::Span)
					}
					true
				},
				_ => {
					v.push(token);
					false
				},
			};
		}

		v
	}
}

struct Formatter {
	token_stream: Peekable<IntoIter<Token>>,
}

impl<'a> From<Lexer<'a>> for Formatter {
	fn from(lexer: Lexer<'a>) -> Self {
		Self {
			token_stream: Vec::from(lexer).into_iter().peekable(),
		}
	}
}

impl<'a> Iterator for Formatter {
	type Item = Vec<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		use Token::*;

		let Some(current) = self.token_stream.next() else {
            return None;
        };
		let Some(next) = self.token_stream.peek() else {
            return Some(vec![current]);
        };

		if match (&current, next) {
			(Right | Left, Right | Left) => false,
			(Right | Left, _)
			| (Out | In | JumpRight | JumpLeft, Inc | Dec) => true,
			_ => false,
		} {
			Some(vec![current, Token::Span])
		} else {
			Some(vec![current])
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
		let token_stream: Vec<Token> = lexer.into();

		assert_eq!(
			token_stream,
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
			rep.flatten().collect::<Vec<Token>>(),
			[
				Inc, JumpRight, Right, Span, Inc, Inc, Right, Right, Span, Inc,
				Inc, JumpLeft, Right, Right, Right, Span, JumpLeft, Left, Span,
				JumpLeft, Right, Right, Span, Out, Right, Span, Dec, Out,
			],
		)
	}
}
