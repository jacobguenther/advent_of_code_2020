// File: day_1.rs
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

pub struct Challenge {
	parsed_lines: Vec<(usize, usize, char, &'static str)>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		2
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_2.txt");
		Self {
			parsed_lines: input.lines().map(|line| parse_line(line)).collect(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.parsed_lines
			.iter()
			.filter(|(min, max, letter, password)| {
				let count = password
					.chars()
					.fold(0, |acc, c| if c == *letter { acc + 1 } else { acc });
				count >= *min && count <= *max
			})
			.count()
	}
	fn part_2(&self) -> Self::Output2 {
		self.parsed_lines
			.iter()
			.filter(|(first_pos, second_pos, letter, password)| {
				let (first_i, second_i) = (first_pos - 1, second_pos - 1);
				let [first_letter, second_letter] = password[0..*second_pos]
					.chars()
					.enumerate()
					.fold([' ', ' '], |[first_letter, second_letter], (i, c)| {
						if i == first_i {
							[c, second_letter]
						} else if i == second_i {
							[first_letter, c]
						} else {
							[first_letter, second_letter]
						}
					});
				first_letter != second_letter
					&& (*letter == first_letter || *letter == second_letter)
			})
			.count()
	}
}
fn parse_line(line: &str) -> (usize, usize, char, &str) {
	let mut min: usize = 0;
	let mut max: usize = 0;
	let mut letter = ' ';
	let mut password = "";
	line.split(&['-', ' '][..])
		.enumerate()
		.for_each(|(i, s)| match i {
			0 => min = s.parse().unwrap(),
			1 => max = s.parse().unwrap(),
			2 => letter = s.chars().next().unwrap(),
			3 => password = s,
			_ => (),
		});
	(min, max, letter, password)
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1() {
		assert_eq!(Challenge::new().part_1(), 517);
	}
	#[test]
	fn part_2() {
		assert_eq!(Challenge::new().part_2(), 284);
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
