// File: day_n.rs
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
use std::collections::HashSet;

use super::common::*;

pub struct Challenge {
	parsed_input: Vec<&'static str>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		n
	}
	fn new() -> Self {
		let parsed_input = include_str!("../inputs/day_n.txt")
			.lines()
			.filter_map(|line| {
				None
			})
			.collect();
		Self {
			parsed_input: parsed_input,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		0
	}
	fn part_2(&self) -> Self::Output2 {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 0);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 0);
	}

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_1() )
	}
    #[bench]
    fn part_2_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_2() )
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
