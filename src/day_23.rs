// File: day_23.rs
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

use super::common::*;
use std::collections::VecDeque;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		23
	}
	fn new() -> Self {
		let input = [3, 6, 8, 1, 9, 5, 7, 4, 2];

		let mut cups = VecDeque::with_capacity(10);
		for label in input.iter() {
			cups.push_back(*label);
		}

		let max = 9;
		let moves = 100;
		(0..moves).for_each(|_| do_move(&mut cups, max));

		while cups[0] != 1 {
			cups.rotate_right(1);
		}
		cups.pop_front();
		let mut part_1_result = 0;
		for &cup in cups.iter() {
			part_1_result *= 10;
			part_1_result += cup as usize;
		}

		// index is cup, value is next cup
		// ignore index 0
		let mut cups = (1..(1_000_002)).collect::<Vec<u32>>();
		for (&current, &next) in input.iter().zip(input[1..].iter()) {
			cups[current as usize] = next;
		}
		cups[*input.last().unwrap() as usize] = 10;
		cups[1_000_000] = input[0];

		let max = 1_000_000;
		let moves = 10_000_000;
		let mut current = *input.first().unwrap();
		(0..moves).for_each(|_| {
			current = do_move_2(&mut cups, current as usize, max);
		});

		let second = cups[1];
		let third = cups[second as usize];
		let part_2_result = second as usize * third as usize;
		Self {
			part_1_result,
			part_2_result,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result.clone()
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}

fn do_move(cups: &mut VecDeque<u32>, max: u32) {
	let current_cup = cups[0];
	cups.rotate_left(1);
	let picked_up_1 = cups.pop_front().unwrap();
	let picked_up_2 = cups.pop_front().unwrap();
	let picked_up_3 = cups.pop_front().unwrap();

	let mut destination_cup = current_cup - 1;
	let mut valid_destination = false;
	while !valid_destination {
		if destination_cup > 0 {
			if destination_cup != picked_up_1
				&& destination_cup != picked_up_2
				&& destination_cup != picked_up_3
			{
				valid_destination = true;
			} else {
				destination_cup -= 1;
			}
		} else {
			destination_cup = max;
		}
	}

	let destination_index = cups.iter().position(|cup| *cup == destination_cup).unwrap();
	cups.insert(destination_index + 1, picked_up_3);
	cups.insert(destination_index + 1, picked_up_2);
	cups.insert(destination_index + 1, picked_up_1);
}

fn do_move_2(cups: &mut Vec<u32>, current: usize, max: usize) -> u32 {
	let picked_up_1 = cups[current];
	let picked_up_2 = cups[picked_up_1 as usize];
	let picked_up_3 = cups[picked_up_2 as usize];

	let mut destination_cup = current - 1;
	loop {
		if destination_cup > 0 {
			if destination_cup != picked_up_1 as usize
				&& destination_cup != picked_up_2 as usize
				&& destination_cup != picked_up_3 as usize
			{
				break;
			} else {
				destination_cup -= 1;
			}
		} else {
			destination_cup = max;
		}
	}

	cups[current] = cups[picked_up_3 as usize];
	cups[picked_up_3 as usize] = cups[destination_cup];
	cups[destination_cup] = picked_up_1;
	cups[current]
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 95648732);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 192515314252);
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
