// File: common/grid.rs
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

use std::fmt;

use super::vec2::*;

pub trait GridT<T> {
	fn new(x: usize, y: usize, default: &T) -> Self;
	fn get(&self, x: usize, y: usize) -> Option<&T>;
	fn set(&mut self, x: usize, y: usize, value: &T);

	fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
	fn diagonal_adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)>;

	fn rotate(&mut self);
	fn flip(&mut self, flip_direction: FlipDirection);
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
	pub size: Vec2<usize>,
	pub data: Vec<T>,
}
impl<T> Grid<T> {
	#[inline(always)]
	pub fn index(&self, x: usize, y: usize) -> usize {
		y * self.size.x + x
	}
}
impl<T> GridT<T> for Grid<T>
where
	T: Clone,
{
	fn new(x: usize, y: usize, default: &T) -> Self {
		let s = x * y;
		let mut data = Vec::with_capacity(x);
		for _ in 0..s {
			data.push(default.clone());
		}
		Self {
			size: Vec2::new(x, y),
			data,
		}
	}
	#[inline(always)]
	fn get(&self, x: usize, y: usize) -> Option<&T> {
		assert!(x < self.size.x);
		assert!(y < self.size.y);
		self.data.get(self.index(x, y))
	}
	#[inline(always)]
	fn set(&mut self, x: usize, y: usize, value: &T) {
		let i = self.index(x, y);
		self.data[i] = value.clone();
	}
	fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
		let a = &[(-1, 0), (0, -1), (0, 1), (1, 0)];
		let x_i = x as i32;
		let y_i = y as i32;
		let size_i = Vec2::<i32>::new(self.size.x as i32, self.size.y as i32);
		let mut adjacent = Vec::new();
		for (dx, dy) in a.iter() {
			let new_x = dx + x_i;
			let new_y = dy + y_i;
			if new_x >= 0 && new_x < size_i.x && new_y >= 0 && new_y < size_i.y {
				adjacent.push((new_x as usize, new_y as usize));
			}
		}
		adjacent
	}
	fn diagonal_adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
		let a = &[
			(-1, -1),
			(-1, 0),
			(-1, 1),
			(0, -1),
			(0, 1),
			(1, -1),
			(1, 0),
			(1, 1),
		];
		let x_i = x as i32;
		let y_i = y as i32;
		let size_i = Vec2::<i32>::new(self.size.x as i32, self.size.y as i32);
		let mut adjacent = Vec::new();
		for (dx, dy) in a.iter() {
			let new_x = dx + x_i;
			let new_y = dy + y_i;
			if new_x >= 0 && new_x < size_i.x && new_y >= 0 && new_y < size_i.y {
				adjacent.push((new_x as usize, new_y as usize));
			}
		}
		adjacent
	}

	fn rotate(&mut self) {
		assert_eq!(self.size.x, self.size.y);
		let size = self.size.x;
		for cycle in 0..(size / 2) {
			let a = cycle;
			let b = size - cycle - 1;
			for element in a..b {
				let c = element - a;
				let top = self.get(a, element).unwrap().clone();
				let right = self.get(element, b).unwrap().clone();
				let bottom = self.get(b, b - c).unwrap().clone();
				let left = self.get(b - c, a).unwrap().clone();

				self.set(a, element, &right);
				self.set(element, b, &bottom);
				self.set(b, b - c, &left);
				self.set(b - c, a, &top);
			}
		}
	}
	fn flip(&mut self, flip_direction: FlipDirection) {
		match flip_direction {
			FlipDirection::Horizontal => {
				for y in 0..self.size.y / 2 {
					let opposite = self.size.y - y - 1;
					for x in 0..self.size.x {
						let top = self.get(x, y).unwrap().clone();
						let bottom = self.get(x, opposite).unwrap().clone();
						self.set(x, y, &bottom);
						self.set(x, opposite, &top);
					}
				}
			}
			FlipDirection::Vertical => {
				for x in 0..self.size.x / 2 {
					let opposite = self.size.x - x - 1;
					for y in 0..self.size.y {
						let top = self.get(x, y).unwrap().clone();
						let bottom = self.get(opposite, y).unwrap().clone();
						self.set(x, y, &bottom);
						self.set(opposite, y, &top);
					}
				}
			}
		}
	}
}
impl<T> fmt::Display for Grid<T>
where
	T: Clone + fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = String::new();
		for y in 0..self.size.x {
			for x in 0..self.size.y {
				s = format!("{}{} ", s, self.get(x, y).unwrap().clone());
			}
			s = format!("{}\n", s);
		}
		write!(f, "{}", s)
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlipDirection {
	Horizontal,
	Vertical,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
	Top,
	Right,
	Bottom,
	Left,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
	R0,
	R90,
	R180,
	R270,
}
impl Iterator for Rotation {
	type Item = Rotation;
	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Rotation::R0 => Some(Rotation::R90),
			Rotation::R90 => Some(Rotation::R180),
			Rotation::R180 => Some(Rotation::R270),
			Rotation::R270 => Some(Rotation::R0),
		}
	}
}
