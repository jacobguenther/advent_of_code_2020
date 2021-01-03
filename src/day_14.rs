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
		let part_1_result = include_str!("../inputs/day_14.txt")
			.lines()
			.filter_map(|line| {
				if line.starts_with("ma") {
					mask_0s = 0;
					mask_1s = 0;
					for c in line.split("mask = ").nth(1).unwrap().chars() {
						mask_0s <<= 1;
						mask_1s <<= 1;
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

		let mut mask_1s = 0;
		let mut mask_floating = 0;
		let mut floating_bits_count = 0;
		let mut memory = HashMap::<u64, u64>::new();
		// Note: '0' is 48 and '1' is 49
		include_str!("../inputs/day_14.txt")
			.lines()
			.for_each(|line| {
				if line.starts_with("ma") {
					mask_1s = 0;
					mask_floating = 0;
					floating_bits_count = 0;
					line.split("mask = ")
						.nth(1)
						.unwrap()
						.as_bytes()
						.iter()
						.for_each(|b| {
							mask_1s <<= 1;
							mask_floating <<= 1;
							match b {
								48 => (),
								49 => mask_1s += 1,
								_ => {
									mask_floating += 1;
									floating_bits_count += 1;
								}
							}
						});
				} else {
					let base_address = line
						.split(|c| c == '[' || c == ']')
						.nth(1)
						.unwrap()
						.parse::<u64>()
						.unwrap() | mask_1s;

					let value = line.split("= ").nth(1).unwrap().parse::<u64>().unwrap();
					let mut addresses = Vec::with_capacity(1 << floating_bits_count);
					build_addresses(mask_floating, base_address, 0, &mut addresses);
					addresses.iter().for_each(|address| {
						memory.insert(*address, value);
					});
				}
			});
		let part_2_result = memory.iter().map(|(_, v)| *v).sum();
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
fn build_addresses(mask: u64, address: u64, current: u64, addresses: &mut Vec<u64>) {
	if current == 64 {
		addresses.push(address);
		return;
	}
	let mask_bit = (mask >> current) & 1;
	let address_bit = (address >> current) & 1;
	match (mask_bit, address_bit) {
		(1, 0) => {
			let a = address + (1 << current);
			build_addresses(mask, a, current + 1, addresses);
		}
		(1, 1) => {
			let a = address - (1 << current);
			build_addresses(mask, a, current + 1, addresses);
		}
		_ => (),
	};
	build_addresses(mask, address, current + 1, addresses)
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
