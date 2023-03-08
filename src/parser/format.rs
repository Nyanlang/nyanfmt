use std::{
	fmt::{self, Display, Formatter},
	ops::Not,
};

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

impl Display for Paragraph {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}\n{}",
			self.0
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join("\n"),
			self.1
				.iter()
				.map(|i| format!("{i}"))
				.collect::<Vec<_>>()
				.join("\n"),
		)
	}
}

impl Display for Code {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut l = self
			.leading_sentences
			.iter()
			.map(|i| format!("{i}"))
			.collect::<Vec<_>>()
			.join("\n");
		let mut p = self
			.paragraphs
			.iter()
			.map(|i| format!("{i}"))
			.collect::<Vec<_>>()
			.join("\n\n");
		let t = self
			.trailing_comments
			.iter()
			.map(|i| format!("{i}"))
			.collect::<Vec<_>>()
			.join("\n");

		match (l.len(), p.len(), t.len()) {
			(1.., 1.., 1..) => {
				l.push_str("\n\n");
				p.push_str("\n\n");
			},
			(1.., 1.., _) | (1.., _, 1..) => l.push_str("\n\n"),
			(_, 1.., 1..) => p.push_str("\n\n"),
			_ => {},
		}

		write!(f, "{}{}{}", l, p, t)
	}
}

impl Display for Root {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let res = self.0.to_string();

		write!(
			f,
			"{res}{}",
			res.is_empty()
				.not()
				.then_some("\n")
				.unwrap_or_default()
		)
	}
}

#[cfg(test)]
#[path = "format.spec.rs"]
mod tests;
