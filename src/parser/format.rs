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

impl Display for Head {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join("")
		)
	}
}

impl Display for Body {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join("")
		)
	}
}

impl Display for Tail {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join("")
		)
	}
}

impl Display for Word {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}{}{}",
			self.head
				.as_ref()
				.map(|v| format!("{v}"))
				.unwrap_or_default(),
			self.body
				.as_ref()
				.map(|v| format!("{v}"))
				.unwrap_or_default(),
			self.tail
				.as_ref()
				.map(|v| format!("{v}"))
				.unwrap_or_default(),
		)
	}
}

impl Display for Sentence {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join(" ")
		)
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, r#""{}""#, self.0)
	}
}

#[cfg(test)]
#[path = "format.spec.rs"]
mod tests;
