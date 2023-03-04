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
