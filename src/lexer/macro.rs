macro_rules! char_token {
	($ident: ident: $char: literal -> $tok: expr) => {
		fn $ident<'a, E>(input: &'a str) -> nom::IResult<&'a str, Token, E>
		where
			E: nom::error::ParseError<&'a str>,
		{
			nom::combinator::value(
				$tok,
				nom::character::complete::char($char),
			)(input)
		}
	};
}
