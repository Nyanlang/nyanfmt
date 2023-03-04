struct Root;

enum Head {
	Inc,
	Dec,
	Debug,
}

enum Body {
	Out,
	In,
	JumpRight,
	JumpLeft,
}

enum Tail {
	Right,
	Left,
}

struct Word {
	Head: Head,
	Body: Body,
	Tail: Tail,
}
