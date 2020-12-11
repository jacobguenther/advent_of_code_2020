// File: day_7.rs
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

use std::collections::HashMap;

use super::common::ChallengeT;

pub struct Challenge {
	parsed_input: HashMap<&'static str, Vec<(&'static str, usize)>>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		7
	}
	fn new() -> Self {
		let parsed_input = include_str!("../inputs/day_7.txt")
			.lines()
			.map(|line| parse_line(line))
			.collect::<HashMap<&str, Vec<(&str, usize)>>>();

		Self {
			parsed_input: parsed_input
		}
	}
	fn part_1(&self) -> Self::Output1 {
		let mut bags_that_contain_gold_bag = 0;
		let mut cache = HashMap::new();
		for (bag, _) in self.parsed_input.clone() {
			if contains_gold(&bag, &self.parsed_input, &mut cache) {
				bags_that_contain_gold_bag += 1;
			}
		}
		bags_that_contain_gold_bag
	}
	fn part_2(&self) -> Self::Output2 {
		count_bags_in("shiny gold", &self.parsed_input)
	}
}

fn parse_line(line: &str) -> (&str, Vec<(&str, usize)>) {
	let mut iter = line.split(" bags ");
	let color = iter.next().unwrap();
	let rest = iter.next().unwrap();
	let mut rules = Vec::with_capacity(iter.count());

	for rule in rest[8..].split(", ") {
		if rule == "no other bags." {
			break;
		}
		let mut numbers_digits = 0;
		for c in rule.chars() {
			if c.is_ascii_digit() {
				numbers_digits += 1;
			} else {
				break;
			}
		}
		let bag_count = rule[..(numbers_digits)].parse::<usize>().unwrap();

		let rule_color = if rule.ends_with(".") {
			rule[(numbers_digits + 1)..(rule.len() - 5)] // remove " bags." and " bag."
				.trim()
		} else {
			rule[(numbers_digits + 1)..(rule.len() - 4)] // remove " bags" and " bag"
				.trim()
		};
		rules.push((rule_color, bag_count));
	}

	(color, rules)
}

fn contains_gold(current: &str, bags: &HashMap<&'static str, Vec<(&'static str, usize)>>, cache: &mut HashMap<&str, bool>) -> bool {
	match cache.get(current) {
		Some(val) => return *val,
		_ => (),
	}

	let current_info = bags.get(current).unwrap();
	let current_contains_gold = current_info
		.iter()
		.find(|el| el.0 == "shiny gold")
		.is_some();

	if current_contains_gold {
		return true;
	} else {
		for (bag, _) in current_info {
			if contains_gold(*bag, bags, cache) {
				cache.insert(*bag, true);
				return true;
			} else {
				cache.insert(*bag, false);
			}
		}
		return false;
	}
}
fn count_bags_in(current: &str, bags: &HashMap<&str, Vec<(&str, usize)>>) -> usize {
	let mut count = 0;
	for (bag_color, bags_in_bag) in bags.get(current).unwrap() {
		count += bags_in_bag + bags_in_bag * count_bags_in(&bag_color, bags);
	}
	count
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 151);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 41559);
	}

	#[bench]
	fn part_1_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_1())
	}
	#[bench]
	fn part_2_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_2())
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
