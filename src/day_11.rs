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
type MapRow = Vec<Tile>;

pub struct Challenge {
	parsed_input: Vec<MapRow>,
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
			.collect::<Vec<MapRow>>();

		Self { parsed_input }
	}
	fn part_1(&self) -> Self::Output1 {
		let mut previous = Vec::with_capacity(self.parsed_input.len());
		let mut current = self.parsed_input.clone();
		while previous != current {
			previous = current;
			current = step_map(&previous, &p1_adjacency);
		}
		count_seats(&current)
	}
	fn part_2(&self) -> Self::Output2 {
		let mut previous = Vec::with_capacity(self.parsed_input.len());
		let mut current = self.parsed_input.clone();
		while previous != current {
			previous = current;
			current = step_map(&previous, &p2_adjacency);
		}
		count_seats(&current)
	}
}
fn step_map(
	current: &[MapRow],
	adjacency_fn: &dyn Fn(&[MapRow], &Tile, &mut [MapRow], usize, usize),
) -> Vec<MapRow> {
	let mut new = current.to_owned();
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
fn p1_adjacency(current: &[MapRow], current_tile: &Tile, new: &mut [MapRow], x: usize, y: usize) {
	let width = current.first().unwrap().len();
	let height = current.len();
	let adjacent_filled = steps().iter().fold(0, |mut acc, (dx, dy)| {
		let pos_x = (*dx + x as i32) as usize;
		let pos_y = (*dy + y as i32) as usize;
		if pos_x < width && pos_y < height {
			if let Tile::Filled = current[pos_y][pos_x] {
				acc += 1;
			}
		}
		acc
	});
	if adjacent_filled == 0 && *current_tile == Tile::Empty {
		new[y][x] = Tile::Filled;
	} else if adjacent_filled > 3 && *current_tile == Tile::Filled {
		new[y][x] = Tile::Empty;
	}
}
fn p2_adjacency(current: &[MapRow], current_tile: &Tile, new: &mut [MapRow], x: usize, y: usize) {
	let mut visible_filled_seats = 0;
	let height = current.len() as i32;
	let width = current[0].len() as i32;
	for (dx, dy) in steps() {
		let (mut pos_x, mut pos_y) = (x as i32, y as i32);
		loop {
			pos_x += dx;
			pos_y += dy;
			if pos_x < 0 || pos_y < 0 || pos_x >= width || pos_y >= height {
				break;
			} else {
				match current[pos_y as usize][pos_x as usize] {
					Tile::Filled => {
						visible_filled_seats += 1;
						break;
					}
					Tile::Empty => break,
					_ => (),
				}
			}
		}
	}
	if visible_filled_seats == 0 && *current_tile == Tile::Empty {
		new[y][x] = Tile::Filled;
	} else if visible_filled_seats > 4 && *current_tile == Tile::Filled {
		new[y][x] = Tile::Empty;
	}
}
const fn steps() -> &'static [(i32, i32); 8] {
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
fn count_seats(current: &[MapRow]) -> usize {
	current.iter().fold(0, |filled, row| {
		filled
			+ row.iter().fold(0, |part, tile| {
				if let Tile::Filled = tile {
					part + 1
				} else {
					part
				}
			})
	})
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
