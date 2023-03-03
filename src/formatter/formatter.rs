use crate::lexer::Token;

trait IFormatter<I>
where
	I: Iterator<Item = Token>,
{
	type Output;

	fn format(&mut self, stream_in: I, stream_out: &mut Vec<Self::Output>);
}
