use super::ast::*;
use crate::lexer::{Token::*, TokenStream};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, value},
	multi::many1,
	IResult,
};

parse_token! { parse_inc: Inc => HeadTok::Inc => HeadTok }
parse_token! { parse_dec: Dec => HeadTok::Dec => HeadTok }
parse_token! { parse_debug: Debug => HeadTok::Debug => HeadTok }
parse_token! { parse_out: Out => BodyTok::Out => HeadTok }
parse_token! { parse_in: In => BodyTok::In => HeadTok }
parse_token! { parse_jump_right: JumpRight => BodyTok::JumpRight => HeadTok }
parse_token! { parse_jump_left: JumpLeft => BodyTok::JumpLeft => HeadTok }
parse_token! { parse_right: Right => TailTok::Right => HeadTok }
parse_token! { parse_left: Left => TailTok::Left => HeadTok }

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

#[cfg(test)]
#[path = "parser.spec.rs"]
mod tests;
