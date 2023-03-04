use nom::{bytes::complete::is_a, combinator::map, IResult, InputIter};

use super::ast::*;
use crate::lexer::{Token::*, TokenStream};

fn parse_head(input: TokenStream) -> IResult<TokenStream, Head> {
	map(
		is_a(TokenStream::from(
			&[Inc, Dec, Debug][..],
		)),
		|o: TokenStream| {
			Head(
				o.iter_elements()
					.map(|t| match t {
						Inc => HeadTok::Inc,
						Dec => HeadTok::Dec,
						Debug => HeadTok::Debug,
						_ => unreachable!(),
					})
					.collect::<Vec<_>>(),
			)
		},
	)(input)
}

fn parse_body(input: TokenStream) -> IResult<TokenStream, Body> {
	map(
		is_a(TokenStream::from(
			&[Out, In, JumpRight, JumpLeft][..],
		)),
		|o: TokenStream| {
			Body(
				o.iter_elements()
					.map(|t| match t {
						Out => BodyTok::Out,
						In => BodyTok::In,
						JumpRight => BodyTok::JumpRight,
						JumpLeft => BodyTok::JumpLeft,
						_ => unreachable!(),
					})
					.collect::<Vec<_>>(),
			)
		},
	)(input)
}

fn parse_tail(input: TokenStream) -> IResult<TokenStream, Tail> {
	map(
		is_a(TokenStream::from(&[Right, Left][..])),
		|o: TokenStream| {
			Tail(
				o.iter_elements()
					.map(|t| match t {
						Right => TailTok::Right,
						Left => TailTok::Left,
						_ => unreachable!(),
					})
					.collect::<Vec<_>>(),
			)
		},
	)(input)
}
