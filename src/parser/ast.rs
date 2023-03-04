struct Root;

#[derive(Clone, Debug, PartialEq)]
pub enum HeadTok {
	Inc,
	Dec,
	Debug,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Head(pub Vec<HeadTok>);

#[derive(Clone, Debug, PartialEq)]
pub enum BodyTok {
	Out,
	In,
	JumpRight,
	JumpLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Body(pub Vec<BodyTok>);

#[derive(Clone, Debug, PartialEq)]
pub enum TailTok {
	Right,
	Left,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tail(pub Vec<TailTok>);

pub struct Word {
	Head: Option<Head>,
	Body: Option<Body>,
	Tail: Option<Tail>,
}
