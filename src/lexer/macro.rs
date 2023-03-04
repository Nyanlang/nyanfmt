macro_rules! char_token {
	($ident: ident: $char: literal -> $tok: expr) => {
		fn $ident(input: &str) -> nom::IResult<&str, Token> {
			nom::combinator::value(
				$tok,
				nom::character::complete::char($char),
			)(input)
		}
	};
}
