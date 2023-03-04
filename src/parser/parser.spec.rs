use super::*;
use nom::{error::ErrorKind, Finish};
use pretty_assertions::assert_eq;

#[test]
fn parse_head_tokens() {
	let code = ts![Inc, Debug, Inc, Dec];

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
	let code = ts![Inc, Debug, Right, Inc, Dec];

	assert_eq!(
		parse_head(code),
		Ok((
			ts![Right, Inc, Dec],
			Head(vec![HeadTok::Inc, HeadTok::Debug])
		))
	)
}

#[test]
fn must_fail_to_parse_head_if_there_are_no_matched_tokens() {
	let code = ts![Out, Inc, Debug, Right, Inc, Dec];

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
	let code = ts![Out, JumpRight, JumpRight, Dec, In, JumpLeft, Out];

	assert_eq!(
		parse_body(code),
		Ok((
			ts![Dec, In, JumpLeft, Out],
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
	let code = ts![Out, JumpRight, JumpRight, In, JumpLeft, Out];

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
	let code = ts![Debug, Out, JumpRight, JumpRight, In, JumpLeft, Out];

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
	let code = ts![Right, Left, Right, Right];

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
	let code = ts![Right, Left, JumpRight, Right, Right];

	assert_eq!(
		parse_tail(code),
		Ok((
			ts![JumpRight, Right, Right],
			Tail(vec![TailTok::Right, TailTok::Left])
		))
	)
}

#[test]
fn must_fail_to_parse_tail_if_there_are_no_matched_tokens() {
	let code = ts![Out, Right, Left, JumpRight, Right, Right];

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
	let code = ts![Out, JumpRight, In, JumpLeft, Left, Out, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Out, Debug],
			word!(
				,
				[
					BodyTok::Out,
					BodyTok::JumpRight,
					BodyTok::In,
					BodyTok::JumpLeft
				],
				[TailTok::Left],
			)
		))
	)
}

#[test]
fn must_parse_word2() {
	let code = ts![Debug, Inc, JumpRight, In, JumpLeft, Left, Out, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Out, Debug],
			word!(
				[HeadTok::Debug, HeadTok::Inc],
				[BodyTok::JumpRight, BodyTok::In, BodyTok::JumpLeft],
				[TailTok::Left],
			)
		))
	)
}

#[test]
fn parse_word_only_match_head() {
	let code = ts![Inc, Inc];

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::new(),
			word!([HeadTok::Inc, HeadTok::Inc],,)
		))
	)
}

#[test]
fn parse_word_only_match_body() {
	let code = ts![Out, JumpLeft, Dec, JumpLeft];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Dec, JumpLeft],
			word!(, [BodyTok::Out, BodyTok::JumpLeft],),
		))
	)
}

#[test]
fn parse_word_only_match_tail() {
	let code = ts![Left, Left, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Debug],
			word!(
				,,
				[TailTok::Left, TailTok::Left]
			)
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

#[test]
fn test_parse_words0() {
	let code = ts![
		Inc, Dec, Out, In, Right, Left, Inc, Inc, Debug, Out, JumpLeft,
		JumpRight, Left, Left, Left,
	];

	assert_eq!(
		parse_words0(code),
		Ok((
			TokenStream::new(),
			vec![
				word!(
					[HeadTok::Inc, HeadTok::Dec],
					[BodyTok::Out, BodyTok::In],
					[TailTok::Right, TailTok::Left],
				),
				word!(
					[HeadTok::Inc, HeadTok::Inc, HeadTok::Debug],
					[BodyTok::Out, BodyTok::JumpLeft, BodyTok::JumpRight],
					[TailTok::Left, TailTok::Left, TailTok::Left],
				)
			]
		))
	)
}

#[test]
fn test_parse_words0_with_empty_input() {
	let code = TokenStream::new();

	assert_eq!(
		parse_words0(code),
		Ok((TokenStream::new(), vec![]))
	)
}

#[test]
fn test_parse_words1() {
	let code = ts![
		Inc, Dec, Out, In, Right, Left, Inc, Inc, Debug, Out, JumpLeft,
		JumpRight, Left, Left, Left,
	];

	assert_eq!(
		parse_words1(code),
		Ok((
			TokenStream::new(),
			vec![
				word!(
					[HeadTok::Inc, HeadTok::Dec],
					[BodyTok::Out, BodyTok::In],
					[TailTok::Right, TailTok::Left],
				),
				word!(
					[HeadTok::Inc, HeadTok::Inc, HeadTok::Debug],
					[BodyTok::Out, BodyTok::JumpLeft, BodyTok::JumpRight],
					[TailTok::Left, TailTok::Left, TailTok::Left],
				),
			]
		))
	)
}

#[test]
fn test_parse_words1_with_empty_input() {
	let code = TokenStream::new();

	assert_eq!(
		parse_words1(code).finish(),
		Err(nom::error::Error::new(
			TokenStream::new(),
			ErrorKind::Verify
		))
	);
}
