use super::*;
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
