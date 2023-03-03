use super::token::Token;
use nom::IResult;

use nom::character::complete::char;
use nom::combinator::value;

fn parse_right(input: &str) -> IResult<&str, Token> {
	value(Token::Right, char('?'))(input)
}
