use crate::lexer::Token;

pub enum Span {
	Token(Token),
	Span,
}
