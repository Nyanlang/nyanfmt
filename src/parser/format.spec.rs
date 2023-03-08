use super::*;
use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn format_head() {
	let ast = Head(vec![
		HeadTok::Inc,
		HeadTok::Debug,
		HeadTok::Dec,
		HeadTok::Dec,
		HeadTok::Inc,
	]);

	assert_eq!(ast.to_string(), "냥뀨냐냐냥");
}

#[test]
fn format_body() {
	let ast = Body(vec![
		BodyTok::JumpLeft,
		BodyTok::In,
		BodyTok::Out,
		BodyTok::JumpRight,
		BodyTok::JumpLeft,
		BodyTok::Out,
		BodyTok::Out,
		BodyTok::JumpLeft,
		BodyTok::In,
		BodyTok::JumpRight,
	]);

	assert_eq!(ast.to_string(), "-,.~-..-,~");
}

#[test]
fn format_tail() {
	let ast = Tail(vec![
		TailTok::Right,
		TailTok::Left,
		TailTok::Right,
		TailTok::Right,
		TailTok::Left,
		TailTok::Left,
		TailTok::Left,
		TailTok::Right,
		TailTok::Right,
	]);

	assert_eq!(ast.to_string(), "?!??!!!??");
}

#[test]
fn format_word() {
	let ast = word!(
		[HeadTok::Inc, HeadTok::Debug],
		[
			BodyTok::JumpRight,
			BodyTok::In,
			BodyTok::Out,
			BodyTok::JumpLeft,
		],
		[
			TailTok::Right,
			TailTok::Left,
			TailTok::Right,
			TailTok::Right,
		]
	);

	assert_eq!(ast.to_string(), "냥뀨~,.-?!??");
}

#[test]
fn format_sentence() {
	let ast = sentence![
		word!(
			[HeadTok::Inc],
			[BodyTok::In, BodyTok::Out],
			[TailTok::Right, TailTok::Right],
		),
		word!(
			[HeadTok::Dec, HeadTok::Inc],
			[BodyTok::JumpLeft],
		),
		word!(, [
            BodyTok::JumpRight,
            BodyTok::JumpRight,
            BodyTok::In,
        ], [TailTok::Left, TailTok::Right]),
		word!([HeadTok::Debug],, [TailTok::Right]),
		word!(, [BodyTok::Out, BodyTok::JumpRight],),
	];

	assert_eq!(
		ast.to_string(),
		"냥,.?? 냐냥- ~~,!? 뀨? .~"
	)
}

#[test]
fn format_comment() {
	let ast = Comment("hello".to_owned());

	assert_eq!(ast.to_string(), r#""hello""#);
}

#[test]
fn format_paragraph_with_single_comment_and_single_sentence() {
	let ast = Paragraph(
		vec![Comment("comment".to_owned())],
		vec![sentence![word!([HeadTok::Inc],,)]],
	);

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "comment"
            냥"#
		}
	);
}

#[test]
fn format_paragraph_with_multiple_comment_and_single_sentence() {
	let ast = Paragraph(
		vec![
			Comment("this".to_owned()),
			Comment("is".to_owned()),
			Comment("very".to_owned()),
			Comment("long".to_owned()),
			Comment("comment".to_owned()),
		],
		vec![sentence![word!([HeadTok::Inc],,)]],
	);

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "this"
            "is"
            "very"
            "long"
            "comment"
            냥"#
		}
	);
}

#[test]
fn format_paragraph_with_single_comment_and_multiple_sentence() {
	let ast = Paragraph(
		vec![Comment("comment".to_owned())],
		vec![
			sentence![
				word!([HeadTok::Inc],,[TailTok::Right]),
				word!(, [BodyTok::JumpRight],)
			],
			sentence![word!([HeadTok::Inc],, [TailTok::Left])],
			sentence![
				word!([HeadTok::Dec], [BodyTok::In],),
				word!([HeadTok::Inc],,)
			],
			sentence![word!([HeadTok::Dec],,)],
		],
	);

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "comment"
            냥? ~
            냥!
            냐, 냥
            냐"#
		}
	);
}

#[test]
fn format_paragraph_with_multiple_comment_and_multiple_sentence() {
	let ast = Paragraph(
		vec![
			Comment("this".to_owned()),
			Comment("is".to_owned()),
			Comment("very".to_owned()),
			Comment("long".to_owned()),
			Comment("comment".to_owned()),
		],
		vec![
			sentence![
				word!([HeadTok::Inc],,[TailTok::Right]),
				word!(, [BodyTok::JumpRight],)
			],
			sentence![word!([HeadTok::Inc],, [TailTok::Left])],
			sentence![
				word!([HeadTok::Dec], [BodyTok::In],),
				word!([HeadTok::Inc],,)
			],
			sentence![word!([HeadTok::Dec],,)],
		],
	);

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "this"
            "is"
            "very"
            "long"
            "comment"
            냥? ~
            냥!
            냐, 냥
            냐"#
		}
	);
}

#[test]
fn format_code_with_empty_vectors() {
	let ast = Code {
		leading_sentences: vec![],
		paragraphs: vec![],
		trailing_comments: vec![],
	};

	assert_eq!(ast.to_string(), "");
}

#[test]
fn format_code_with_only_leading_sentences() {
	let ast = Code {
		leading_sentences: vec![
			sentence![
				word!(
					[HeadTok::Inc],
					[BodyTok::JumpLeft, BodyTok::In],
				),
				word!([HeadTok::Dec],, [TailTok::Right]),
			],
			sentence![
				word!([HeadTok::Debug],,[TailTok::Left]),
				word!(,[BodyTok::Out], [TailTok::Right, TailTok::Left, TailTok::Left]),
			],
		],
		paragraphs: vec![],
		trailing_comments: vec![],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            냥-, 냐?
            뀨! .?!!"#
		}
	);
}

#[test]
fn format_code_with_only_trailing_comments() {
	let ast = Code {
		leading_sentences: vec![],
		paragraphs: vec![],
		trailing_comments: vec![
			Comment("this is".to_owned()),
			Comment("very long and".to_owned()),
			Comment("newline delimited".to_owned()),
			Comment("comment!".to_owned()),
		],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "this is"
            "very long and"
            "newline delimited"
            "comment!""#
		}
	);
}

#[test]
fn format_code_with_paragraphs() {
	let ast = Code {
		leading_sentences: vec![],
		paragraphs: vec![
			Paragraph(
				vec![
					Comment("this".to_owned()),
					Comment("is".to_owned()),
					Comment("comment".to_owned()),
				],
				vec![
					sentence![
						word!(
							[HeadTok::Inc, HeadTok::Dec, HeadTok::Debug],
							[
								BodyTok::JumpLeft,
								BodyTok::JumpRight,
								BodyTok::Out,
								BodyTok::In,
							],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc, HeadTok::Debug],
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						)
					],
					sentence![
						word!(
							,
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc],,
							[TailTok::Right, TailTok::Left]
						)
					],
				],
			),
			Paragraph(
				vec![Comment("comment2".to_owned())],
				vec![
					sentence![word!(
						[HeadTok::Inc, HeadTok::Debug],
						[BodyTok::In],
						[TailTok::Right, TailTok::Left]
					)],
					sentence![word!(
						[HeadTok::Inc],,
						[TailTok::Right, TailTok::Left]
					)],
				],
			),
		],
		trailing_comments: vec![],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "this"
            "is"
            "comment"
            냥냐뀨-~.,?! 냥뀨,?!
            ,?! 냥?!

            "comment2"
            냥뀨,?!
            냥?!"#
		}
	);
}

#[test]
fn format_code_with_paragraphs_and_leading_sentences() {
	let ast = Code {
		leading_sentences: vec![
			sentence![
				word!(
					[HeadTok::Inc],
					[BodyTok::JumpLeft, BodyTok::In],
				),
				word!([HeadTok::Dec],, [TailTok::Right]),
			],
			sentence![
				word!([HeadTok::Debug],,[TailTok::Left]),
				word!(,[BodyTok::Out], [TailTok::Right, TailTok::Left, TailTok::Left]),
			],
		],
		paragraphs: vec![
			Paragraph(
				vec![
					Comment("this".to_owned()),
					Comment("is".to_owned()),
					Comment("comment".to_owned()),
				],
				vec![
					sentence![
						word!(
							[HeadTok::Inc, HeadTok::Dec, HeadTok::Debug],
							[
								BodyTok::JumpLeft,
								BodyTok::JumpRight,
								BodyTok::Out,
								BodyTok::In,
							],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc, HeadTok::Debug],
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						)
					],
					sentence![
						word!(
							,
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc],,
							[TailTok::Right, TailTok::Left]
						)
					],
				],
			),
			Paragraph(
				vec![Comment("comment2".to_owned())],
				vec![
					sentence![word!(
						[HeadTok::Inc, HeadTok::Debug],
						[BodyTok::In],
						[TailTok::Right, TailTok::Left]
					)],
					sentence![word!(
						[HeadTok::Inc],,
						[TailTok::Right, TailTok::Left]
					)],
				],
			),
		],
		trailing_comments: vec![],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            냥-, 냐?
            뀨! .?!!

            "this"
            "is"
            "comment"
            냥냐뀨-~.,?! 냥뀨,?!
            ,?! 냥?!

            "comment2"
            냥뀨,?!
            냥?!"#
		}
	);
}

#[test]
fn format_code_with_paragraphs_and_trailing_comments() {
	let ast = Code {
		leading_sentences: vec![],
		paragraphs: vec![
			Paragraph(
				vec![
					Comment("this".to_owned()),
					Comment("is".to_owned()),
					Comment("comment".to_owned()),
				],
				vec![
					sentence![
						word!(
							[HeadTok::Inc, HeadTok::Dec, HeadTok::Debug],
							[
								BodyTok::JumpLeft,
								BodyTok::JumpRight,
								BodyTok::Out,
								BodyTok::In,
							],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc, HeadTok::Debug],
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						)
					],
					sentence![
						word!(
							,
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc],,
							[TailTok::Right, TailTok::Left]
						)
					],
				],
			),
			Paragraph(
				vec![Comment("comment2".to_owned())],
				vec![
					sentence![word!(
						[HeadTok::Inc, HeadTok::Debug],
						[BodyTok::In],
						[TailTok::Right, TailTok::Left]
					)],
					sentence![word!(
						[HeadTok::Inc],,
						[TailTok::Right, TailTok::Left]
					)],
				],
			),
		],
		trailing_comments: vec![
			Comment("this is".to_owned()),
			Comment("very long and".to_owned()),
			Comment("newline delimited".to_owned()),
			Comment("comment!".to_owned()),
		],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            "this"
            "is"
            "comment"
            냥냐뀨-~.,?! 냥뀨,?!
            ,?! 냥?!

            "comment2"
            냥뀨,?!
            냥?!

            "this is"
            "very long and"
            "newline delimited"
            "comment!""#
		}
	);
}

#[test]
fn format_code_full() {
	let ast = Code {
		leading_sentences: vec![
			sentence![
				word!(
					[HeadTok::Inc],
					[BodyTok::JumpLeft, BodyTok::In],
				),
				word!([HeadTok::Dec],, [TailTok::Right]),
			],
			sentence![
				word!([HeadTok::Debug],,[TailTok::Left]),
				word!(,[BodyTok::Out], [TailTok::Right, TailTok::Left, TailTok::Left]),
			],
		],
		paragraphs: vec![
			Paragraph(
				vec![
					Comment("this".to_owned()),
					Comment("is".to_owned()),
					Comment("comment".to_owned()),
				],
				vec![
					sentence![
						word!(
							[HeadTok::Inc, HeadTok::Dec, HeadTok::Debug],
							[
								BodyTok::JumpLeft,
								BodyTok::JumpRight,
								BodyTok::Out,
								BodyTok::In,
							],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc, HeadTok::Debug],
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						)
					],
					sentence![
						word!(
							,
							[BodyTok::In],
							[TailTok::Right, TailTok::Left]
						),
						word!(
							[HeadTok::Inc],,
							[TailTok::Right, TailTok::Left]
						)
					],
				],
			),
			Paragraph(
				vec![Comment("comment2".to_owned())],
				vec![
					sentence![word!(
						[HeadTok::Inc, HeadTok::Debug],
						[BodyTok::In],
						[TailTok::Right, TailTok::Left]
					)],
					sentence![word!(
						[HeadTok::Inc],,
						[TailTok::Right, TailTok::Left]
					)],
				],
			),
		],
		trailing_comments: vec![
			Comment("this is".to_owned()),
			Comment("very long and".to_owned()),
			Comment("newline delimited".to_owned()),
			Comment("comment!".to_owned()),
		],
	};

	assert_eq!(
		ast.to_string(),
		indoc! {r#"
            냥-, 냐?
            뀨! .?!!

            "this"
            "is"
            "comment"
            냥냐뀨-~.,?! 냥뀨,?!
            ,?! 냥?!

            "comment2"
            냥뀨,?!
            냥?!

            "this is"
            "very long and"
            "newline delimited"
            "comment!""#
		}
	);
}

#[test]
fn root_must_emit_nothing_if_given_code_is_empty() {
	let code = Root(Code {
		leading_sentences: vec![],
		paragraphs: vec![],
		trailing_comments: vec![],
	});

	assert_eq!(code.to_string(), "");
}

#[test]
fn root_must_add_trailing_newline_character_if_code_is_not_empty() {
	let code = Root(Code {
		leading_sentences: vec![],
		paragraphs: vec![],
		trailing_comments: vec![Comment(".".to_owned())],
	});

	assert_eq!(
		code.to_string(),
		indoc! {r#"
            "."
        "#}
	)
}
