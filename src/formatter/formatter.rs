use crate::lexer::Token;

use super::OutStream;

trait IFormatter {
	fn format<I>(&mut self, stream_in: I, stream_out: &mut OutStream)
	where
		I: Iterator<Item = Token>;
}
