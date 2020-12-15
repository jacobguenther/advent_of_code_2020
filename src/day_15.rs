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

use std::collections::HashMap;

// number, turn last spoken, turn spoken before last
type NumbersSpoken = HashMap<usize, (usize, Option<usize>)>;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		15
	}
	fn new() -> Self {
		let starting_numbers = [0, 6, 1, 7, 2, 19, 20];
		let mut numbers_spoken = starting_numbers
			.iter()
			.enumerate()
			.map(|(i, n)| (*n, (i, None)))
			.collect::<NumbersSpoken>();

		let part_1_result = van_eck_sequence(
			&mut numbers_spoken,
			*starting_numbers.last().unwrap(),
			starting_numbers.len(),
			2020,
		);
		Self {
			part_1_result: part_1_result,
			part_2_result: van_eck_sequence(&mut numbers_spoken, part_1_result, 2020, 30_000_000),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
fn van_eck_sequence(
	numbers_spoken: &mut NumbersSpoken,
	last_spoken: usize,
	start: usize,
	nth: usize,
) -> usize {
	let mut previous_spoken = last_spoken;
	let mut previous_spoken_index = start;
	let mut previous_spoken_before = None;

	for current_i in start..nth {
		let current_spoken = match previous_spoken_before {
			Some(turn) => previous_spoken_index - turn,
			None => 0,
		};
		let current_last_spoken_on = match numbers_spoken.get(&current_spoken) {
			Some((t, _)) => Some(*t),
			None => None,
		};
		numbers_spoken.insert(current_spoken, (current_i, current_last_spoken_on));
		previous_spoken = current_spoken;
		previous_spoken_index = current_i;
		previous_spoken_before = current_last_spoken_on;
	}
	previous_spoken
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 706);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 19331);
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
