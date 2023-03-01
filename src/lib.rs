use std::str::Chars;

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
}
