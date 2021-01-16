// File: day_13.rs
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

use super::common::{chinese_remainder_theorem::chinese_remainder_theorem, ChallengeT};

pub struct Challenge {
	part_1_result: usize,
	part_2_result: u128,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = u128;

	fn day() -> u8 {
		13
	}
	fn new() -> Self {
		let lines = include_str!("../inputs/day_13.txt")
			.lines()
			.collect::<Vec<&str>>();

		let ealiest_departure = lines[0].parse::<usize>().unwrap();
		let buses = lines[1]
			.split(',')
			.enumerate()
			.filter_map(|(i, b)| match b {
				"x" => None,
				_ => Some((i, b.parse().unwrap())),
			})
			.collect::<Vec<(usize, usize)>>();

		// part 1
		let mut lowest = usize::MAX;
		let mut best_bus = 0;
		buses.iter().for_each(|(_, b)| {
			let mut dep = *b;
			dep = (ealiest_departure as f64 / dep as f64).ceil() as usize * dep;
			if dep < lowest {
				lowest = dep;
				best_bus = *b;
			}
		});

		// part 2
		let mut residues = Vec::new();
		let mut modulii = Vec::new();
		buses[1..].iter().for_each(|(i, b)| {
			let mut j = -(*i as i128);
			while j < 0 {
				j += *b as i128;
			}
			residues.push(j as u128);
			modulii.push(*b as u128);
		});
		let (mut part_2_result, modulus) = chinese_remainder_theorem(&residues, &modulii);
		let (_, bus_id) = buses[0];
		let first_bus = bus_id as u128;
		loop {
			if part_2_result % first_bus == 0 {
				break;
			}
			part_2_result += modulus;
		}

		Self {
			part_1_result: best_bus * (lowest - ealiest_departure),
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

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 2406);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 225850756401039);
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
