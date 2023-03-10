use super::*;
use indoc::indoc;
use nom::{
	error::{Error, ErrorKind},
	Finish,
};
use str_macro::str as s;
use Token::*;

#[test]
fn parse_string() {
	let code = indoc! {r#"
            "주석"냥냥 !냐??
            냐? ~- -"comme" ?냐냐
        "#};

	assert_eq!(
		lex_tokenstream::<Error<_>>(code).finish(),
		Ok((
			"",
			vec![
				Comment(s!("주석")),
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
				Comment(s!("comme")),
				Right,
				Dec,
				Dec,
				NewLine,
			]
		))
	)
}

#[test]
fn parse_string2() {
	let code = r#"냥냥 ,!냐%$#?"#;

	assert_eq!(
		lex_code(code),
		Err(Error::new("%$#?", ErrorKind::Eof))
	)
}
