use std::{iter::Enumerate, slice::Iter};

use nom::{InputIter, InputLength, InputTake, Needed};

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

impl<'a> TokenStream<'a> {
	fn new() -> Self {
		Self { stream: &[] }
	}
}

impl<'a> From<&'a [Token]> for TokenStream<'a> {
	fn from(stream: &'a [Token]) -> Self {
		Self { stream }
	}
}

impl<'a> InputIter for TokenStream<'a> {
	type Item = &'a Token;

	type Iter = Enumerate<Self::IterElem>;

	type IterElem = Iter<'a, Token>;

	fn iter_indices(&self) -> Self::Iter {
		self.iter_elements().enumerate()
	}

	fn iter_elements(&self) -> Self::IterElem {
		self.stream.iter()
	}

	fn position<P>(&self, predicate: P) -> Option<usize>
	where
		P: Fn(Self::Item) -> bool,
	{
		self.iter_elements().position(predicate)
	}

	fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
		if self.input_len() >= count {
			Ok(count)
		} else {
			Err(Needed::Unknown)
		}
	}
}

impl<'a> InputLength for TokenStream<'a> {
	fn input_len(&self) -> usize {
		self.stream.len()
	}
}

impl<'a> InputTake for TokenStream<'a> {
	fn take(&self, count: usize) -> Self {
		Self::from(&self.stream[..count])
	}

	fn take_split(&self, count: usize) -> (Self, Self) {
		match self.stream.split_at(count) {
			(l, r) => (Self::from(r), Self::from(l)),
		}
	}
}
