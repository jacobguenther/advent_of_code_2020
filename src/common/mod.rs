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

pub mod vec2;
pub mod grid;

use std::fmt;

pub trait ChallengeT {
	type Output1: fmt::Display;
	type Output2: fmt::Display;

	fn print_result() {
		println!("{}", Self::result_string());
	}
	fn result_string() -> String {
		format!("Day {}\n  part 1: {}\n  part 2: {}", Self::day(), Self::part_1(), Self::part_2())
	}

	fn day() -> u8;
	fn part_1() -> Self::Output1;
	fn part_2() -> Self::Output2;
}

pub fn get_lines_from_content(input: &str) -> Vec<String> {
	input.to_owned()
		.lines()
		.map(|line| line.to_owned())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_lines_from_file_test() {
		let content = include_str!("../../inputs/test.txt");
		let lines = get_lines_from_content(content);
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[0], "Hello World!");
		assert_eq!(lines[1], "We come in peace");
	}
}