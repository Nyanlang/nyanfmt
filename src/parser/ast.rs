struct Root;

pub enum Head {
	Inc,
	Dec,
	Debug,
}

pub enum Body {
	Out,
	In,
	JumpRight,
	JumpLeft,
}

pub enum Tail {
	Right,
	Left,
}

pub struct Word {
	Head: Option<Head>,
	Body: Option<Body>,
	Tail: Option<Tail>,
}
