use super::*;
use nom::{error::ErrorKind, Finish};

#[test]
fn parse_head_tokens() {
	let code = TokenStream::from(&[Inc, Debug, Inc, Dec][..]);

	assert_eq!(
		parse_head(code),
		Ok((
			TokenStream::new(),
			Head(vec![
				HeadTok::Inc,
				HeadTok::Debug,
				HeadTok::Inc,
				HeadTok::Dec,
			])
		))
	)
}

#[test]
fn parse_head_tokens2() {
	let code = TokenStream::from(&[Inc, Debug, Right, Inc, Dec][..]);

	assert_eq!(
		parse_head(code),
		Ok((
			TokenStream::from(&[Right, Inc, Dec,][..]),
			Head(vec![HeadTok::Inc, HeadTok::Debug,])
		))
	)
}

#[test]
fn must_fail_to_parse_head_if_there_are_no_matched_tokens() {
	let code = TokenStream::from(&[Out, Inc, Debug, Right, Inc, Dec][..]);

	assert_eq!(
		parse_head(code.clone()).finish(),
		Err(nom::error::Error::new(
			code,
			ErrorKind::Tag
		))
	);
}

#[test]
fn parse_body_tokens() {
	let code = TokenStream::from(
		&[Out, JumpRight, JumpRight, Dec, In, JumpLeft, Out][..],
	);

	assert_eq!(
		parse_body(code),
		Ok((
			TokenStream::from(&[Dec, In, JumpLeft, Out][..]),
			Body(vec![
				BodyTok::Out,
				BodyTok::JumpRight,
				BodyTok::JumpRight,
			])
		))
	)
}

#[test]
fn parse_body_tokens2() {
	let code =
		TokenStream::from(&[Out, JumpRight, JumpRight, In, JumpLeft, Out][..]);

	assert_eq!(
		parse_body(code),
		Ok((
			TokenStream::new(),
			Body(vec![
				BodyTok::Out,
				BodyTok::JumpRight,
				BodyTok::JumpRight,
				BodyTok::In,
				BodyTok::JumpLeft,
				BodyTok::Out
			])
		))
	)
}

#[test]
fn must_fail_to_parse_body_if_there_are_no_matched_tokens() {
	let code = TokenStream::from(
		&[Debug, Out, JumpRight, JumpRight, In, JumpLeft, Out][..],
	);

	assert_eq!(
		parse_body(code.clone()).finish(),
		Err(nom::error::Error::new(
			code,
			ErrorKind::Tag
		))
	);
}

#[test]
fn parse_tail_tokens() {
	let code = TokenStream::from(&[Right, Left, Right, Right][..]);

	assert_eq!(
		parse_tail(code),
		Ok((
			TokenStream::new(),
			Tail(vec![
				TailTok::Right,
				TailTok::Left,
				TailTok::Right,
				TailTok::Right,
			])
		))
	)
}

#[test]
fn parse_tail_tokens2() {
	let code = TokenStream::from(&[Right, Left, JumpRight, Right, Right][..]);

	assert_eq!(
		parse_tail(code),
		Ok((
			TokenStream::from(&[JumpRight, Right, Right][..]),
			Tail(vec![TailTok::Right, TailTok::Left,])
		))
	)
}

#[test]
fn must_fail_to_parse_tail_if_there_are_no_matched_tokens() {
	let code =
		TokenStream::from(&[Out, Right, Left, JumpRight, Right, Right][..]);

	assert_eq!(
		parse_tail(code.clone()).finish(),
		Err(nom::error::Error::new(
			code,
			ErrorKind::Tag
		))
	);
}

#[test]
fn must_parse_word() {
	let code = TokenStream::from(
		&[Out, JumpRight, In, JumpLeft, Left, Out, Debug][..],
	);

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::from(&[Out, Debug][..]),
			Word {
				head: None,
				body: Some(Body(vec![
					BodyTok::Out,
					BodyTok::JumpRight,
					BodyTok::In,
					BodyTok::JumpLeft
				])),
				tail: Some(Tail(vec![TailTok::Left]))
			}
		))
	)
}

#[test]
fn must_parse_word2() {
	let code = TokenStream::from(
		&[Debug, Inc, JumpRight, In, JumpLeft, Left, Out, Debug][..],
	);

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::from(&[Out, Debug][..]),
			Word {
				head: Some(Head(
					vec![HeadTok::Debug, HeadTok::Inc,]
				)),
				body: Some(Body(vec![
					BodyTok::JumpRight,
					BodyTok::In,
					BodyTok::JumpLeft
				])),
				tail: Some(Tail(vec![TailTok::Left]))
			}
		))
	)
}

#[test]
fn parse_word_only_match_head() {
	let code = TokenStream::from(&[Inc, Inc][..]);

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::new(),
			Word {
				head: Some(Head(vec![HeadTok::Inc, HeadTok::Inc,])),
				body: None,
				tail: None
			}
		))
	)
}

#[test]
fn parse_word_only_match_body() {
	let code = TokenStream::from(&[Out, JumpLeft, Dec, JumpLeft][..]);

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::from(&[Dec, JumpLeft][..]),
			Word {
				head: None,
				body: Some(Body(vec![
					BodyTok::Out,
					BodyTok::JumpLeft
				])),
				tail: None
			}
		))
	)
}

#[test]
fn parse_word_only_match_tail() {
	let code = TokenStream::from(&[Left, Left, Debug][..]);

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::from(&[Debug][..]),
			Word {
				head: None,
				body: None,
				tail: Some(Tail(vec![TailTok::Left, TailTok::Left]))
			}
		))
	)
}

#[test]
fn must_fail_to_parse_word_if_input_is_empty() {
	let code = TokenStream::new();

	assert_eq!(
		parse_word(code).finish(),
		Err(nom::error::Error::new(
			TokenStream::new(),
			ErrorKind::Verify
		))
	);
}
