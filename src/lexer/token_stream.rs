use super::Token;
use nom::{
	Compare, CompareResult, FindToken, InputIter, InputLength, InputTake,
	Needed, UnspecializedInput,
};
use std::{iter::Enumerate, slice::Iter};

#[derive(Debug, PartialEq, Clone)]
pub struct TokenStream<'a> {
	stream: &'a [Token],
}

impl<'a> TokenStream<'a> {
	pub fn new() -> Self {
		Self { stream: &[] }
	}
}

impl<'a, T> From<T> for TokenStream<'a>
where
	T: Into<&'a [Token]>,
{
	fn from(stream: T) -> Self {
		Self {
			stream: stream.into(),
		}
	}
}

impl<'a> Compare<&Token> for TokenStream<'a> {
	fn compare(&self, t: &Token) -> CompareResult {
		match self.stream.get(0) {
			Some(v) if v == t => CompareResult::Ok,
			_ => CompareResult::Error,
		}
	}

	fn compare_no_case(&self, t: &Token) -> nom::CompareResult {
		self.compare(t)
	}
}

impl<'a> FindToken<&Token> for TokenStream<'a> {
	fn find_token(&self, token: &Token) -> bool {
		self.stream.contains(token)
	}
}

impl<'a> InputIter for TokenStream<'a> {
	type Item = &'a Token;

	type Iter = Enumerate<Self::IterElem>;

	type IterElem = Iter<'a, Token>;

	#[inline]
	fn iter_indices(&self) -> Self::Iter {
		self.iter_elements().enumerate()
	}

	#[inline]
	fn iter_elements(&self) -> Self::IterElem {
		self.stream.iter()
	}

	#[inline]
	fn position<P>(&self, predicate: P) -> Option<usize>
	where
		P: Fn(Self::Item) -> bool,
	{
		self.iter_elements().position(predicate)
	}

	#[inline]
	fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
		if self.input_len() >= count {
			Ok(count)
		} else {
			Err(Needed::Unknown)
		}
	}
}

impl<'a> InputLength for TokenStream<'a> {
	#[inline]
	fn input_len(&self) -> usize {
		self.stream.len()
	}
}

impl<'a> InputTake for TokenStream<'a> {
	#[inline]
	fn take(&self, count: usize) -> Self {
		Self::from(&self.stream[..count])
	}

	#[inline]
	fn take_split(&self, count: usize) -> (Self, Self) {
		match self.stream.split_at(count) {
			(l, r) => (Self::from(r), Self::from(l)),
		}
	}
}

impl<'a> UnspecializedInput for TokenStream<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use nom::{bytes::complete::tag, error::ErrorKind};
	use Token::*;

	#[test]
	fn test_tag() {
		let p = tag::<_, _, (_, ErrorKind)>(&Debug);

		let code = TokenStream::from(&[Debug, NewLine][..]);

		assert_eq!(
			p(code),
			Ok((
				TokenStream::from(&[NewLine][..]),
				TokenStream::from(&[Debug][..])
			))
		)
	}
}
