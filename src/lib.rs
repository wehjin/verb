pub use crate::core::*;
use crate::utils::{drop_last_char, last_char};

mod core;
mod utils;
pub mod all;
pub mod random;

#[cfg(test)]
mod tests {
	use crate::Polarity::{Affirmative, Negative};

	use super::*;
	use super::Audience::{Plain, Polite};
	use super::Mode::{Immediate, Potential};
	use super::Tense::{Past, Present};

	#[test]
	fn past_plain_affirmative_immediate() {
		let tests = vec![
			(miru(), "みた"),
			(kau(), "かった"),
			(matsu(), "まった"),
			(kaeru(), "かえった"),
			(yomu(), "よんだ"),
			(asobu(), "あそんだ"),
			(shinu(), "しんだ"),
			(kaku(), "かいた"),
			(iku(), "行った"),
			(nugu(), "ぬいだ"),
			(sagasu(), "さがした"),
		];
		assert_conjugations(tests, Past, Plain, Affirmative, Immediate);
	}

	fn assert_conjugations(tests: Vec<(Verb, &str)>, tense: Tense, audience: Audience, polariy: Polarity, mode: Mode) {
		tests.iter().for_each(|(verb, expected)| {
			let conjugation = verb.conjugate(tense, audience, polariy, mode);
			assert_eq!(conjugation, expected.to_string());
		})
	}

	#[test]
	fn past_plain_negative_immediate() {
		let tests = vec![
			(miru(), "みなかった"),
			(kau(), "かわなかった"),
			(matsu(), "またなかった"),
			(kaeru(), "かえらなかった"),
			(yomu(), "よまなかった"),
			(asobu(), "あそばなかった"),
			(shinu(), "しななかった"),
			(kaku(), "かかなかった"),
			(iku(), "行かなかった"),
			(nugu(), "ぬがなかった"),
			(sagasu(), "さがさなかった"),
		];
		assert_conjugations(tests, Past, Plain, Negative, Immediate);
	}

	#[test]
	fn present_polite_affirmative_immediate() {
		let tests = vec![
			(miru(), "みます"),
			(kau(), "かいます"),
			(matsu(), "まちます"),
			(kaeru(), "かえります"),
			(yomu(), "よみます"),
			(asobu(), "あそびます"),
			(shinu(), "しにます"),
			(kaku(), "かきます"),
			(iku(), "行きます"),
			(nugu(), "ぬぎます"),
			(sagasu(), "さがします"),
		];
		assert_conjugations(tests, Present, Polite, Affirmative, Immediate);
	}

	#[test]
	fn present_polite_negative_immediate() {
		let tests = vec![
			(miru(), "みません"),
			(kau(), "かいません"),
			(matsu(), "まちません"),
			(kaeru(), "かえりません"),
			(yomu(), "よみません"),
			(asobu(), "あそびません"),
			(shinu(), "しにません"),
			(kaku(), "かきません"),
			(iku(), "行きません"),
			(nugu(), "ぬぎません"),
			(sagasu(), "さがしません"),
		];
		assert_conjugations(tests, Present, Polite, Negative, Immediate);
	}

	#[test]
	fn past_polite_affirmative_potential() {
		let tests = vec![
			(miru(), "みられました"),
			(kau(), "かえました"),
			(matsu(), "まてました"),
			(kaeru(), "かえれました"),
			(yomu(), "よめました"),
			(asobu(), "あそべました"),
			(shinu(), "しねました"),
			(kaku(), "かけました"),
			(nugu(), "ぬげました"),
			(sagasu(), "さがせました"),
		];
		assert_conjugations(tests, Past, Polite, Affirmative, Potential);
	}

	#[test]
	fn past_polite_negative_potential() {
		let tests = vec![
			(miru(), "みられませんでした"),
			(kau(), "かえませんでした"),
			(matsu(), "まてませんでした"),
			(kaeru(), "かえれませんでした"),
			(yomu(), "よめませんでした"),
			(asobu(), "あそべませんでした"),
			(shinu(), "しねませんでした"),
			(kaku(), "かけませんでした"),
			(nugu(), "ぬげませんでした"),
			(sagasu(), "さがせませんでした"),
		];
		assert_conjugations(tests, Past, Polite, Negative, Potential);
	}

	#[test]
	fn ru_verb() {
		let verb = miru();
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Immediate), "みる");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Immediate), "みます");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Immediate), "みた");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Immediate), "みました");
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Potential), "みられる");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Potential), "みられます");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Potential), "みられた");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Potential), "みられました");

		assert_eq!(verb.conjugate(Present, Plain, Negative, Immediate), "みない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Immediate), "みません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Immediate), "みなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Immediate), "みませんでした");
		assert_eq!(verb.conjugate(Present, Plain, Negative, Potential), "みられない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Potential), "みられません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Potential), "みられなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Potential), "みられませんでした");
	}

	#[test]
	fn u_verb() {
		let verb = kaeru();
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Immediate), "かえる");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Immediate), "かえります");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Immediate), "かえった");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Immediate), "かえりました");
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Potential), "かえれる");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Potential), "かえれます");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Potential), "かえれた");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Potential), "かえれました");

		assert_eq!(verb.conjugate(Present, Plain, Negative, Immediate), "かえらない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Immediate), "かえりません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Immediate), "かえらなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Immediate), "かえりませんでした");
		assert_eq!(verb.conjugate(Present, Plain, Negative, Potential), "かえれない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Potential), "かえれません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Potential), "かえれなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Potential), "かえれませんでした");
	}

	#[test]
	fn suru_verb() {
		let verb = Verb {
			ch: 100,
			kind: Kind::Suru,
			search: "うんてんする".to_string(),
			english: "drive".to_string(),
		};
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Immediate), "うんてんする");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Immediate), "うんてんします");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Immediate), "うんてんした");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Immediate), "うんてんしました");
		assert_eq!(verb.conjugate(Present, Plain, Affirmative, Potential), "うんてんできる");
		assert_eq!(verb.conjugate(Present, Polite, Affirmative, Potential), "うんてんできます");
		assert_eq!(verb.conjugate(Past, Plain, Affirmative, Potential), "うんてんできた");
		assert_eq!(verb.conjugate(Past, Polite, Affirmative, Potential), "うんてんできました");

		assert_eq!(verb.conjugate(Present, Plain, Negative, Immediate), "うんてんしない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Immediate), "うんてんしません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Immediate), "うんてんしなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Immediate), "うんてんしませんでした");
		assert_eq!(verb.conjugate(Present, Plain, Negative, Potential), "うんてんできない");
		assert_eq!(verb.conjugate(Present, Polite, Negative, Potential), "うんてんできません");
		assert_eq!(verb.conjugate(Past, Plain, Negative, Potential), "うんてんできなかった");
		assert_eq!(verb.conjugate(Past, Polite, Negative, Potential), "うんてんできませんでした");
	}

	#[test]
	fn suru_base_verb() {
		let base_verb = Verb { ch: 100, kind: Kind::Suru, search: "する".to_string(), english: "do".to_string() };
		assert_eq!(base_verb.conjugate(Present, Polite, Affirmative, Immediate), "します");
	}

	#[test]
	fn translate() {
		let verb = miru();
		let tests = vec![
			(Present, Affirmative, Immediate, "will see"),
			(Present, Affirmative, Potential, "can see"),
			(Past, Affirmative, Immediate, "did see"),
			(Past, Affirmative, Potential, "could see")
		];
		tests.iter().for_each(|(tense, polarity, mode, expected)| {
			let translation = verb.translate(*tense, *polarity, *mode);
			assert_eq!(translation.as_str(), *expected);
		})
	}

	#[test]
	fn verb_name() {
		let verb = miru();
		let name = verb.name();
		assert_eq!("0_みる", name);
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct Verb {
	pub ch: u32,
	pub kind: Kind,
	pub search: String,
	pub english: String,
}

impl Verb {
	pub fn name(&self) -> String {
		format!("{}_{}", self.ch, self.search)
	}

	pub fn conjugate(&self, tense: Tense, audience: Audience, polarity: Polarity, mode: Mode) -> String {
		match mode {
			Mode::Immediate => conjugate_verb(&self.search, self.kind, tense, audience, polarity),
			Mode::Potential => conjugate_verb(&potential(&self.search, self.kind), Kind::Ru, tense, audience, polarity),
		}
	}

	pub fn translate(&self, tense: Tense, polarity: Polarity, mode: Mode) -> String {
		let prefix = match (mode, tense) {
			(Mode::Immediate, Tense::Present) => "will",
			(Mode::Immediate, Tense::Past) => "did",
			(Mode::Potential, Tense::Present) => "can",
			(Mode::Potential, Tense::Past) => "could",
		};
		let middle = match polarity {
			Polarity::Affirmative => " ",
			Polarity::Negative => " not "
		};
		format!("{}{}{}", prefix, middle, self.english)
	}
}

fn potential(verb: &str, kind: Kind) -> String {
	let (pre, post) = match kind {
		Kind::Ru => (drop_last_char(verb), "られる"),
		Kind::U => (drop_last_char(verb), match last_char(verb).as_str() {
			"う" => "える",
			"つ" => "てる",
			"る" => "れる",
			"む" => "める",
			"ぶ" => "べる",
			"ぬ" => "ねる",
			"く" => "ける",
			"ぐ" => "げる",
			"す" => "せる",
			_ => panic!("Invalid ending")
		}),
		Kind::Suru => (drop_last_char(&drop_last_char(verb)), "できる"),
	};
	format!("{}{}", pre, post)
}

fn conjugate_verb(verb: &str, kind: Kind, tense: Tense, audience: Audience, polarity: Polarity) -> String {
	match (tense, audience, polarity) {
		(Tense::Present, Audience::Plain, Polarity::Affirmative) => verb.to_string(),
		(Tense::Present, Audience::Plain, Polarity::Negative) => format!("{}ない", pre_nai(verb, kind)),
		(Tense::Present, Audience::Polite, Polarity::Affirmative) => format!("{}ます", pre_masu(verb, kind)),
		(Tense::Present, Audience::Polite, Polarity::Negative) => format!("{}ません", pre_masu(verb, kind)),
		(Tense::Past, Audience::Plain, Polarity::Affirmative) => ta(verb, kind),
		(Tense::Past, Audience::Plain, Polarity::Negative) => format!("{}なかった", pre_nai(verb, kind)),
		(Tense::Past, Audience::Polite, Polarity::Affirmative) => format!("{}ました", pre_masu(verb, kind)),
		(Tense::Past, Audience::Polite, Polarity::Negative) => format!("{}ませんでした", pre_masu(verb, kind)),
	}
}

fn ta(verb: &str, kind: Kind) -> String {
	match kind {
		Kind::Ru => format!("{}{}", drop_last_char(verb), "た"),
		Kind::U if verb.ends_with("行く") || verb == "いく" || verb.ends_with("ていく") =>
			format!("{}{}", drop_last_char(verb), "った"),
		Kind::U => format!("{}{}", drop_last_char(verb), match last_char(verb).as_str() {
			"う" => "った",
			"つ" => "った",
			"る" => "った",
			"む" => "んだ",
			"ぶ" => "んだ",
			"ぬ" => "んだ",
			"く" => "いた",
			"ぐ" => "いだ",
			"す" => "した",
			_ => panic!("Invalid ending")
		}),
		Kind::Suru => format!("{}{}", drop_last_char(&drop_last_char(verb)), "した"),
	}
}

fn pre_nai(verb: &str, kind: Kind) -> String {
	match kind {
		Kind::Ru => drop_last_char(verb),
		Kind::U => format!("{}{}", drop_last_char(verb), a(&last_char(verb))),
		Kind::Suru => format!("{}{}", drop_last_char(&drop_last_char(verb)), "し"),
	}
}

fn a(u: &str) -> &str {
	match u {
		"う" => "わ",
		"つ" => "た",
		"る" => "ら",
		"む" => "ま",
		"ぶ" => "ば",
		"ぬ" => "な",
		"く" => "か",
		"ぐ" => "が",
		"す" => "さ",
		_ => panic!("Invalid ending")
	}
}

fn pre_masu(verb: &str, kind: Kind) -> String {
	match kind {
		Kind::Ru => drop_last_char(verb),
		Kind::U => format!("{}{}", drop_last_char(verb), i(&last_char(verb))),
		Kind::Suru => format!("{}{}", drop_last_char(&drop_last_char(verb)), "し"),
	}
}

fn i(u: &str) -> &str {
	match u {
		"う" => "い",
		"つ" => "ち",
		"る" => "り",
		"む" => "み",
		"ぶ" => "び",
		"ぬ" => "に",
		"く" => "き",
		"ぐ" => "ぎ",
		"す" => "し",
		_ => panic!("Invalid ending")
	}
}

pub fn verbs() -> Vec<Verb> {
	vec![miru(), kau(), matsu(), kaeru(), yomu(), asobu(), shinu(), kaku(), iku(), nugu(), sagasu()]
}

fn miru() -> Verb { Verb { ch: 0, kind: Kind::Ru, search: "みる".to_string(), english: "see".to_string() } }

fn kau() -> Verb { Verb { ch: 0, kind: Kind::U, search: "かう".to_string(), english: "buy".to_string() } }

fn matsu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "まつ".to_string(), english: "wait".to_string() } }

fn kaeru() -> Verb { Verb { ch: 0, kind: Kind::U, search: "かえる".to_string(), english: "go home".to_string() } }

fn yomu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "よむ".to_string(), english: "read".to_string() } }

fn asobu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "あそぶ".to_string(), english: "play".to_string() } }

fn shinu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "しぬ".to_string(), english: "die".to_string() } }

fn kaku() -> Verb { Verb { ch: 0, kind: Kind::U, search: "かく".to_string(), english: "write".to_string() } }

fn iku() -> Verb { Verb { ch: 0, kind: Kind::U, search: "行く".to_string(), english: "go".to_string() } }

fn nugu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "ぬぐ".to_string(), english: "disrobe".to_string() } }

fn sagasu() -> Verb { Verb { ch: 0, kind: Kind::U, search: "さがす".to_string(), english: "search".to_string() } }
