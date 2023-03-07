use std::fmt::{self, Display, Formatter};

use super::ast::*;

impl Display for HeadTok {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Inc => '냥',
				Self::Dec => '냐',
				Self::Debug => '뀨',
			}
		)
	}
}

impl Display for BodyTok {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Out => '.',
				Self::In => ',',
				Self::JumpRight => '~',
				Self::JumpLeft => '-',
			}
		)
	}
}

impl Display for TailTok {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Right => '?',
				Self::Left => '!',
			}
		)
	}
}
