// File: day_3.rs
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

use super::common::{
	ChallengeT,
};

type TreeMap = Vec<Vec<bool>>;

pub struct Challenge {
	tree_map: TreeMap,
	part_1_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		3
	}
	fn new() -> Self {
		let tree_map = include_str!("../inputs/day_3.txt")
			.lines()
			.map(|line| line.chars().map(|c| c == '#' ).collect() )
			.collect();
		Self {
			part_1_result: count_trees_hit(&tree_map, 3, 1),
			tree_map: tree_map,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		let steps = [(1, 1), (5, 1), (7, 1), (1, 2)];
		self.part_1_result * steps.iter()
			.map(|step| count_trees_hit(&self.tree_map, step.0, step.1) )
		.product::<usize>()
	}
}
fn count_trees_hit(tree_map: &TreeMap, step_x: usize, step_y: usize) -> usize {
	let mut pos_x = 0;
	tree_map.iter()
		.enumerate()
		.filter(|(i, _)| i % step_y == 0 )
		.fold(0, |acc, (_, line)| {
			if pos_x >= line.len() {
				pos_x = pos_x - line.len();
			}
			let i = pos_x;
			pos_x += step_x;
			acc + line[i] as usize
		})
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::{
		ChallengeT,
	};
	use test::{
		Bencher,
		// black_box
	};

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 156);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 3521829480);
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
