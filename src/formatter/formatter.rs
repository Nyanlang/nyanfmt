// use crate::lexer::Token;
//
// use super::Span;

trait IFormatter<I>
where
	I: Iterator<Item = Self::Input>,
{
	type Input;
	type Output;

	fn get_code(&self) -> I;

	fn format<O>(&mut self, stream_out: &mut Vec<Self::Output>);
}
