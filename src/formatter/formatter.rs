use crate::lexer::Token;

use super::Span;

trait IFormatter<I>
where
	I: Iterator<Item = Self::Input>,
{
	type Input;
	type Output;

	fn get_code(&self) -> &I;

	fn format<O>(&mut self, stream_out: &mut Vec<Self::Output>);
}

struct Formatter<I>
where
	I: Iterator<Item = Token>,
{
	iter: I,
}

impl<I> IFormatter<I> for Formatter<I>
where
	I: Iterator<Item = Token>,
{
	type Input = Token;

	type Output = Span;

	fn get_code(&self) -> &I {
		&self.iter
	}

	fn format<O>(&mut self, stream_out: &mut Vec<Self::Output>) {
		todo!()
	}
}
