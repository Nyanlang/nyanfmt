use super::*;
use indoc::indoc;
use nom::{
	combinator::eof,
	error::{Error, ErrorKind},
	sequence::terminated,
	Finish,
};
use pretty_assertions::assert_eq;

type HT = HeadTok;
type BT = BodyTok;
type TT = TailTok;

#[test]
fn parse_head_tokens() {
	let code = ts![Inc, Debug, Inc, Dec];

	assert_eq!(
		parse_head(code),
		Ok((
			TokenStream::new(),
			Head(vec![
				HT::Inc,
				HT::Debug,
				HT::Inc,
				HT::Dec,
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
			Head(vec![HT::Inc, HT::Debug])
		))
	)
}

#[test]
fn must_fail_to_parse_head_if_there_are_no_matched_tokens() {
	let code = ts![Out, Inc, Debug, Right, Inc, Dec];

	assert_eq!(
		parse_head(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
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
				BT::Out,
				BT::JumpRight,
				BT::JumpRight,
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
				BT::Out,
				BT::JumpRight,
				BT::JumpRight,
				BT::In,
				BT::JumpLeft,
				BT::Out,
			])
		))
	)
}

#[test]
fn must_fail_to_parse_body_if_there_are_no_matched_tokens() {
	let code = ts![Debug, Out, JumpRight, JumpRight, In, JumpLeft, Out];

	assert_eq!(
		parse_body(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
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
				TT::Right,
				TT::Left,
				TT::Right,
				TT::Right,
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
			Tail(vec![TT::Right, TT::Left])
		))
	)
}

#[test]
fn must_fail_to_parse_tail_if_there_are_no_matched_tokens() {
	let code = ts![Out, Right, Left, JumpRight, Right, Right];

	assert_eq!(
		parse_tail(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
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
					BT::Out,
					BT::JumpRight,
					BT::In,
					BT::JumpLeft,
				],
				[TT::Left],
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
				[HT::Debug, HT::Inc],
				[BT::JumpRight, BT::In, BT::JumpLeft],
				[TT::Left],
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
			word!([HT::Inc, HT::Inc],,)
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
			word!(, [BT::Out, BT::JumpLeft],),
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
				[TT::Left, TT::Left]
			)
		))
	)
}

#[test]
fn must_fail_to_parse_word_if_input_is_empty() {
	let code = TokenStream::new();

	assert_eq!(
		parse_word(code).finish(),
		Err(Error::new(
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
			sentence![
				word!(
					[HT::Inc, HT::Dec],
					[BT::Out, BT::In],
					[TT::Right, TT::Left],
				),
				word!(
					[HT::Inc, HT::Inc, HT::Debug],
					[BT::Out, BT::JumpLeft, BT::JumpRight],
					[TT::Left, TT::Left, TT::Left],
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
		Ok((TokenStream::new(), sentence![]))
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
			sentence![
				word!(
					[HT::Inc, HT::Dec],
					[BT::Out, BT::In],
					[TT::Right, TT::Left],
				),
				word!(
					[HT::Inc, HT::Inc, HT::Debug],
					[BT::Out, BT::JumpLeft, BT::JumpRight],
					[TT::Left, TT::Left, TT::Left],
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
		Err(Error::new(
			TokenStream::new(),
			ErrorKind::Verify
		))
	);
}

#[test]
fn must_match_without_any_newlines() {
	let code = ts![Debug, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_with_newline_at_the_left() {
	let code = ts![NewLine, Debug, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_with_newline_at_the_right() {
	let code = ts![Debug, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_newlines_at_both_side() {
	let code = ts![NewLine, Debug, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_fail_if_inner_parser_fails() {
	let code = ts![NewLine, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code).finish(),
		Err(Error::new(
			ts![NewLine, Left],
			ErrorKind::Tag
		)),
	);
}

#[test]
fn newlines_should_not_be_repeated() {
	let code = ts![
		NewLine, NewLine, In, NewLine, NewLine, NewLine, NewLine, In, NewLine
	];

	assert_eq!(
		terminated(many0(pad_newline(tag(&In))), eof)(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Eof)),
	);
}

#[test]
fn must_match_with_newline_separated_tokens() {
	let code = ts![NewLine, In, NewLine, NewLine, In, NewLine];

	assert_eq!(
		terminated(many0(pad_newline(tag(&In))), eof)(code.clone()).finish(),
		Ok((ts![], vec![ts![In], ts![In]]))
	);
}

#[test]
fn sentences0_must_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_sentences0(code),
		Ok((ts![], vec![]))
	);
}

#[test]
fn sentences0_must_recognize_single_word_as_single_sentence() {
	let code = ts![In];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![sentence![word!(, [BT::In],)]]
		))
	);
}

#[test]
fn sentences0_must_recognize_many_words_as_single_sentence() {
	let code = ts![In, JumpLeft, Debug, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![sentence![
				word!(, [BT::In, BT::JumpLeft],),
				word!([HT::Debug], [BT::Out],),
			]]
		))
	);
}

#[test]
fn sentences0_must_recognize_many_words_separated_with_newlines_as_multiple_sentence(
) {
	let code = ts![In, NewLine, JumpLeft, In, Debug, NewLine, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				sentence![word!(, [BT::In, ],),],
				sentence![
					word!(, [BT::JumpLeft, BT::In],),
					word!([HT::Debug],,),
				],
				sentence![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences0_must_ignore_trailing_newline() {
	let code = ts![In, NewLine, JumpLeft, In, Debug, NewLine, Out, NewLine];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				sentence![word!(, [BT::In, ],),],
				sentence![
					word!(, [BT::JumpLeft, BT::In],),
					word!([HT::Debug],,),
				],
				sentence![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences0_must_ignore_leading_newline() {
	let code = ts![NewLine, In, NewLine, JumpLeft, In, Debug, NewLine, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				sentence![word!(, [BT::In, ],),],
				sentence![
					word!(, [BT::JumpLeft, BT::In],),
					word!([HT::Debug],,),
				],
				sentence![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences1_must_fail_to_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_sentences1(code).finish(),
		Err(Error::new(ts![], ErrorKind::Verify))
	);
}

#[test]
fn sentences1_must_fail_to_match_with_only_newlines() {
	let code = ts![NewLine];

	assert_eq!(
		parse_sentences1(code).finish(),
		Err(Error::new(ts![], ErrorKind::Verify))
	);
}

#[test]
fn test_match_map_macro_invocation() {
	fn parse_inc_and_dec_as_debug(
		input: TokenStream,
	) -> IResult<TokenStream, HeadTok> {
		map_one(match_map! { Inc | Dec => HT::Debug })(input)
	}

	assert_eq!(
		parse_inc_and_dec_as_debug(ts![Inc]).finish(),
		Ok((ts![], HeadTok::Debug))
	);

	assert_eq!(
		parse_inc_and_dec_as_debug(ts![Dec]).finish(),
		Ok((ts![], HeadTok::Debug))
	);

	assert_eq!(
		parse_inc_and_dec_as_debug(ts![Debug]).finish(),
		Err(Error::new(
			ts![Debug],
			ErrorKind::MapOpt
		))
	);
}

#[test]
fn test_parse_comment1() {
	let code = ts![];

	assert_eq!(
		parse_comment(code).finish(),
		Err(Error::new(ts![], ErrorKind::Eof))
	)
}

#[test]
fn test_parse_comment2() {
	let code = ts![In];

	assert_eq!(
		parse_comment(code).finish(),
		Err(Error::new(ts![In], ErrorKind::MapOpt))
	)
}

#[test]
fn test_parse_comment3() {
	let sl = &[Token::Comment("".to_owned())][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_comment(code).finish(),
		Ok((ts![], ast::Comment("".to_owned())))
	)
}

#[test]
fn test_parse_comment4() {
	let sl = &[Token::Comment("hello".to_owned())][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_comment(code).finish(),
		Ok((ts![], ast::Comment("hello".to_owned())))
	)
}

#[test]
fn parse_comments0_must_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_comments0(code),
		Ok((ts![], vec![]))
	)
}

#[test]
fn parse_comments0_must_match_with_many_consequent_comments() {
	let sl = &[
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		Token::Comment("ents".to_owned()),
		JumpLeft,
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_comments0(code),
		Ok((
			ts![JumpLeft],
			vec![
				ast::Comment("co".to_owned()),
				ast::Comment("mm".to_owned()),
				ast::Comment("ents".to_owned()),
			]
		))
	)
}

#[test]
fn parse_comments1_must_not_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_comments1(code).finish(),
		Err(Error::new(ts![], ErrorKind::Eof))
	)
}

#[test]
fn parse_comments1_must_match_with_many_consequent_comments() {
	let sl = &[
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		Token::Comment("ents".to_owned()),
		JumpLeft,
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_comments1(code),
		Ok((
			ts![JumpLeft],
			vec![
				ast::Comment("co".to_owned()),
				ast::Comment("mm".to_owned()),
				ast::Comment("ents".to_owned()),
			]
		))
	)
}

#[test]
fn parse_paragraph_must_not_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_paragraph(code).finish(),
		Err(Error::new(ts![], ErrorKind::Eof))
	)
}

#[test]
fn parse_paragraph_must_not_match_with_single_comment() {
	let sl = &[Token::Comment("co".to_owned())][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_paragraph(code).finish(),
		Err(Error::new(ts![], ErrorKind::Verify))
	)
}

#[test]
fn parse_paragraph_must_not_match_with_single_sentence() {
	let code = ts![JumpLeft];

	assert_eq!(
		parse_paragraph(code).finish(),
		Err(Error::new(
			ts![JumpLeft],
			ErrorKind::MapOpt
		))
	)
}

#[test]
fn parse_paragraph_must_match_with_comment_and_sentence() {
	let sl = &[Token::Comment("co".to_owned()), JumpLeft][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_paragraph(code).finish(),
		Ok((
			ts![],
			Paragraph(
				vec![ast::Comment("co".to_owned())],
				vec![sentence![word!(, [BT::JumpLeft],)]]
			)
		))
	)
}

#[test]
fn parse_code_must_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![],
				paragraphs: vec![],
				trailing_comments: vec![],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_only_comments() {
	let sl = &[
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		Token::Comment("ents".to_owned()),
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![],
				paragraphs: vec![],
				trailing_comments: vec![
					ast::Comment("co".to_owned()),
					ast::Comment("mm".to_owned()),
					ast::Comment("ents".to_owned())
				],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_only_sentences() {
	let code = ts![JumpLeft, In, NewLine, Right, Debug, Out, JumpRight];

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![
					sentence![word!(, [BT::JumpLeft, BT::In],)],
					sentence![
						word!(,, [TT::Right]),
						word!([HT::Debug], [BT::Out, BT::JumpRight],)
					],
				],
				paragraphs: vec![],
				trailing_comments: vec![],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_sentences_and_comments() {
	let sl = &[
		JumpLeft,
		In,
		NewLine,
		Right,
		Debug,
		Out,
		JumpRight,
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![
					sentence![word!(, [BT::JumpLeft, BT::In],)],
					sentence![
						word!(,, [TT::Right]),
						word!([HT::Debug], [BT::Out, BT::JumpRight],)
					],
				],
				paragraphs: vec![],
				trailing_comments: vec![
					ast::Comment("co".to_owned()),
					ast::Comment("mm".to_owned()),
				],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_comments_and_sentences() {
	let sl = &[
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		JumpLeft,
		In,
		NewLine,
		Right,
		Debug,
		Out,
		JumpRight,
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![],
				paragraphs: vec![Paragraph(
					vec![
						ast::Comment("co".to_owned()),
						ast::Comment("mm".to_owned()),
					],
					vec![
						sentence![word!(, [BT::JumpLeft, BT::In],)],
						sentence![
							word!(,, [TT::Right]),
							word!([HT::Debug], [BT::Out, BT::JumpRight],)
						],
					]
				)],
				trailing_comments: vec![],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_c_s() {
	let sl = &[
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		JumpLeft,
		In,
		NewLine,
		Right,
		Token::Comment("en".to_owned()),
		Debug,
		Out,
		Token::Comment("ts".to_owned()),
		JumpRight,
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![],
				paragraphs: vec![
					Paragraph(
						vec![
							ast::Comment("co".to_owned()),
							ast::Comment("mm".to_owned()),
						],
						vec![
							sentence![word!(, [BT::JumpLeft, BT::In],)],
							sentence![word!(,, [TT::Right]),],
						]
					),
					Paragraph(
						vec![ast::Comment("en".to_owned()),],
						vec![sentence![word!([HT::Debug], [BT::Out],)]],
					),
					Paragraph(
						vec![ast::Comment("ts".to_owned()),],
						vec![sentence![word!(, [BT::JumpRight],)]]
					)
				],
				trailing_comments: vec![],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_s_c() {
	let sl = &[
		JumpLeft,
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		In,
		NewLine,
		Right,
		Token::Comment("en".to_owned()),
		Debug,
		Out,
		NewLine,
		JumpRight,
		Token::Comment("ts".to_owned()),
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![sentence![word!(, [BT::JumpLeft],)]],
				paragraphs: vec![
					Paragraph(
						vec![
							ast::Comment("co".to_owned()),
							ast::Comment("mm".to_owned()),
						],
						vec![
							sentence![word!(, [BT::In],)],
							sentence![word!(,, [TT::Right]),],
						]
					),
					Paragraph(
						vec![ast::Comment("en".to_owned()),],
						vec![
							sentence![word!([HT::Debug], [BT::Out],)],
							sentence![word!(, [BT::JumpRight],)],
						],
					),
				],
				trailing_comments: vec![ast::Comment("ts".to_owned())],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_c_c() {
	let sl = &[
		Token::Comment("hello".to_owned()),
		JumpLeft,
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		In,
		NewLine,
		Right,
		Token::Comment("en".to_owned()),
		Debug,
		Out,
		NewLine,
		JumpRight,
		Token::Comment("ts".to_owned()),
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![],
				paragraphs: vec![
					Paragraph(
						vec![ast::Comment("hello".to_owned()),],
						vec![sentence![word!(, [BT::JumpLeft],)]]
					),
					Paragraph(
						vec![
							ast::Comment("co".to_owned()),
							ast::Comment("mm".to_owned()),
						],
						vec![
							sentence![word!(, [BT::In],)],
							sentence![word!(,, [TT::Right]),],
						]
					),
					Paragraph(
						vec![ast::Comment("en".to_owned()),],
						vec![
							sentence![word!([HT::Debug], [BT::Out],)],
							sentence![word!(, [BT::JumpRight],)],
						],
					),
				],
				trailing_comments: vec![ast::Comment("ts".to_owned())],
			}
		))
	)
}

#[test]
fn parse_code_must_match_with_s_s() {
	let sl = &[
		JumpLeft,
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		In,
		NewLine,
		Right,
		Token::Comment("en".to_owned()),
		Debug,
		Out,
		NewLine,
		JumpRight,
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_code(code),
		Ok((
			ts![],
			Code {
				leading_sentences: vec![sentence![word!(, [BT::JumpLeft],)]],
				paragraphs: vec![
					Paragraph(
						vec![
							ast::Comment("co".to_owned()),
							ast::Comment("mm".to_owned()),
						],
						vec![
							sentence![word!(, [BT::In],)],
							sentence![word!(,, [TT::Right]),],
						]
					),
					Paragraph(
						vec![ast::Comment("en".to_owned()),],
						vec![
							sentence![word!([HT::Debug], [BT::Out],)],
							sentence![word!(, [BT::JumpRight],)],
						],
					),
				],
				trailing_comments: vec![],
			}
		))
	)
}

#[test]
fn parse_root_must_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_root(code),
		Ok((
			ts![],
			Root(Code {
				leading_sentences: vec![],
				paragraphs: vec![],
				trailing_comments: vec![]
			})
		))
	)
}

#[test]
fn test_parse_root() {
	let sl = &[
		JumpLeft,
		Token::Comment("co".to_owned()),
		Token::Comment("mm".to_owned()),
		In,
		NewLine,
		Right,
		Token::Comment("en".to_owned()),
		Debug,
		Out,
		NewLine,
		JumpRight,
		Token::Comment("ts".to_owned()),
	][..];
	let code = TokenStream::from(sl);

	assert_eq!(
		parse_root(code),
		Ok((
			ts![],
			Root(Code {
				leading_sentences: vec![sentence![word!(, [BT::JumpLeft],)]],
				paragraphs: vec![
					Paragraph(
						vec![
							ast::Comment("co".to_owned()),
							ast::Comment("mm".to_owned()),
						],
						vec![
							sentence![word!(, [BT::In],)],
							sentence![word!(,, [TT::Right]),],
						]
					),
					Paragraph(
						vec![ast::Comment("en".to_owned()),],
						vec![
							sentence![word!([HT::Debug], [BT::Out],)],
							sentence![word!(, [BT::JumpRight],)],
						],
					),
				],
				trailing_comments: vec![ast::Comment("ts".to_owned())],
			})
		))
	)
}
