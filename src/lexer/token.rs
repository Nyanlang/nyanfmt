use nom::{InputLength, InputTake};

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

#[derive(Debug, PartialEq, Clone)]
pub struct TokenStream<'a> {
	stream: &'a [Token],
}

impl<'a> InputLength for TokenStream<'a> {
	fn input_len(&self) -> usize {
		self.stream.len()
	}
}
