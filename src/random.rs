use crate::{all, Audience, Form, Mode, Polarity, Tense};

#[cfg(test)]
mod tests {
	use crate::random;

	#[test]
	fn randoms() {
		random::tense();
		random::audience();
		random::polarity();
		random::mode();
		random::form();
	}
}

pub fn form() -> Form {
	Form {
		tense: tense(),
		audience: audience(),
		polarity: polarity(),
		mode: mode(),
	}
}

pub fn tense() -> Tense { random_pick(all::tenses()) }

fn random_pick<T: Clone>(options: Vec<T>) -> T {
	let index: usize = random_u8() as usize % options.len();
	options[index].clone()
}

fn random_u8() -> u8 {
	let mut buffer = [0u8];
	getrandom::getrandom(&mut buffer).expect("Random number is required");
	buffer[0]
}

pub fn audience() -> Audience { random_pick(all::audiences()) }

pub fn mode() -> Mode { random_pick(all::modes()) }

pub fn polarity() -> Polarity { random_pick(all::polarities()) }
