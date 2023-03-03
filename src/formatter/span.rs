use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Span {
	Token(Token),
	Span,
}
