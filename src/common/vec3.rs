// File: common/vec3.rs
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

use super::NeighborsT;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}
impl<T> Vec3<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
}
impl NeighborsT for Vec3<i16> {
	fn neighbors(&self) -> Vec<Vec3<i16>> {
		let mut neighbors = Vec::with_capacity(26);
		for z in self.z - 1..self.z + 2 {
			for x in self.x - 1..self.x + 2 {
				for y in self.y - 1..self.y + 2 {
					let n = Vec3::new(x, y, z);
					if n != *self {
						neighbors.push(n);
					}
				}
			}
		}
		neighbors
	}
}
