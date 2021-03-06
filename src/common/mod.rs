// File: common/mod.rs
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

pub mod chinese_remainder_theorem;
pub mod grid;
pub mod vec2;
pub mod vec3;
pub mod vec4;

use std::fmt;

pub trait ChallengeT
where
	Self: Sized,
{
	type Output1: fmt::Display;
	type Output2: fmt::Display;

	fn print_result() {
		println!("{}", Self::result_string());
	}
	fn result_string() -> String {
		let challenge = Self::new();
		format!(
			"Day {}\n  part 1: {}\n  part 2: {}",
			Self::day(),
			challenge.part_1(),
			challenge.part_2()
		)
	}

	fn day() -> u8;

	fn new() -> Self;
	fn part_1(&self) -> Self::Output1;
	fn part_2(&self) -> Self::Output2;
}

pub trait NeighborsT
where
	Self: Sized,
{
	fn neighbors(&self) -> Vec<Self>;
}

// greatest common divisor
// https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd(a: usize, b: usize) -> Option<usize> {
	match (a, b) {
		(0, 0) => None,
		(0, _) => Some(b),
		(_, 0) => Some(a),
		_ => gcd(b, a % b),
	}
}

// least common multiple
// https://en.wikipedia.org/wiki/Least_common_multiple
pub fn lcm(a: usize, b: usize) -> Option<usize> {
	Some(a * b / gcd(a, b)?)
}
