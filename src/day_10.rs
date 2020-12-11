// File: day_10.rs
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

/*
19
16
15
12
11
10
7
6
5
4
1
*/
/*
3
1
3
1
1
3
1
1
1
3
*/

use super::common::ChallengeT;
use std::collections::HashMap;

pub struct Challenge {
	adapters: Vec<usize>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		10
	}
	fn new() -> Self {
		let mut adapters = include_str!("../inputs/day_10.txt")
			.lines()
			.map(|line| line.parse().unwrap())
			.collect::<Vec<usize>>();
		adapters.sort();

		Self { adapters: adapters }
	}
	fn part_1(&self) -> Self::Output1 {
		let [difference_1, difference_3] = self.adapters.iter().zip(&self.adapters[1..]).fold(
			[1, 1],
			|mut acc, (current, next)| {
				if current + 1 == *next {
					acc[0] += 1;
				} else if current + 3 == *next {
					acc[1] += 1;
				}
				acc
			},
		);
		difference_1 * difference_3
	}
	fn part_2(&self) -> Self::Output2 {
		let mut adapters = self.adapters.clone();
		adapters.insert(0, 0);
		find_permutations(&adapters, &mut HashMap::new())
	}
}

fn find_permutations(adapters: &[usize], cache: &mut HashMap<usize, usize>) -> usize {
	match adapters.split_first().unwrap() {
		(_, []) => 1,
		(first, rest) => rest
			.iter()
			.take_while(|a| *a - first <= 3)
			.enumerate()
			.map(|(i, val)| {
				match cache.get(val) {
					Some(v) => *v,
					None => {
						let partial_count = find_permutations(&rest[i..], cache);
						cache.insert(*val, partial_count);
						partial_count
					}
				}
			})
			.sum(),
	}
}
#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 1998);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 347250213298688);
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
