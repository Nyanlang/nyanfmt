macro_rules! parse_token {
	($ident: ident: $token: expr => $ast_var: expr => $ast: ty) => {
		fn $ident(input: TokenStream) -> nom::IResult<TokenStream, $ast> {
			nom::combinator::value(
				$ast_var,
				nom::bytes::complete::tag(&$token),
			)(input)
		}
	};
}

macro_rules! match_map {
    ($pat: pat $(if $guard: expr)? => $map: expr) => {
        |o: TokenStream| match o.iter_elements().next()? {
            $pat $(if $guard)? => Some($map),
            _ => None,
        }
    };
}
