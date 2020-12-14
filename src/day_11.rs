// File: day_11.rs
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
	Floor,
	Empty,
	Filled,
}
type SeatMap = Vec<Vec<Tile>>;

pub struct Challenge {
	parsed_input: Vec<Vec<Tile>>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		11
	}
	fn new() -> Self {
		let parsed_input = include_str!("../inputs/day_11.txt")
			.lines()
			.map(|line| {
				line.chars()
					.map(|c| match c {
						'.' => Tile::Floor,
						'L' => Tile::Empty,
						_ => Tile::Empty,
					})
					.collect()
			})
			.collect::<Vec<Vec<Tile>>>();

		Self {
			parsed_input: parsed_input,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		let mut previous = Vec::new();
		let mut current = self.parsed_input.clone();
		while previous != current {
			previous = current;
			current = step_map(&previous, &p1_adjacency);
		}
		count_seats(&current)
	}
	fn part_2(&self) -> Self::Output2 {
		let mut previous = Vec::new();
		let mut current = self.parsed_input.clone();
		while previous != current {
			previous = current;
			current = step_map(&previous, &p2_adjacency);
		}
		count_seats(&current)
	}
}
fn step_map(
	current: &SeatMap,
	adjacency_fn: &dyn Fn(&SeatMap, &Tile, &mut SeatMap, usize, usize) -> (),
) -> SeatMap {
	let mut new = current.clone();
	for (y, row) in current.iter().enumerate() {
		for (x, tile) in row.iter().enumerate() {
			if *tile == Tile::Floor {
				continue;
			}
			adjacency_fn(current, tile, &mut new, x, y);
		}
	}
	new
}
fn p1_adjacency(current: &SeatMap, current_tile: &Tile, new: &mut SeatMap, x: usize, y: usize) {
	let width = current.first().unwrap().len();
	let height = current.len();
	let adjacent_filled = steps().iter().fold(0, |mut acc, (dx, dy)| {
		let pos_x = (*dx + x as i32) as usize;
		let pos_y = (*dy + y as i32) as usize;
		if pos_x < width && pos_y < height {
			match current[pos_y][pos_x] {
				Tile::Filled => acc += 1,
				_ => (),
			}
		}
		acc
	});
	if adjacent_filled == 0 && *current_tile == Tile::Empty {
		new[y][x] = Tile::Filled;
	} else if adjacent_filled >= 4 && *current_tile == Tile::Filled {
		new[y][x] = Tile::Empty;
	}
}
fn p2_adjacency(current: &SeatMap, current_tile: &Tile, new: &mut SeatMap, x: usize, y: usize) {
	let mut visible_filled_seats = 0;
	for (dx, dy) in steps() {
		let (mut px, mut py) = (x as i32, y as i32);
		loop {
			px += dx;
			py += dy;
			if current.get(py as usize).is_some() && current[py as usize].get(px as usize).is_some()
			{
				match current[py as usize][px as usize] {
					Tile::Filled => {
						visible_filled_seats += 1;
						break;
					}
					Tile::Empty => break,
					_ => (),
				}
			} else {
				break;
			}
		}
	}
	if visible_filled_seats == 0 && *current_tile == Tile::Empty {
		new[y][x] = Tile::Filled;
	} else if visible_filled_seats >= 5 && *current_tile == Tile::Filled {
		new[y][x] = Tile::Empty;
	}
}
fn steps() -> &'static [(i32, i32); 8] {
	&[
		(-1, -1),
		(-1, 0),
		(-1, 1),
		(0, -1),
		(0, 1),
		(1, -1),
		(1, 0),
		(1, 1),
	]
}
fn count_seats(current: &SeatMap) -> usize {
	let mut total_filled = 0;
	for row in current.iter() {
		for tile in row {
			match tile {
				Tile::Filled => total_filled += 1,
				_ => (),
			}
		}
	}
	total_filled
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 2386);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 2091);
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
