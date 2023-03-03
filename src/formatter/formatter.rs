// use crate::lexer::Token;
//
// use super::Span;

trait IFormatter {
	type Input;
	type Output;

	fn format<I, O>(
		&mut self,
		stream_in: I,
		stream_out: &mut Vec<Self::Output>,
	) where
		I: Iterator<Item = Self::Input>;
}
