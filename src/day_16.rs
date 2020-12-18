// File: day_16.rs
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

use super::common::{vec2::Vec2, *};

pub struct Challenge {
	part_1_result: usize,
	notes: Notes,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		16
	}
	fn new() -> Self {
		let mut split_input = include_str!("../inputs/day_16.txt").split("\n\n");

		let fields = split_input
			.next()
			.unwrap()
			.lines()
			.map(|line| {
				let mut parts = line.split(": ");
				let name = parts.next().unwrap();
				let mut ranges = parts.next().unwrap().split(" or ");
				let mut nums_1 = ranges.next().unwrap().split('-');
				let mut nums_2 = ranges.next().unwrap().split('-');
				(
					name,
					Vec2::new(
						nums_1.next().unwrap().parse::<usize>().unwrap(),
						nums_1.next().unwrap().parse::<usize>().unwrap(),
					),
					Vec2::new(
						nums_2.next().unwrap().parse::<usize>().unwrap(),
						nums_2.next().unwrap().parse::<usize>().unwrap(),
					),
				)
			})
			.collect::<Vec<(&str, Vec2<usize>, Vec2<usize>)>>();

		let my_ticket = split_input
			.next()
			.unwrap()
			.split(":\n")
			.nth(1)
			.unwrap()
			.split(',')
			.map(|s| s.parse::<usize>().unwrap())
			.collect::<Vec<_>>();

		let mut error_rate = 0;
		let filtered_tickets = split_input
			.next()
			.unwrap()
			.split(":\n")
			.nth(1)
			.unwrap()
			.split_whitespace()
			.filter_map(|line| {
				let ticket = line
					.split(',')
					.map(|s| s.parse::<usize>().unwrap())
					.collect::<Vec<_>>();
				for value in ticket.iter() {
					let mut in_range = false;
					for (_, range1, range2) in fields.iter() {
						if bound_by(*value, range1, range2) {
							in_range = true;
							break;
						}
					}
					if !in_range {
						error_rate += value;
						return None;
					}
				}
				Some(ticket)
			})
			.collect::<Vec<_>>();

		Self {
			part_1_result: error_rate,
			notes: Notes {
				fields,
				my_ticket,
				filtered_tickets,
			},
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		let height = self.notes.filtered_tickets.len();
		let width = self.notes.filtered_tickets[0].len();

		let mut columns = vec![vec![0; height]; width];
		for (y, col) in columns.iter_mut().enumerate() {
			for (x, item) in col.iter_mut().enumerate() {
				*item = self.notes.filtered_tickets[x][y];
			}
		}

		let matches = columns
			.iter()
			.map(|column| {
				self.notes
					.fields
					.iter()
					.filter_map(|(field_name, range1, range2)| {
						for value in column.iter() {
							if !bound_by(*value, range1, range2) {
								return None;
							}
						}
						Some(*field_name)
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();

		let col_names = find_order(&matches, 0, &Vec::new()).unwrap();
		col_names.iter().zip(self.notes.my_ticket.iter()).fold(
			1,
			|product, (col_name, ticket_val)| {
				if col_name.starts_with("departure") {
					product * *ticket_val
				} else {
					product
				}
			},
		)
	}
}

type FieldName = &'static str;
type Field = (FieldName, Vec2<usize>, Vec2<usize>);
type Ticket = Vec<usize>;
struct Notes {
	fields: Vec<Field>,
	my_ticket: Ticket,
	filtered_tickets: Vec<Ticket>,
}
fn bound_by(value: usize, range1: &Vec2<usize>, range2: &Vec2<usize>) -> bool {
	(value >= range1.x && value <= range1.y) || (value >= range2.x && value <= range2.y)
}
fn find_order(
	matches: &[Vec<FieldName>],
	current: usize,
	partial: &[FieldName],
) -> Option<Vec<FieldName>> {
	if current == matches.len() {
		return Some(partial.to_owned());
	}
	for option in matches[current].iter() {
		if partial.contains(option) {
			continue;
		}
		let mut param = partial.to_owned();
		param.push(option);
		if let Some(result) = find_order(matches, current + 1, &param) {
			return Some(result);
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 26941);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 634796407951);
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
