use nom::{
	branch::alt,
	bytes::complete::take_until,
	character::complete::{char, line_ending, multispace0, space0},
	combinator::{map, value},
	multi::{many0, many1},
	sequence::delimited,
	IResult,
};

use super::token::{Token, TokenStream};

char_token! { parse_right: '?' -> Token::Right }
char_token! { parse_left: '!' -> Token::Left }
char_token! { parse_inc: '냥' -> Token::Inc }
char_token! { parse_dec: '냐' -> Token::Dec }
char_token! { parse_out: '.' -> Token::Out }
char_token! { parse_in: ',' -> Token::In }
char_token! { parse_jump_right: '~' -> Token::JumpRight }
char_token! { parse_jump_left: '-' -> Token::JumpLeft }
char_token! { parse_debug: '뀨' -> Token::Debug }

fn parse_comment(input: &str) -> IResult<&str, Token> {
	map(
		delimited(char('"'), take_until(r#"""#), char('"')),
		|o: &str| Token::Comment(o.to_owned()),
	)(input)
}

fn parse_newline(input: &str) -> IResult<&str, Token> {
	value(Token::NewLine, many1(line_ending))(input)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
	alt((
		parse_right,
		parse_left,
		parse_inc,
		parse_dec,
		parse_out,
		parse_in,
		parse_jump_right,
		parse_jump_left,
		parse_debug,
		parse_comment,
		parse_newline,
	))(input)
}

pub fn parse_tokenstream(input: &str) -> IResult<&str, TokenStream> {
	many0(delimited(space0, parse_token, space0))(input)
}

#[cfg(test)]
mod tests {
	use super::*;
	use indoc::indoc;
	use nom::Finish;
	use Token::*;

	#[test]
	fn parse_string() {
		let code = indoc! {r#"
            "주석"냥냥 !냐??
            냐? ~- -"comme" ?냐냐
        "#};

		assert_eq!(
			parse_tokenstream(code).finish(),
			Ok((
				"",
				vec![
					Comment("주석".to_owned()),
					Inc,
					Inc,
					Left,
					Dec,
					Right,
					Right,
					NewLine,
					Dec,
					Right,
					JumpRight,
					JumpLeft,
					JumpLeft,
					Comment("comme".to_owned()),
					Right,
					Dec,
					Dec,
					NewLine,
				]
			))
		)
	}
}
