use std::{iter::Peekable, str::Chars, vec::IntoIter};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
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

impl<'a> From<Chars<'a>> for Lexer<'a> {
	fn from(chars: Chars<'a>) -> Self {
		Self::new(chars)
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

#[derive(Default)]
struct State {
	counter: u32,
}

type TokenStream = Peekable<IntoIter<Token>>;

struct Formatter {
	token_stream: TokenStream,
	state: State,
}

impl Formatter {
	fn new(token_stream: TokenStream) -> Self {
		Self {
			token_stream,
			state: State::default(),
		}
	}
}

impl<'a> From<Lexer<'a>> for Formatter {
	fn from(lexer: Lexer<'a>) -> Self {
		Self::new(Vec::from(lexer).into_iter().peekable())
	}
}

impl From<Formatter> for Vec<Token> {
	fn from(mut formatter: Formatter) -> Self {
		use Token::*;

		let mut v = vec![];

		let mut ts = formatter
			.token_stream
			.filter(|&tok| tok != Span)
			.peekable();

		while let Some(token) = ts.next() {
			let Some(next) = ts.peek() else {
                v.push(token);
                break;
            };

			match token {
				Right | Left => {
					v.push(token);
					if !matches!(next, Right | Left) {
						v.push(Span);
					}
				},
				Out | In | JumpRight | JumpLeft => {
					v.push(token);
					if matches!(next, Inc | Dec) {
						v.push(Span);
					}
				},
				Inc | Dec => {
					v.push(token);
					if matches!(next, Inc | Dec) {
						formatter.state.counter += 1;
						if formatter.state.counter == 5 {
							v.push(Span);
							formatter.state.counter = 0;
						}
					} else {
						formatter.state.counter = 0;
					}
				},
				Span => (),
			}
		}

		v
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn lex() {
		use Token::*;

		let code = "냥냥냥냥냥  냥냥냥~? 냥냥냥냥~? 냥냥?    냥냥냥? 냥냥냥?냥!!  !!  냐-? 냥?냥?  냐??냥~!-!   냐-??.? 냐  냐냐. ";
		let lexer: Lexer = code.chars().into();
		let token_stream: Vec<Token> = lexer.into();

		assert_eq!(
			token_stream,
			[
				Inc, Inc, Inc, Inc, Inc, Span, Inc, Inc, Inc, JumpRight, Right,
				Span, Inc, Inc, Inc, Inc, JumpRight, Right, Span, Inc, Inc,
				Right, Span, Inc, Inc, Inc, Right, Span, Inc, Inc, Inc, Right,
				Inc, Left, Left, Span, Left, Left, Span, Dec, JumpLeft, Right,
				Span, Inc, Right, Inc, Right, Span, Dec, Right, Right, Inc,
				JumpRight, Left, JumpLeft, Left, Span, Dec, JumpLeft, Right,
				Right, Out, Right, Span, Dec, Span, Dec, Dec, Out, Span
			],
		)
	}

	#[test]
	fn format() {
		use Token::*;

		let code = " 냥 ~?냥 냥? ?냥냥냥 냥냥냥 -? ??- !- ?? .? 냐.";
		let lexer: Lexer = code.chars().into();
		let formatter: Formatter = lexer.into();
		let formatted_token_stream: Vec<Token> = formatter.into();

		assert_eq!(
			formatted_token_stream,
			[
				Inc, JumpRight, Right, Span, Inc, Inc, Right, Right, Span, Inc,
				Inc, Inc, Inc, Inc, Span, Inc, JumpLeft, Right, Right, Right,
				Span, JumpLeft, Left, Span, JumpLeft, Right, Right, Span, Out,
				Right, Span, Dec, Out,
			],
		)
	}
}
