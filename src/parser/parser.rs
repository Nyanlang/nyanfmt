use super::ast::{self, *};
use crate::lexer::{
	Token::{self, *},
	TokenStream,
};
use nom::{
	branch::alt,
	bytes::complete::{tag, take},
	combinator::{eof, map, map_opt, opt, verify},
	error::{Error, ParseError},
	multi::{many0, many1},
	sequence::{delimited, pair, terminated, tuple},
	Finish, IResult, InputIter, InputTake, Parser,
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

fn parse_head<'a, E>(
	input: TokenStream<'a>,
) -> IResult<TokenStream<'a>, Head, E>
where
	E: ParseError<TokenStream<'a>>,
{
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

fn parse_words0(input: TokenStream) -> IResult<TokenStream, Sentence> {
	map(many0(parse_word), Sentence)(input)
}

fn parse_words1(input: TokenStream) -> IResult<TokenStream, Sentence> {
	map(many1(parse_word), Sentence)(input)
}

fn pad_newline<'a, O, F>(
	parser: F,
) -> impl FnMut(TokenStream<'a>) -> IResult<TokenStream<'a>, O>
where
	F: Parser<TokenStream<'a>, O, Error<TokenStream<'a>>>,
{
	delimited(
		opt(tag(&NewLine)),
		parser,
		opt(tag(&NewLine)),
	)
}

fn parse_sentences0(input: TokenStream) -> IResult<TokenStream, Vec<Sentence>> {
	many0(pad_newline(parse_words1))(input)
}

fn parse_sentences1(input: TokenStream) -> IResult<TokenStream, Vec<Sentence>> {
	many1(pad_newline(parse_words1))(input)
}

fn map_one<I, O, E, G>(f: G) -> impl FnOnce(I) -> IResult<I, O, E>
where
	G: FnMut(I) -> Option<O>,
	I: InputIter + InputTake + Clone,
	E: ParseError<I>,
{
	map_opt(take(1usize), f)
}

fn parse_comment(input: TokenStream) -> IResult<TokenStream, ast::Comment> {
	map_one(match_map! { Token::Comment(s) => ast::Comment(s.clone()) })(input)
}

fn parse_comments0(
	input: TokenStream,
) -> IResult<TokenStream, Vec<ast::Comment>> {
	many0(parse_comment)(input)
}

fn parse_comments1(
	input: TokenStream,
) -> IResult<TokenStream, Vec<ast::Comment>> {
	many1(parse_comment)(input)
}

fn parse_paragraph(input: TokenStream) -> IResult<TokenStream, Paragraph> {
	map(
		pair(parse_comments1, parse_sentences1),
		|(c, s)| Paragraph(c, s),
	)(input)
}

fn parse_code(input: TokenStream) -> IResult<TokenStream, Code> {
	map(
		tuple((
			parse_sentences0,
			many0(parse_paragraph),
			parse_comments0,
		)),
		|(leading_sentences, paragraphs, trailing_comments)| Code {
			leading_sentences,
			paragraphs,
			trailing_comments,
		},
	)(input)
}

fn parse_root(input: TokenStream) -> IResult<TokenStream, Root> {
	map(terminated(parse_code, eof), Root)(input)
}

pub fn parse_ast(input: TokenStream) -> Result<Root, Error<TokenStream>> {
	let (_, o) = parse_root(input).finish()?;

	Ok(o)
}

#[cfg(test)]
#[path = "parser.spec.rs"]
mod tests;
