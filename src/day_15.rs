// File: day_15.rs
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
	part_1_result: i32,
	part_2_result: i32,
}
impl ChallengeT for Challenge {
	type Output1 = i32;
	type Output2 = i32;

	fn day() -> u8 {
		15
	}
	fn new() -> Self {
		let starting_numbers = [0, 6, 1, 7, 2, 19, 20];
		// index is number, value is the turn it was spoken on
		let mut turn_spoken_on = vec![-1; 30_000_000];
		for (i, num) in starting_numbers.iter().enumerate() {
			turn_spoken_on[*num as usize] = i as i32;
		}

		let mut previous_spoken = *starting_numbers.last().unwrap() as usize;
		let mut part_1_result = 0;
		let mut part_2_result = 0;

		let start = starting_numbers.len() as i32;
		for current_turn in start..30_000_000 {
			let spoken = match turn_spoken_on[previous_spoken] {
				-1 => 0,
				last_spoken_on => current_turn - last_spoken_on - 1,
			};
			turn_spoken_on[previous_spoken] = current_turn - 1;
			if current_turn == 2020 - 1 {
				part_1_result = spoken;
			} else if current_turn == 30_000_000 - 1 {
				part_2_result = spoken;
			}
			previous_spoken = spoken as usize;
		}
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

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	// use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 706);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 19331);
	}
}
