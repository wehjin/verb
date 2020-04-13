use crate::{Audience, Form, Mode, Polarity, Tense};
use crate::Polarity::{Affirmative, Negative};

#[cfg(test)]
mod tests {
	use crate::all::forms;

	#[test]
	fn all_variants_contains_tn_x_an_x_mn_members() {
		let forms = forms();
		let tense_count = 2;
		let audience_count = 2;
		let polarity_count = 2;
		let mode_count = 2;
		assert_eq!(forms.len(), tense_count * audience_count * polarity_count * mode_count);
	}
}

pub fn forms() -> Vec<Form> {
	let mut all = Vec::new();
	for tense in tenses() {
		for audience in audiences() {
			for polarity in polarities() {
				for mode in modes() {
					all.push(Form { tense, audience, polarity, mode })
				}
			}
		}
	}
	all
}

pub fn tenses() -> Vec<Tense> { vec![Tense::Present, Tense::Past] }

pub fn audiences() -> Vec<Audience> { vec![Audience::Plain, Audience::Polite] }

pub fn polarities() -> Vec<Polarity> { vec![Affirmative, Negative] }

pub fn modes() -> Vec<Mode> { vec![Mode::Immediate, Mode::Potential] }



