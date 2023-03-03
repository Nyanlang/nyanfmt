use nom::IResult;

use super::token::Token;
use nom::branch::alt;

char_token! { parse_right: '?' -> Token::Right }
char_token! { parse_left: '!' -> Token::Left }
char_token! { parse_inc: '냥' -> Token::Inc }
char_token! { parse_dec: '냐' -> Token::Dec }
char_token! { parse_out: '.' -> Token::Out }
char_token! { parse_in: ',' -> Token::In }
char_token! { parse_jump_right: '~' -> Token::JumpRight }
char_token! { parse_jump_left: '-' -> Token::JumpLeft }
char_token! { parse_debug: '뀨' -> Token::Debug }

pub fn parse_token(input: &str) -> IResult<&str, Token> {
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
	))(input)
}
