use std::str::Chars;

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

struct Lexer<'a> {
	code: Chars<'a>,
}

impl<'a> Lexer<'a> {
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
