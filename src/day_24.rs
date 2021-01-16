// File: day_24.rs
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

use super::common::{vec3::Vec3, *};
use std::collections::HashMap;
use std::collections::HashSet;
use std::slice::Iter;

type Coord = Vec3<i16>;
type FlippedTiles = HashSet<Coord>;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		24
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_24.txt");
		let directions = parse_input(input);

		let floor = init_floor(&directions);
		let part_1_result = floor.len();

		let part_2_result = game_of_life(&floor);

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

fn parse_input(input: &str) -> Vec<Vec<HexDirection>> {
	let east = b'e';
	let south = b's';
	let west = b'w';
	let north = b'n';
	input
		.lines()
		.map(|line| {
			let bytes = line.as_bytes();
			let mut path = Vec::new();
			let mut i = 0;
			while i < line.len() {
				let direction = match bytes[i] {
					b1 if b1 == east => {
						i += 1;
						HexDirection::East
					}
					b1 if b1 == south => {
						i += 2;
						match bytes[i - 1] {
							b2 if b2 == east => HexDirection::SouthEast,
							b2 if b2 == west => HexDirection::SouthWest,
							b2 => panic!("{}{} not a supported direction", b1 as char, b2 as char),
						}
					}
					b1 if b1 == west => {
						i += 1;
						HexDirection::West
					}
					b1 if b1 == north => {
						i += 2;
						match bytes[i - 1] {
							b2 if b2 == west => HexDirection::NorthWest,
							b2 if b2 == east => HexDirection::NorthEast,
							b2 => panic!("{}{} not a supported direction", b1 as char, b2 as char),
						}
					}
					b1 => panic!("{} not a supported direction", b1 as char),
				};
				path.push(direction);
			}
			path
		})
		.collect()
}
fn init_floor(paths: &[Vec<HexDirection>]) -> FlippedTiles {
	let mut floor = FlippedTiles::new();
	for path in paths.iter() {
		let mut coord = Coord::new(0, 0, 0);
		for direction in path.iter() {
			let offset = direction.into();
			coord.add_other(&offset);
		}
		if !floor.insert(coord) {
			floor.remove(&coord);
		}
	}
	floor
}
fn game_of_life(floor: &FlippedTiles) -> usize {
	let mut floor = floor.clone();
	for _ in 0..100 {
		let mut adjacent_counts = HashMap::<Coord, u8>::new();
		for coord in floor.iter() {
			for direction in HexDirection::iterator() {
				let offset = direction.into();
				let mut adjacent = *coord;
				adjacent.add_other(&offset);
				let entry = adjacent_counts.entry(adjacent).or_insert(0);
				*entry += 1;
			}
		}
		floor = adjacent_counts
			.iter()
			.filter_map(|(coord, count)| {
				if *count == 2 || (*count == 1 && floor.contains(coord)) {
					Some(coord)
				} else {
					None
				}
			})
			.copied()
			.collect();
	}
	floor.len()
}

#[derive(Debug, Copy, Clone)]
enum HexDirection {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast,
}
impl HexDirection {
	pub fn iterator() -> Iter<'static, HexDirection> {
		static DIRECTIONS: [HexDirection; 6] = [
			HexDirection::East,
			HexDirection::SouthEast,
			HexDirection::SouthWest,
			HexDirection::West,
			HexDirection::NorthWest,
			HexDirection::NorthEast,
		];
		DIRECTIONS.iter()
	}
}
impl From<&HexDirection> for Vec3<i16> {
	fn from(dir: &HexDirection) -> Vec3<i16> {
		match dir {
			HexDirection::East => Vec3::new(1, -1, 0),
			HexDirection::SouthEast => Vec3::new(0, -1, 1),
			HexDirection::SouthWest => Vec3::new(-1, 0, 1),
			HexDirection::West => Vec3::new(-1, 1, 0),
			HexDirection::NorthWest => Vec3::new(0, 1, -1),
			HexDirection::NorthEast => Vec3::new(1, 0, -1),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 512);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 4120);
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
