use super::ast::*;
use crate::lexer::{Token::*, TokenStream};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, value},
	multi::many1,
	IResult,
};

fn parse_inc(input: TokenStream) -> IResult<TokenStream, HeadTok> {
	value(HeadTok::Inc, tag(&Inc))(input)
}

fn parse_dec(input: TokenStream) -> IResult<TokenStream, HeadTok> {
	value(HeadTok::Dec, tag(&Dec))(input)
}

fn parse_debug(input: TokenStream) -> IResult<TokenStream, HeadTok> {
	value(HeadTok::Debug, tag(&Debug))(input)
}

fn parse_out(input: TokenStream) -> IResult<TokenStream, BodyTok> {
	value(BodyTok::Out, tag(&Out))(input)
}

fn parse_in(input: TokenStream) -> IResult<TokenStream, BodyTok> {
	value(BodyTok::In, tag(&In))(input)
}

fn parse_jump_righy(input: TokenStream) -> IResult<TokenStream, BodyTok> {
	value(BodyTok::JumpRight, tag(&JumpRight))(input)
}

fn parse_jump_left(input: TokenStream) -> IResult<TokenStream, BodyTok> {
	value(BodyTok::JumpLeft, tag(&JumpLeft))(input)
}

fn parse_right(input: TokenStream) -> IResult<TokenStream, TailTok> {
	value(TailTok::Right, tag(&Right))(input)
}

fn parse_left(input: TokenStream) -> IResult<TokenStream, TailTok> {
	value(TailTok::Left, tag(&Left))(input)
}

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
			parse_jump_righy,
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
