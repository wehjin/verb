#[cfg(test)]
mod tests {
	use crate::{Audience, Form, Mode, Polarity, Tense};

	#[test]
	fn form_has_name() {
		let form = Form {
			tense: Tense::Present,
			audience: Audience::Plain,
			polarity: Polarity::Affirmative,
			mode: Mode::Immediate,
		};
		let name = form.name();
		assert_eq!(&name, "Present_Plain_Affirmative_Immediate")
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Kind {
	U,
	Ru,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tense {
	Present,
	Past,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Audience {
	Plain,
	Polite,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	Immediate,
	Potential,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Polarity {
	Affirmative,
	Negative,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Form {
	pub tense: Tense,
	pub audience: Audience,
	pub polarity: Polarity,
	pub mode: Mode,
}

impl Form {
	pub fn name(&self) -> String {
		format!("{:?}_{:?}_{:?}_{:?}", self.tense, self.audience, self.polarity, self.mode)
	}
}