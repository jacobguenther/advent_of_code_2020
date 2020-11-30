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

use super::vec2::*;

pub struct Grid<T> {
	pub size: Vec2<usize>,
	pub data: Vec<Vec<T>>,
}
impl<T> Grid<T> where
	T: Default + Copy + Clone,
{
	pub fn new(x: usize, y: usize) -> Self {
		let mut data = Vec::with_capacity(y);
		for _ in 0..y {
			let mut row = Vec::with_capacity(x);
			for _ in 0..x {
				row.push(T::default())
			}
			data.push(row);
		}
		Self {
			size: Vec2::new(x, y),
			data: data
		}
	}
	pub fn set_elem(&mut self, x: usize, y: usize, elem: &T) {
		self.data[y][x] = elem.clone();
	}
	pub fn get_elem(&self, x: usize, y: usize) -> &T {
		self.data[y].get(x).unwrap()
	}
	pub fn get_mut_elem(&mut self, x: usize, y: usize) -> &T {
		self.data[y].get_mut(x).unwrap()
	}
}