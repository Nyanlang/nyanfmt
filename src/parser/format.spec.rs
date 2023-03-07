use super::*;

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
