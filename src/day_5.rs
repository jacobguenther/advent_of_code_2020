// File: day_5.rs
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

use super::common::{
	ChallengeT,
};

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		5
	}
	fn new() -> Self {
		let mut highest = 0;
		let mut filled_seat_ids: Vec<usize> = vec![0; 128*8];
		include_str!("../inputs/day_5.txt").lines()
			.for_each(|line| {
				let row = get_row(line);
				let col = get_seat(line);
				let seat_id = row * 8 + col;
				filled_seat_ids[seat_id] = seat_id;
				if seat_id > highest {
					highest = seat_id;
				}
			});

		let mut my_id = 0;
		&filled_seat_ids[0..].iter()
			.zip(&filled_seat_ids[1..])
			.zip(&filled_seat_ids[2..])
			.find(|((previouse, current), next)|
				if **current == 0 && **previouse + 2 == **next {
					my_id = **previouse + 1;
					true
				} else {
					false
				});

		Self {
			part_1_result: highest,
			part_2_result: my_id,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}

fn get_row(line: &str) -> usize {
	let mut lower_row = 0;
	let mut upper_row = 127;
	let mut row = 0;
	for (i, c) in (&line[0..7]).chars().enumerate() {
		if i == 6 && c == 'B' {
			row = upper_row;
		} else if i == 6 && c == 'F' {
			row = lower_row;
		}
		match c {
			'B' => lower_row += split(upper_row, lower_row),
			'F' => upper_row -= split(upper_row, lower_row),
			_ => (),
		}
	}
	row
}
fn get_seat(line: &str) -> usize {
	let mut lower_seat = 0;
	let mut upper_seat = 7;
	let mut col = 0;
	for (i, c) in (&line[7..]).chars().enumerate() {
		if i == 2 && c == 'L' {
			col = lower_seat;
		} else if i == 2 && c == 'R' {
			col = upper_seat;
		}
		match c {
			'R' => lower_seat += split(upper_seat, lower_seat),
			'L' => upper_seat -= split(upper_seat, lower_seat),
			_ => (),
		}
	}
	col
}
fn split(upper: usize, lower: usize) -> usize {
	((upper - lower) as f32 / 2.0).ceil() as usize
}


#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::{
		ChallengeT,
	};
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 235);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 194);
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
