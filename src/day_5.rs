// File: day_5.rs
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
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		5
	}
	fn new() -> Self {
		let mut lowest = usize::MAX;
		let mut highest = 0;
		let mut filled_seat_ids = vec![0; 128 * 8];
		include_str!("../inputs/day_5.txt")
			.lines()
			.for_each(|line| {
				let seat_id = get_id(line);
				filled_seat_ids[seat_id] = seat_id;
				if seat_id > highest {
					highest = seat_id;
				}
				if seat_id != 0 && seat_id < lowest {
					lowest = seat_id;
				}
			});

		let (previous, _) = &filled_seat_ids[lowest..]
			.iter()
			.zip(&filled_seat_ids[(lowest + 1)..])
			.find(|(previous, next)| **previous + 1 != **next)
			.unwrap();

		Self {
			part_1_result: highest,
			part_2_result: *previous + 1,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}

// Note: 'B' is 66 and 'R' is 82
fn get_id(line: &str) -> usize {
	line.bytes().fold(0, |acc, b| {
		acc * 2
			+ match b {
				c if c == 66 || c == 82 => 1,
				_ => 0,
			}
	})
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 974);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 646);
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
