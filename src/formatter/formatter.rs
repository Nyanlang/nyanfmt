use crate::lexer::Token;

use super::Span;

trait IFormatter<I>
where
	I: Iterator<Item = Self::Input>,
{
	type Input;
	type Output;

	fn get_code(&self) -> I;

	fn format(&mut self, stream_out: &mut Vec<Self::Output>);
}

struct Formatter<I>
where
	I: Iterator<Item = Token> + Clone,
{
	iter: I,
}

impl<I> IFormatter<I> for Formatter<I>
where
	I: Iterator<Item = Token> + Clone,
{
	type Input = Token;

	type Output = Span;

	fn get_code(&self) -> I {
		self.iter.clone()
	}

	fn format(&mut self, stream_out: &mut Vec<Self::Output>) {
		let mut iter = self.get_code().clone().peekable();

		let mut count = 0;

		while let Some(token) = iter.next() {
			stream_out.push(Span::Token(token.clone()));

			let Some(next) = iter.peek() else {
                break;
            };

			use Token::*;

			match (token, next) {
				(Right | Left, _) if !matches!(next, Right | Left) => {
					stream_out.push(Span::Span)
				},
				(Out | In | JumpRight | JumpLeft, Inc | Dec) => {
					stream_out.push(Span::Span)
				},
				(Inc | Dec, Inc | Dec) => {
					count += 1;
					if count == 5 {
						stream_out.push(Span::Span);
						count = 0;
					}
				},
				(Inc | Dec, _) => {
					count = 0;
				},
				_ => (),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Token::*;

	#[test]
	fn format() {
		let ts = [
			Right,
			JumpRight,
			Inc,
			Comment("com".to_owned()),
			Dec,
			Left,
			JumpRight,
			Dec,
			Dec,
		];

		let mut f = Formatter {
			iter: ts.into_iter(),
		};

		let mut buf = Vec::new();
		f.format(&mut buf);

		assert_eq!(
			buf,
			[
				Span::Token(Right),
				Span::Span,
				Span::Token(JumpRight),
				Span::Span,
				Span::Token(Inc),
				Span::Token(Comment("com".to_owned())),
				Span::Token(Dec),
				Span::Token(Left),
				Span::Span,
				Span::Token(JumpRight),
				Span::Span,
				Span::Token(Dec),
				Span::Token(Dec)
			]
		)
	}
}
