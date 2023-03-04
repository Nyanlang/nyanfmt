use nom::InputLength;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Right,
	Left,
	Inc,
	Dec,
	Out,
	In,
	JumpRight,
	JumpLeft,
	Debug,
	Comment(String),
	NewLine,
}

impl InputLength for Token {
	#[inline]
	fn input_len(&self) -> usize {
		1
	}
}

impl InputLength for &Token {
	#[inline]
	fn input_len(&self) -> usize {
		Token::input_len(self)
	}
}
