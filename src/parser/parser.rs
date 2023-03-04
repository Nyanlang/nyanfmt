use super::ast::*;
use crate::lexer::{Token::*, TokenStream};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, opt, value, verify},
	multi::many1,
	sequence::tuple,
	IResult,
};

parse_token! { parse_inc: Inc => HeadTok::Inc => HeadTok }
parse_token! { parse_dec: Dec => HeadTok::Dec => HeadTok }
parse_token! { parse_debug: Debug => HeadTok::Debug => HeadTok }
parse_token! { parse_out: Out => BodyTok::Out => BodyTok }
parse_token! { parse_in: In => BodyTok::In => BodyTok }
parse_token! { parse_jump_right: JumpRight => BodyTok::JumpRight => BodyTok }
parse_token! { parse_jump_left: JumpLeft => BodyTok::JumpLeft => BodyTok }
parse_token! { parse_right: Right => TailTok::Right => TailTok }
parse_token! { parse_left: Left => TailTok::Left => TailTok }

fn parse_head(input: TokenStream) -> IResult<TokenStream, Head> {
	map(
		many1(alt((parse_inc, parse_dec, parse_debug))),
		Head,
	)(input)
}

fn parse_body(input: TokenStream) -> IResult<TokenStream, Body> {
	map(
		many1(alt((
			parse_out,
			parse_in,
			parse_jump_right,
			parse_jump_left,
		))),
		Body,
	)(input)
}

fn parse_tail(input: TokenStream) -> IResult<TokenStream, Tail> {
	map(
		many1(alt((parse_right, parse_left))),
		Tail,
	)(input)
}

fn parse_word(input: TokenStream) -> IResult<TokenStream, Word> {
	map(
		verify(
			tuple((
				opt(parse_head),
				opt(parse_body),
				opt(parse_tail),
			)),
			|o| !matches!(o, (None, None, None)),
		),
		|(head, body, tail)| Word { head, body, tail },
	)(input)
}

#[cfg(test)]
#[path = "parser.spec.rs"]
mod tests;
