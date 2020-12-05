// File: day_4.rs
// Author: Jacob Guenther
// Date: December 2020

/*
Copyright 2020 Jacob Guenther

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::common::{
	ChallengeT,
};

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		4
	}
	fn new() -> Self {
		let [part_1_result, part_2_result] = include_str!("../inputs/day_4.txt")
			.split("\n\n")
			.map(|with_whitespaces|
				with_whitespaces.replace(char::is_whitespace, ":")
			)
			.fold([0, 0], |acc: [usize; 2], passport_string| {
				let key_value = passport_string.split(':').collect::<Vec<&str>>();
				let [partial_1, partial_2] = key_value
					.iter()
					.zip(&key_value[1..])
					.fold([Partial::default(), Partial::default()], |acc, (key, value)| {
						[to_partial_1(&acc[0], key),
						 to_partial_2(&acc[1], key, value)]
					});
				[acc[0] + partial_1.is_valid() as usize,
				 acc[1] + partial_2.is_valid() as usize]
			});

		Self {
			part_1_result: part_1_result,
			part_2_result: part_2_result,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
fn to_partial_1(partial: &Partial, key: &str) -> Partial {
	let mut new = partial.clone();
	match key {
		"byr" => new.birth_year = true,
		"iyr" => new.issue_year = true,
		"eyr" => new.experation_year = true,
		"hgt" => new.height = true,
		"hcl" => new.hair_color = true,
		"ecl" => new.eye_color = true,
		"pid" => new.passport_id = true,
		_ => (),
	}
	new
}
fn to_partial_2(partial: &Partial, key: &str, value: &str) -> Partial {
	let mut new = partial.clone();
	match key {
		"byr" => new.birth_year = {
			let v = value.parse().unwrap();
			1919 < v && v < 2003
		},
		"iyr" => new.issue_year = {
			let v = value.parse().unwrap();
			2009 < v && v < 2021
		},
		"eyr" => new.experation_year = {
			let v = value.parse().unwrap();
			2019 < v && v < 2031
		},
		"hgt" => new.height = {
			let len = value.len();
			match *&value[..(len-2)].parse::<usize>() {
				Ok(n) => if value.trim().ends_with("cm") {
						149 < n && n < 194
					} else if value.trim().ends_with("in") {
						58 < n && n < 77
					} else {
						false
					}
				Err(_) => false,
			}
		},
		"hcl" => new.hair_color = {
			if value.len() != 7 || value.chars().nth(0).unwrap() != '#'{
				false
			} else {
				let mut temp = true;
				for c in value[1..].chars() {
					if !c.is_ascii_hexdigit() {
						temp = false;
						break;
					}
				}
				temp
			}
		},
		"ecl" => new.eye_color = match value {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
			_ => false,
		},
		"pid" => new.passport_id = {
			if value.len() != 9 {
				false
			} else {
				let mut are_digits = true;
				for c in value.chars() {
					if !c.is_ascii_digit() {
						are_digits = false;
						break;
					}
				}
				are_digits
			}
		},
		_ => (),
	}
	new
}
#[derive(Debug, Copy, Clone)]
struct Partial {
	birth_year: bool,
	issue_year: bool,
	experation_year: bool,
	height: bool,
	hair_color: bool,
	eye_color: bool,
	passport_id: bool,
}
impl Default for Partial {
	fn default() -> Self {
		Self {
			birth_year: false,
			issue_year: false,
			experation_year: false,
			height: false,
			hair_color: false,
			eye_color: false,
			passport_id: false,
		}
	}
}
impl Partial {
	fn is_valid(&self) -> bool {
		self.birth_year
		&& self.issue_year
		&& self.experation_year
		&& self.height
		&& self.hair_color
		&& self.eye_color
		&& self.passport_id
	}
}
#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::{
		ChallengeT,
	};
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 235);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 194);
	}

    #[bench]
    fn both(b: &mut Bencher) {
		b.iter(|| {
			let challenge = Challenge::new();
			challenge.part_1();
			challenge.part_2();
		})
	}
}
