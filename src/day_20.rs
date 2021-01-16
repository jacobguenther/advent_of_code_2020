// File: day_20.rs
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

use super::common::{grid::*, *};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		20
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_20.txt");
		let tiles = parse_input(input);
		let adjacency_list = build_adjacency_list(&tiles);

		let part_1_result = solve_1(&adjacency_list);
		let part_2_result = solve_2(&tiles, &adjacency_list);

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

fn parse_input(input: &str) -> Vec<Tile> {
	let mut tiles = Vec::<Tile>::with_capacity(144);
	input.split("\n\n").for_each(|s| {
		let mut lines = s.lines();
		let mut tile_id_split = lines.next().unwrap().split(|c| c == ' ' || c == ':');
		let id = tile_id_split.nth(1).unwrap().parse::<u32>().unwrap();

		let mut top = 0;
		let mut right = 0;
		let mut bottom = 0;
		let mut left = 0;

		let mut grid = Grid::new(8, 8, &Pixel::default());
		lines.enumerate().for_each(|(y, line)| {
			line.bytes().enumerate().for_each(|(x, byte)| {
				let pixel = Pixel::try_from(byte).unwrap();
				if !(x == 0 || y == 0 || x == 9 || y == 9) {
					grid.set(x - 1, y - 1, &pixel);
				} else {
					let pixel_i = match pixel {
						Pixel::Black => 0,
						Pixel::White => 1,
					};
					let push_bit = |side: &mut u16, bit: u16| {
						*side <<= 1;
						*side += bit;
					};
					match y {
						0 => push_bit(&mut top, pixel_i),
						9 => push_bit(&mut bottom, pixel_i),
						_ => (),
					}
					match x {
						0 => push_bit(&mut left, pixel_i),
						9 => push_bit(&mut right, pixel_i),
						_ => (),
					}
				}
			});
		});
		tiles.push(Tile::new(id, &grid, &[top, right, bottom, left]));
	});
	tiles.sort();
	tiles
}
fn build_adjacency_list(tiles: &[Tile]) -> HashMap<u32, Vec<u32>> {
	tiles
		.iter()
		.map(|tile| {
			let tile_boarders = tile.all_boarders_iter().cloned().collect::<Vec<_>>();
			let adjacent = tiles
				.iter()
				.filter_map(|other| {
					if other.id == tile.id {
						None
					} else {
						for other_boarder in other.all_boarders_iter() {
							for tile_boarder in tile_boarders.iter() {
								if *other_boarder == *tile_boarder {
									return Some(other.id);
								}
							}
						}
						None
					}
				})
				.collect();
			(tile.id, adjacent)
		})
		.collect()
}
fn solve_1(adjacency_list: &HashMap<u32, Vec<u32>>) -> usize {
	let mut product = 1;
	for (id, a) in adjacency_list.iter() {
		if a.len() == 2 {
			product *= *id as usize;
		}
	}
	product
}

fn solve_2(tiles: &[Tile], adjacency_list: &HashMap<u32, Vec<u32>>) -> usize {
	let mut start = tiles[0].clone();
	for (id, adjacent_ids) in adjacency_list.iter() {
		let i = tiles.binary_search_by(|t| t.id.cmp(&id)).unwrap();
		if 2 == adjacent_ids.len() {
			start = tiles[i].clone();
			break;
		}
	}

	let mut img = Grid::<Option<Tile>>::new(12, 12, &None);

	let starting_adjacent = adjacency_list
		.get(&start.id)
		.unwrap()
		.iter()
		.map(|id| {
			let i = tiles.binary_search_by(|t| t.id.cmp(id)).unwrap();
			tiles[i].clone()
		})
		.collect::<Vec<_>>();

	let a = starting_adjacent.get(0).unwrap();
	let b = starting_adjacent.get(1).unwrap();

	let (s1, s1_rev) = shared_side_value(&start, a);
	let (s2, s2_rev) = shared_side_value(&start, b);

	let start_in_correct_orientation =
		|right: u16, bottom: u16, s1: u16, s1_rev: u16, s2: u16, s2_rev: u16| {
			((right == s1 || right == s1_rev) && (bottom == s2 || bottom == s2_rev))
				|| ((right == s2 || right == s2_rev) && (bottom == s1 || bottom == s1_rev))
		};
	loop {
		let right = start.get_boarder(Side::Right);
		let bottom = start.get_boarder(Side::Bottom);
		if start_in_correct_orientation(right, bottom, s1, s1_rev, s2, s2_rev) {
			break;
		}
		let mut temp = start.clone();
		temp.flip(FlipDirection::Horizontal);
		let right = temp.get_boarder(Side::Right);
		let bottom = temp.get_boarder(Side::Bottom);
		if start_in_correct_orientation(right, bottom, s1, s1_rev, s2, s2_rev) {
			start = temp;
			break;
		} else {
			temp.flip(FlipDirection::Vertical);
			let right = temp.get_boarder(Side::Right);
			let bottom = temp.get_boarder(Side::Bottom);
			if start_in_correct_orientation(right, bottom, s1, s1_rev, s2, s2_rev) {
				start = temp;
				break;
			}
		}
		let mut temp = start.clone();
		temp.flip(FlipDirection::Vertical);
		let right = temp.get_boarder(Side::Right);
		let bottom = temp.get_boarder(Side::Bottom);
		if start_in_correct_orientation(right, bottom, s1, s1_rev, s2, s2_rev) {
			start = temp;
			break;
		} else {
			temp.flip(FlipDirection::Horizontal);
			let right = temp.get_boarder(Side::Right);
			let bottom = temp.get_boarder(Side::Bottom);
			if start_in_correct_orientation(right, bottom, s1, s1_rev, s2, s2_rev) {
				start = temp;
				break;
			}
		}
		start.rotate();
	}
	img.set(0, 0, &Some(start));

	for y in 1..12 {
		let previous = img.get(0, y - 1).unwrap().as_ref().unwrap();
		let side_to_match = previous.get_boarder(Side::Bottom);
		let adjacent = adjacency_list.get(&previous.id).unwrap();
		let mut current = None;
		for id in adjacent.iter() {
			let i = match tiles.binary_search_by(|t| t.id.cmp(id)) {
				Ok(i) => i,
				Err(_) => continue,
			};
			for s in tiles[i].all_boarders_iter() {
				if *s == side_to_match {
					current = Some(tiles[i].clone());
					break;
				}
			}
			if current.is_some() {
				break;
			}
		}
		let mut current = current.unwrap();
		orient_tile(&mut current, Side::Top, side_to_match);
		img.set(0, y, &Some(current));
	}
	for y in 0..12 {
		for x in 1..12 {
			let previous = img.get(x - 1, y).unwrap().as_ref().unwrap();
			let side_to_match = previous.get_boarder(Side::Right);
			let adjacent = adjacency_list.get(&previous.id).unwrap();
			let mut current = None;
			for id in adjacent.iter() {
				let i = match tiles.binary_search_by(|t| t.id.cmp(id)) {
					Ok(i) => i,
					Err(_) => continue,
				};
				for s in tiles[i].all_boarders_iter() {
					if *s == side_to_match {
						current = Some(tiles[i].clone());
						break;
					}
				}
				if current.is_some() {
					break;
				}
			}
			let mut current = current.unwrap();
			orient_tile(&mut current, Side::Left, side_to_match);
			img.set(x, y, &Some(current));
		}
	}

	let size = 8 * 12;
	let mut image = Grid::<Pixel>::new(size, size, &Pixel::Black);
	let mut rough_water_count = 0;
	for t_x in 0..12 {
		let offset_x = t_x * 8;
		for t_y in 0..12 {
			let offset_y = t_y * 8;
			let tile = img.get(t_x, t_y).unwrap().as_ref().unwrap();
			for x in 0..8 {
				for y in 0..8 {
					let pixel = tile.grid.get(x, y).unwrap();
					image.set(x + offset_x, y + offset_y, pixel);
					if let Pixel::Black = pixel {
						rough_water_count += 1;
					}
				}
			}
		}
	}

	let sea_monster_count;
	loop {
		if let Some(count) = count_sea_monsters(&image) {
			sea_monster_count = count;
			break;
		}
		let mut temp = image.clone();
		temp.flip(FlipDirection::Horizontal);
		if let Some(count) = count_sea_monsters(&temp) {
			sea_monster_count = count;
			break;
		} else {
			temp.flip(FlipDirection::Vertical);
			if let Some(count) = count_sea_monsters(&temp) {
				sea_monster_count = count;
				break;
			}
		}
		let mut temp = image.clone();
		temp.flip(FlipDirection::Vertical);
		if let Some(count) = count_sea_monsters(&temp) {
			sea_monster_count = count;
			break;
		} else {
			temp.flip(FlipDirection::Horizontal);
			if let Some(count) = count_sea_monsters(&temp) {
				sea_monster_count = count;
				break;
			}
		}
		image.rotate();
	}

	rough_water_count - sea_monster_count * 15
}
fn orient_tile(current: &mut Tile, side: Side, side_to_match: u16) {
	loop {
		if current.get_boarder(side) == side_to_match {
			break;
		}
		let mut temp = current.clone();
		temp.flip(FlipDirection::Horizontal);
		if temp.get_boarder(side) == side_to_match {
			*current = temp;
			break;
		} else {
			temp.flip(FlipDirection::Vertical);
			if temp.get_boarder(side) == side_to_match {
				*current = temp;
				break;
			}
		}
		let mut temp = current.clone();
		temp.flip(FlipDirection::Vertical);
		if temp.get_boarder(side) == side_to_match {
			*current = temp;
			break;
		} else {
			temp.flip(FlipDirection::Horizontal);
			if temp.get_boarder(side) == side_to_match {
				*current = temp;
				break;
			}
		}
		current.rotate();
	}
}
fn count_sea_monsters(image: &Grid<Pixel>) -> Option<usize> {
	let mut count = 0;
	let size_x = image.size.x;
	let size_y = image.size.y;
	let mut y = 0;
	while y < size_y - 2 {
		let mut x = 0;
		while x < size_x - 19 {
			if *image.get(x + 18, y).unwrap() == Pixel::Black
				&& *image.get(x, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 5, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 6, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 11, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 12, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 17, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 18, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 19, y + 1).unwrap() == Pixel::Black
				&& *image.get(x + 1, y + 2).unwrap() == Pixel::Black
				&& *image.get(x + 4, y + 2).unwrap() == Pixel::Black
				&& *image.get(x + 7, y + 2).unwrap() == Pixel::Black
				&& *image.get(x + 10, y + 2).unwrap() == Pixel::Black
				&& *image.get(x + 13, y + 2).unwrap() == Pixel::Black
				&& *image.get(x + 16, y + 2).unwrap() == Pixel::Black
			{
				count += 1;
				x += 19;
				continue;
			}
			x += 1;
		}
		y += 1;
	}
	if count > 0 {
		Some(count)
	} else {
		None
	}
}
fn shared_side_value(tile: &Tile, other: &Tile) -> (u16, u16) {
	for tile_boarder in tile.all_boarders_iter() {
		for other_boarder in other.all_boarders_iter() {
			if *tile_boarder == *other_boarder {
				return (*tile_boarder, Tile::reverse_bits(*tile_boarder));
			}
		}
	}
	(0, 0)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel {
	Black,
	White,
}
impl Default for Pixel {
	fn default() -> Pixel {
		Pixel::Black
	}
}
use std::convert::TryFrom;
impl TryFrom<u8> for Pixel {
	type Error = ();
	fn try_from(byte: u8) -> Result<Self, Self::Error> {
		match byte {
			35 => Ok(Pixel::Black), // #
			46 => Ok(Pixel::White), // .
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone)]
struct Tile {
	id: u32,
	grid: Grid<Pixel>,
	boarders: [u16; 4],
	reversed_boarders: [u16; 4],
	rotation: Rotation,
}
impl Tile {
	fn new(id: u32, grid: &Grid<Pixel>, boarders: &[u16; 4]) -> Tile {
		Tile {
			id,
			grid: grid.clone(),
			boarders: *boarders,
			reversed_boarders: [
				Self::reverse_bits(boarders[0]),
				Self::reverse_bits(boarders[1]),
				Self::reverse_bits(boarders[2]),
				Self::reverse_bits(boarders[3]),
			],
			rotation: Rotation::R0,
		}
	}
	fn reverse_bits(side: u16) -> u16 {
		side.reverse_bits() >> 6
	}
	fn all_boarders_iter(&self) -> impl Iterator<Item = &u16> {
		self.boarders.iter().chain(self.reversed_boarders.iter())
	}

	fn rotate(&mut self) {
		self.grid.rotate();
		self.rotate_boarders();
	}
	fn rotate_boarders(&mut self) {
		self.boarders.rotate_right(1);
		self.reversed_boarders.rotate_right(1);
		std::mem::swap(&mut self.boarders[0], &mut self.reversed_boarders[0]);
		std::mem::swap(&mut self.boarders[2], &mut self.reversed_boarders[2]);
		self.rotation = self.rotation.next().unwrap();
	}

	fn flip(&mut self, direction: FlipDirection) {
		self.grid.flip(direction);
		self.flip_boarders(direction);
	}
	fn flip_boarders(&mut self, direction: FlipDirection) {
		match direction {
			FlipDirection::Horizontal => {
				self.boarders.swap(0, 2);
				self.reversed_boarders.swap(0, 2);
				std::mem::swap(&mut self.boarders[1], &mut self.reversed_boarders[1]);
				std::mem::swap(&mut self.boarders[3], &mut self.reversed_boarders[3]);
			}
			FlipDirection::Vertical => {
				self.boarders.swap(1, 3);
				self.reversed_boarders.swap(1, 3);
				std::mem::swap(&mut self.boarders[0], &mut self.reversed_boarders[0]);
				std::mem::swap(&mut self.boarders[2], &mut self.reversed_boarders[2]);
			}
		}
	}

	fn get_boarder(&self, side: Side) -> u16 {
		match side {
			Side::Top => self.boarders[0],
			Side::Right => self.boarders[1],
			Side::Bottom => self.boarders[2],
			Side::Left => self.boarders[3],
		}
	}
}
impl Ord for Tile {
	fn cmp(&self, other: &Self) -> Ordering {
		self.id.cmp(&other.id)
	}
}
impl PartialOrd for Tile {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl PartialEq for Tile {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Tile {}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 28057939502729);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 2489);
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
