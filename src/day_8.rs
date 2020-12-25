// File: day_8.rs
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
	part_1_answer: i32,
	part_2_answer: i32,
}
impl ChallengeT for Challenge {
	type Output1 = i32;
	type Output2 = i32;

	fn day() -> u8 {
		8
	}
	fn new() -> Self {
		let instructions = include_str!("../inputs/day_8.txt")
			.lines()
			.map(|line| {
				let instruction_name = match &line[..3] {
					"acc" => InstructionType::Acc,
					"jmp" => InstructionType::Jmp,
					_ => InstructionType::Nop,
				};
				let num = line[4..].parse().unwrap();
				(instruction_name, num)
			})
			.collect::<Vec<(InstructionType, i32)>>();

		let broken_acc = run_instructions(&instructions);

		let mut final_acc = 0;
		let mut modified_instructions = instructions.clone();
		for (i, (inst, number)) in instructions.iter().enumerate() {
			match inst {
				InstructionType::Nop => modified_instructions[i] = (InstructionType::Jmp, *number),
				InstructionType::Jmp => modified_instructions[i] = (InstructionType::Nop, *number),
				_ => continue,
			};
			if let Ok(result) = run_instructions(&modified_instructions) {
				final_acc = result;
				break;
			}
			modified_instructions[i] = (*inst, *number);
		}
		Self {
			part_1_answer: broken_acc.unwrap_err(),
			part_2_answer: final_acc,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_answer
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_answer
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum InstructionType {
	Acc,
	Jmp,
	Nop,
}

fn run_instructions(instructions: &[(InstructionType, i32)]) -> Result<i32, i32> {
	let mut already_ran = vec![false; instructions.len()];
	let mut acc = 0;
	let mut index: i32 = 0;
	loop {
		if index as usize >= (instructions.len() - 1) {
			return Ok(acc);
		}

		let inst_already_ran = already_ran[index as usize];
		if inst_already_ran {
			return Err(acc);
		} else {
			already_ran[index as usize] = true;
		}

		let (inst_name, inst_value) = instructions[index as usize];
		match inst_name {
			InstructionType::Acc => {
				index += 1;
				acc += inst_value;
			}
			InstructionType::Jmp => index += inst_value,
			InstructionType::Nop => index += 1,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 1675);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 1532);
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
