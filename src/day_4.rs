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

use super::common::ChallengeT;

pub struct Challenge {
	part_1_result: u16,
	part_2_result: u16,
}
impl ChallengeT for Challenge {
	type Output1 = u16;
	type Output2 = u16;

	fn day() -> u8 {
		4
	}
	fn new() -> Self {
		let [part_1_result, part_2_result] = include_str!("../inputs/day_4.txt")
			.split("\n\n")
			.map(|with_whitespaces| with_whitespaces.replace(char::is_whitespace, ":"))
			.fold([0, 0], |acc: [u16; 2], passport_string| {
				let key_value = passport_string.split(':').collect::<Vec<&str>>();
				let [passport_data_1, passport_data_2] =
					key_value.iter().zip(&key_value[1..]).fold(
						[PassportData::default(), PassportData::default()],
						|acc, (key, value)| {
							[
								to_passport_data_1(&acc[0], key),
								to_passport_data_2(&acc[1], key, value),
							]
						},
					);
				[
					acc[0] + passport_data_1.is_valid() as u16,
					acc[1] + passport_data_2.is_valid() as u16,
				]
			});

		Self {
			part_1_result,
			part_2_result,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
fn to_passport_data_1(passport_data: &PassportData, key: &str) -> PassportData {
	let mut new = *passport_data;
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
fn to_passport_data_2(passport_data: &PassportData, key: &str, value: &str) -> PassportData {
	let mut new = *passport_data;
	match key {
		"byr" => {
			new.birth_year = {
				let v = value.parse().unwrap();
				1919 < v && v < 2003
			}
		}
		"iyr" => {
			new.issue_year = {
				let v = value.parse().unwrap();
				2009 < v && v < 2021
			}
		}
		"eyr" => {
			new.experation_year = {
				let v = value.parse().unwrap();
				2019 < v && v < 2031
			}
		}
		"hgt" => {
			new.height = {
				let len = value.len();
				match value[..(len - 2)].parse::<u16>() {
					Ok(n) => match &value[(len - 2)..] {
						"cm" => 149 < n && n < 194,
						"in" => 58 < n && n < 77,
						_ => false,
					},
					Err(_) => false,
				}
			}
		}
		// Note: '#' is 35
		"hcl" => {
			new.hair_color = {
				if value.len() != 7 || value.bytes().next().unwrap() != 35 {
					false
				} else {
					u32::from_str_radix(&value[1..], 16).is_ok()
				}
			}
		}
		"ecl" => {
			new.eye_color = matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
		}
		"pid" => {
			new.passport_id = {
				if value.len() != 9 {
					false
				} else {
					value.parse::<u32>().is_ok()
				}
			}
		}
		_ => (),
	}
	new
}
#[derive(Debug, Copy, Clone)]
struct PassportData {
	birth_year: bool,
	issue_year: bool,
	experation_year: bool,
	height: bool,
	hair_color: bool,
	eye_color: bool,
	passport_id: bool,
}
impl Default for PassportData {
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
impl PassportData {
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
	use crate::common::ChallengeT;
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
