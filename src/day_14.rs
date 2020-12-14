// File: day_14.rs
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

use std::collections::HashMap;

pub struct Challenge {
	part_1_result: u64,
	part_2_result: u64,
}
impl ChallengeT for Challenge {
	type Output1 = u64;
	type Output2 = u64;

	fn day() -> u8 {
		14
	}
	fn new() -> Self {
		let mut mask_0s = 0;
		let mut mask_1s = 0;
		let part_1 = include_str!("../inputs/day_14.txt")
			.lines()
			.filter_map(|line| {
				if line.starts_with("mask") {
					mask_0s = 0;
					mask_1s = 0;
					for c in line.split("mask = ").nth(1).unwrap().chars() {
						mask_0s = mask_0s << 1;
						mask_1s = mask_1s << 1;
						match c {
							'0' => (),
							'1' => {
								mask_1s += 1;
								mask_0s += 1;
							}
							_ => mask_0s += 1,
						}
					}
					None
				} else {
					let address = line
						.split(|c| c == '[' || c == ']')
						.nth(1)
						.unwrap()
						.parse::<u64>()
						.unwrap();
					let value = line.split("= ").nth(1).unwrap().parse::<u64>().unwrap();
					let result = (value | mask_1s) & mask_0s;
					Some((address, result))
				}
			})
			.collect::<HashMap<u64, u64>>()
			.iter()
			.map(|(_, v)| *v)
			.sum();

		let mut mask = Vec::new();
		let mut memory = HashMap::<u64, u64>::new();
		let mut floating_bits_count = 0;
		include_str!("../inputs/day_14.txt")
			.lines()
			.for_each(|line| {
				if line.starts_with("mask") {
					floating_bits_count = 0;
					mask = line
						.split("mask = ")
						.nth(1)
						.unwrap()
						.as_bytes()
						.iter()
						.map(|b| match b {
							zero if *zero == '0' as u8 => 0,
							one if *one == '1' as u8 => 1,
							_ => {
								floating_bits_count += 1;
								2
							}
						})
						.collect::<Vec<u8>>();
				} else {
					let base_address = line
						.split(|c| c == '[' || c == ']')
						.nth(1)
						.unwrap()
						.parse::<u64>()
						.unwrap();

					let value = line.split("= ").nth(1).unwrap().parse::<u64>().unwrap();
					let mut addresses = Vec::with_capacity(1 << floating_bits_count);
					build_addresses(&mask, base_address, 0, &mut addresses);
					addresses.iter().for_each(|address| {
						memory.insert(*address, value);
					});
				}
			});
		let part_2 = memory.iter().map(|(_, v)| *v).sum();
		Self {
			part_1_result: part_1,
			part_2_result: part_2,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result.clone()
	}
}
fn build_addresses(mask: &[u8], address: u64, current: u64, addresses: &mut Vec<u64>) {
	if let Some(mask_value) = mask.last() {
		let address_bit = (address >> current) % 2;
		let a = match (mask_value, address_bit) {
			(1, 0) => address + (1 << current),
			(2, 0) => {
				let a = address + (1 << current);
				build_addresses(&mask[0..(mask.len() - 1)], a, current + 1, addresses);
				address
			}
			(2, 1) => {
				let a = address - (1 << current);
				build_addresses(&mask[0..(mask.len() - 1)], a, current + 1, addresses);
				address
			}
			_ => address,
		};
		build_addresses(&mask[0..(mask.len() - 1)], a, current + 1, addresses);
	} else {
		addresses.push(address);
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 15_018_100_062_885);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 5_724_245_857_696);
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
