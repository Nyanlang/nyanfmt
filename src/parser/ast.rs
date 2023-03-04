struct Root;

pub enum HeadTok {
	Inc,
	Dec,
	Debug,
}

pub struct Head(pub Vec<HeadTok>);

pub enum BodyTok {
	Out,
	In,
	JumpRight,
	JumpLeft,
}

pub struct Body(pub Vec<BodyTok>);

pub enum TailTok {
	Right,
	Left,
}

pub struct Tail(pub Vec<TailTok>);

pub struct Word {
	Head: Option<Head>,
	Body: Option<Body>,
	Tail: Option<Tail>,
}
