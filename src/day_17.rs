// File: day_18.rs
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

use super::common::{vec3::Vec3, vec4::Vec4, *};

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		17
	}
	fn new() -> Self {
		let mut active = HashSet::new();
		let mut active_2 = HashSet::new();
		include_str!("../inputs/day_17.txt")
			.lines()
			.enumerate()
			.for_each(|(y, line)| {
				line.bytes().enumerate().for_each(|(x, b)| {
					if b == 35 {
						active.insert(Vec3::<i32>::new(x as i32, y as i32, 0));
						active_2.insert(Vec4::<i32>::new(x as i32, y as i32, 0, 0));
					}
				});
			});

		let mut min = 0;
		let mut max = 7;

		let mut next_active = HashSet::new();
		let mut next_active_2 = HashSet::new();
		for step in 0..6 {
			next_active.clear();
			next_active_2.clear();
			min -= 1;
			max += 1;
			for x in min..max {
				for y in min..max {
					for z in 0..(step + 2) {
						let coord = Vec3::new(x, y, z);
						let active_count = count_active_adjacent(&active, &coord);
						match (active.get(&coord), active_count) {
							(Some(_), 2) | (Some(_), 3) | (None, 3) => {
								next_active.insert(coord);
								next_active.insert(Vec3::new(x, y, -z));
							}
							_ => (),
						}
						for w in 0..(step + 2) {
							let coord = Vec4::new(x, y, z, w);
							let active_count = count_active_adjacent(&active_2, &coord);
							match (active_2.get(&coord), active_count) {
								(Some(_), 2) | (Some(_), 3) | (None, 3) => {
									next_active_2.insert(coord);
									next_active_2.insert(Vec4::new(x, y, z, -w));
									next_active_2.insert(Vec4::new(x, y, -z, w));
									next_active_2.insert(Vec4::new(x, y, -z, -w));
								}
								_ => (),
							}
						}
					}
				}
			}
			std::mem::swap(&mut active, &mut next_active);
			std::mem::swap(&mut active_2, &mut next_active_2);
		}
		Self {
			part_1_result: active.iter().count(),
			part_2_result: active_2.iter().count(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
fn count_active_adjacent<T>(active: &HashSet<T>, coord: &T) -> usize
where
	T: NeighborsT + std::cmp::Eq + std::hash::Hash + Copy,
{
	coord
		.neighbors()
		.iter()
		.fold(0, |count, coord| match active.get(&coord) {
			Some(_) => count + 1,
			None => count,
		})
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 218);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 1908);
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
