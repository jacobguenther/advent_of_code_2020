// File: day_9.rs
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
	nums: Vec<usize>,
	part_1_answer: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		9
	}
	fn new() -> Self {
		let nums = include_str!("../inputs/day_9.txt")
			.lines()
			.map(|line| line.parse().unwrap())
			.collect::<Vec<usize>>();

		let preamble_len = 25;

		let mut error_num = 0;
		for current in preamble_len..nums.len() {
			let mut found_sum = false;
			for lower in &nums[(current - preamble_len)..(current - 1)] {
				for upper in &nums[(current - preamble_len + 1)..current] {
					if nums[current] == lower + upper {
						found_sum = true;
						break;
					}
				}
				if found_sum {
					break;
				}
			}
			if !found_sum {
				error_num = nums[current];
				break;
			}
		}

		Self {
			nums: nums,
			part_1_answer: error_num,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_answer
	}
	fn part_2(&self) -> Self::Output2 {
		let mut smallest = 0;
		let mut largest = 0;
		let mut found_range = false;
		for start in 0..self.nums.len() - 1 {
			for end in (start + 1)..self.nums.len() {
				let sum: usize = self.nums[start..end + 1].iter().sum();
				if sum == self.part_1_answer {
					let mut sum_range = self.nums[start..end + 1].iter().collect::<Vec<_>>();
					sum_range.sort();
					smallest = *sum_range[0];
					largest = **sum_range.last().unwrap();
					found_range = true;
					break;
				}
			}
			if found_range {
				break;
			}
		}
		smallest + largest
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 15690279);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 2174232);
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
