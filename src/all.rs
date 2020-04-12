use crate::{Audience, Mode, Tense};

#[cfg(test)]
mod tests {
	use crate::all::variants;

	#[test]
	fn all_variants_contains_tn_x_an_x_mn_members() {
		let x = variants();
		let tense_count = 2;
		let audience_count = 2;
		let mode_count = 2;
		assert_eq!(x.len(), tense_count * audience_count * mode_count);
	}
}

pub fn variants() -> Vec<(Tense, Audience, Mode)> {
	let mut all = Vec::new();
	for tense in tenses() {
		for audience in audiences() {
			for mode in modes() {
				all.push((tense, audience, mode))
			}
		}
	}
	all
}

pub fn tenses() -> Vec<Tense> { vec![Tense::Present, Tense::Past] }

pub fn audiences() -> Vec<Audience> { vec![Audience::Plain, Audience::Polite] }

pub fn modes() -> Vec<Mode> { vec![Mode::Immediate, Mode::Potential] }



