// File: day_6.rs
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

use std::collections::HashSet;

use super::common::ChallengeT;

pub struct Challenge {
	part_1_answer: usize,
	part_2_answer: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		6
	}
	fn new() -> Self {
		let [p1, p2] = include_str!("../inputs/day_6.txt")
			.split("\n\n")
			.map(|group| {
				// part 1
				let mut group_answers = group.split_whitespace().fold(
					Vec::<u8>::with_capacity(group.len()),
					|mut acc, s| {
						acc.extend(s.as_bytes());
						acc
					},
				);
				group_answers.sort();
				group_answers.dedup();
				let group_answers_count = group_answers.len();

				// part 2
				let member_answers = group
					.split_whitespace()
					.map(|answers| answers.bytes().collect::<HashSet<_>>())
					.collect::<Vec<_>>();

				let group_all_yes_answers_count = member_answers[1..]
					.iter()
					.fold(member_answers[0].clone(), |intersected, members_answers| {
						intersected
							.intersection(members_answers)
							.map(|c| *c)
							.collect()
					})
					.len();

				[group_answers_count, group_all_yes_answers_count]
			})
			.fold([0, 0], |sum, partial| {
				[sum[0] + partial[0], sum[1] + partial[1]]
			});

		Self {
			part_1_answer: p1,
			part_2_answer: p2,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_answer
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_answer
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 6735);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 3221);
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
