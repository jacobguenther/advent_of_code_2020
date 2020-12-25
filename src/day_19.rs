// File: day_19.rs
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

use super::common::*;

pub struct Challenge {
	part_1_result: u32,
	part_2_result: u32,
}
impl ChallengeT for Challenge {
	type Output1 = u32;
	type Output2 = u32;

	fn day() -> u8 {
		19
	}
	fn new() -> Self {
		let mut input_split = include_str!("../inputs/day_19.txt").split("\n\n");
		let mut rules = vec![Rule::Single(0); 200];
		input_split.next().unwrap().lines().for_each(|line| {
			let mut parts = line.split(": ");
			let name = parts.next().unwrap().parse::<usize>().unwrap();
			let rule = parts.next().unwrap();
			let parsed = if rule.contains('"') {
				Rule::Letter(rule.split('\"').nth(1).unwrap().chars().next().unwrap())
			} else {
				let lexemes = rule.split_whitespace();
				let mut nums = Vec::<usize>::new();
				let mut is_either = false;
				for lexeme in lexemes {
					match lexeme {
						"|" => is_either = true,
						_ => nums.push(lexeme.parse().unwrap()),
					}
				}
				match (is_either, nums.len()) {
					(true, 2) => Rule::Either(nums[0], nums[1]),
					(true, 4) => Rule::Either2(nums[0], nums[1], nums[2], nums[3]),
					(_, 1) => Rule::Single(nums[0]),
					(_, 2) => Rule::Double(nums[0], nums[1]),
					_ => panic!("Unexpected rule {}", rule),
				}
			};
			rules[name] = parsed;
		});

		let [part_1_result, part_2_result] = input_split
			.next()
			.unwrap()
			.lines()
			.map(|line| line.chars().collect::<Vec<_>>())
			.fold([0, 0], |[mut part_1_count, mut part_2_count], message| {
				part_1_count += matches(&message, &rules) as u32;
				part_2_count += {
					let mut start = 0;
					let mut count42 = 0;
					loop {
						let (mat, len) = match_message(&message[start..], &rules, 42);
						if mat {
							start += len;
							count42 += 1;
						} else {
							break;
						}
					}
					let mut count31 = 0;
					for _ in 0..(count42 - 1) {
						let (mat31, len31) = match_message(&message[start..], &rules, 31);
						if mat31 {
							start += len31;
							count31 += 1;
						} else {
							break;
						}
					}
					start == message.len() && count31 > 0
				} as u32;

				[part_1_count, part_2_count]
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

#[derive(Debug, Copy, Clone)]
enum Rule {
	// i: "c"
	Letter(char),
	// i: a
	Single(usize),
	// i: a1 a2
	Double(usize, usize),
	// i: a1 | b1
	Either(usize, usize),
	// i: a1 a2 | b1 b2
	Either2(usize, usize, usize, usize),
}
fn matches(message: &[char], rules: &[Rule]) -> bool {
	let (valid, len) = match_message(message, rules, 0);
	valid && message.len() == len
}
fn match_message(message: &[char], rules: &[Rule], rule: usize) -> (bool, usize) {
	match rules[rule] {
		Rule::Letter(ref c) => (Some(c) == message.first(), 1),
		Rule::Single(a) => match_message(message, rules, a),
		Rule::Double(a1, a2) => {
			let (matched, amount) = match_message(message, rules, a1);
			if matched && amount <= message.len() {
				let (next_matched, next_amount) = match_message(&message[amount..], rules, a2);
				if next_matched {
					(true, amount + next_amount)
				} else {
					(false, 0)
				}
			} else {
				(false, 0)
			}
		}
		Rule::Either(a, b) => {
			let (first_m, first_l) = match_message(message, rules, a);
			if first_m && first_l <= message.len() {
				(first_m, first_l)
			} else {
				let (second_m, second_l) = match_message(message, rules, b);
				if second_m && second_l <= message.len() {
					(second_m, second_l)
				} else {
					(false, 0)
				}
			}
		}
		Rule::Either2(a1, a2, b1, b2) => {
			let (matched, amount) = match_message(message, rules, a1);
			if matched && amount < message.len() {
				let (next_matched, next_amount) = match_message(&message[amount..], rules, a2);
				if next_matched && amount + next_amount <= message.len() {
					return (true, amount + next_amount);
				}
			}
			let (matched, amount) = match_message(message, rules, b1);
			if matched && amount < message.len() {
				let (next_matched, next_amount) = match_message(&message[amount..], rules, b2);
				if next_matched && amount + next_amount <= message.len() {
					return (true, amount + next_amount);
				}
			}
			(false, 0)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 239);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 405);
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
