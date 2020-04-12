use std::prelude::v1::Iterator;
use std::string::ToString;

pub fn drop_last_char(s: &str) -> String {
	let last_len = last_char(s).len();
	s[..s.len() - last_len].to_string()
}

pub fn last_char(s: &str) -> String {
	s.chars().last().unwrap().to_string()
}
