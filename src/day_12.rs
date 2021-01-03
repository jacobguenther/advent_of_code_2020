// File: day_12.rs
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

use super::common::vec2::Vec2;
use super::common::ChallengeT;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		12
	}
	fn new() -> Self {
		let res = include_str!("../inputs/day_12.txt")
			.lines()
			.map(|line| {
				let distance = line[1..].parse().unwrap();
				match &line[0..1] {
					"N" => Direction::North(distance),
					"S" => Direction::South(distance),
					"E" => Direction::East(distance),
					"W" => Direction::West(distance),
					"L" => Direction::Left(distance),
					"R" => Direction::Right(distance),
					"F" => Direction::Forward(distance),
					_ => panic!("Error while parsing input"),
				}
			})
			.fold(Payload::new(), |mut acc, dir| {
				match dir {
					Direction::Forward(dist) => {
						match acc.facing {
							Direction::North(_) => acc.distance.y += dist,
							Direction::South(_) => acc.distance.y -= dist,
							Direction::East(_) => acc.distance.x += dist,
							Direction::West(_) => acc.distance.x -= dist,
							_ => (),
						}

						acc.actual_distance.x += dist * acc.waypoint.x;
						acc.actual_distance.y += dist * acc.waypoint.y;
					}

					Direction::Left(angle) => {
						acc.facing = acc.facing.rotate_left(angle);
						acc.waypoint = rotate_waypoint_left(&acc.waypoint, angle);
					}
					Direction::Right(angle) => {
						acc.facing = acc.facing.rotate_right(angle);
						acc.waypoint = rotate_waypoint_right(&acc.waypoint, angle);
					}

					Direction::North(dist) => {
						acc.distance.y += dist;
						acc.waypoint.y += dist;
					}
					Direction::South(dist) => {
						acc.distance.y -= dist;
						acc.waypoint.y -= dist;
					}
					Direction::East(dist) => {
						acc.distance.x += dist;
						acc.waypoint.x += dist;
					}
					Direction::West(dist) => {
						acc.distance.x -= dist;
						acc.waypoint.x -= dist;
					}
				}
				acc
			});
		let part_1_result = res.distance.x.abs() + res.distance.y.abs();
		let part_2_result = res.actual_distance.x.abs() + res.actual_distance.y.abs();
		Self {
			part_1_result: part_1_result as usize,
			part_2_result: part_2_result as usize,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
#[derive(Copy, Clone, Debug)]
struct Payload {
	facing: Direction,
	distance: Vec2<isize>,

	waypoint: Vec2<isize>,
	actual_distance: Vec2<isize>,
}
impl Payload {
	fn new() -> Self {
		Self {
			facing: Direction::East(0),
			distance: Vec2::new(0, 0),

			waypoint: Vec2::new(10, 1),
			actual_distance: Vec2::new(0, 0),
		}
	}
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
	North(isize),
	South(isize),
	East(isize),
	West(isize),
	Left(isize),
	Right(isize),
	Forward(isize),
}
impl Direction {
	fn rotate_left(&self, amount: isize) -> Self {
		let mut new = *self;
		for _ in 0..amount / 90 {
			new = match new {
				Direction::North(d) => Direction::West(d),
				Direction::West(d) => Direction::South(d),
				Direction::South(d) => Direction::East(d),
				Direction::East(d) => Direction::North(d),
				_ => panic!("Cannot rotate a rotation"),
			}
		}
		new
	}
	fn rotate_right(&self, amount: isize) -> Self {
		let mut new = *self;
		for _ in 0..amount / 90 {
			new = match new {
				Direction::North(d) => Direction::East(d),
				Direction::East(d) => Direction::South(d),
				Direction::South(d) => Direction::West(d),
				Direction::West(d) => Direction::North(d),
				_ => panic!("Cannot rotate a rotation"),
			}
		}
		new
	}
}
fn rotate_waypoint_left(waypoint: &Vec2<isize>, angle: isize) -> Vec2<isize> {
	match angle {
		90 => Vec2::new(-waypoint.y, waypoint.x),
		180 => Vec2::new(-waypoint.x, -waypoint.y),
		270 => Vec2::new(waypoint.y, -waypoint.x),
		_ => panic!("Rotation not supported"),
	}
}
fn rotate_waypoint_right(waypoint: &Vec2<isize>, angle: isize) -> Vec2<isize> {
	match angle {
		270 => Vec2::new(-waypoint.y, waypoint.x),
		180 => Vec2::new(-waypoint.x, -waypoint.y),
		90 => Vec2::new(waypoint.y, -waypoint.x),
		_ => panic!("Rotation not supported"),
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 1186);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 47806);
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
