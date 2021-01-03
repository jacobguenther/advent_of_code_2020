// File: day_1.rs
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
	report: Vec<u32>,
}
impl ChallengeT for Challenge {
	type Output1 = u32;
	type Output2 = u32;

	fn day() -> u8 {
		1
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_1.txt");
		let mut report = input
			.lines()
			.map(|l| l.parse().unwrap())
			.collect::<Vec<u32>>();
		report.sort();
		Self { report }
	}
	fn part_1(&self) -> Self::Output1 {
		let mut lower_i = 0;
		let mut upper_i = self.report.len() - 1;
		loop {
			let lower = self.report[lower_i];
			let upper = self.report[upper_i];
			let sum = upper + lower;
			match sum {
				2020 => return upper * lower,
				s if s < 2020 => lower_i += 1,
				_ => upper_i -= 1,
			}
		}
	}
	fn part_2(&self) -> Self::Output2 {
		let len = self.report.len();
		for i in 0..(len - 2) {
			let mut lower_i = i + 1;
			let mut upper_i = len - 1;
			loop {
				let first = self.report[i];
				let second = self.report[lower_i];
				let third = self.report[upper_i];
				let sum = first + second + third;
				match sum {
					2020 => return first * second * third,
					s if s < 2020 => lower_i += 1,
					_ => upper_i -= 1,
				}
				if lower_i == upper_i {
					break;
				}
			}
		}
		0
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1() {
		assert_eq!(Challenge::new().part_1(), 545379);
	}
	#[test]
	fn part_2() {
		assert_eq!(Challenge::new().part_2(), 257778836);
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
