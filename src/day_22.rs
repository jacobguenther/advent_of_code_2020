// File: day_22.rs
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
use std::collections::HashSet;
use std::collections::VecDeque;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		22
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_22.txt");
		let (mut deck_1, mut deck_2) = parse_input(input);

		let mut player_1_deck = deck_1.clone();
		let mut player_2_deck = deck_2.clone();
		combat(&mut player_1_deck, &mut player_2_deck);
		let part_1_result = calculate_winners_score(&player_1_deck, &player_2_deck);

		recursive_combat(&mut deck_1, &mut deck_2);
		let part_2_result = calculate_winners_score(&deck_1, &deck_2);

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

fn parse_input(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
	let parse_deck = |parts: &mut std::str::Split<&str>| -> VecDeque<u8> {
		parts
			.next()
			.unwrap()
			.lines()
			.filter_map(|line| line.parse::<u8>().ok())
			.collect()
	};
	let mut parts = input.split("\n\n");
	let deck_1 = parse_deck(&mut parts);
	let deck_2 = parse_deck(&mut parts);
	(deck_1, deck_2)
}
fn combat(player_1_deck: &mut VecDeque<u8>, player_2_deck: &mut VecDeque<u8>) {
	while !player_1_deck.is_empty() && !player_2_deck.is_empty() {
		if player_1_deck[0] > player_2_deck[0] {
			player_1_deck.rotate_left(1);
			player_1_deck.push_back(player_2_deck.pop_front().unwrap());
		} else {
			player_2_deck.rotate_left(1);
			player_2_deck.push_back(player_1_deck.pop_front().unwrap());
		}
	}
}
fn recursive_combat(player_1_deck: &mut VecDeque<u8>, player_2_deck: &mut VecDeque<u8>) -> bool {
	let mut previous_rounds = HashSet::new();
	while !player_1_deck.is_empty() && !player_2_deck.is_empty() {
		let mut hasher = DefaultHasher::new();
		player_1_deck.hash(&mut hasher);
		player_2_deck.hash(&mut hasher);
		let hash = hasher.finish();
		if previous_rounds.contains(&hash) {
			return true;
		}
		previous_rounds.insert(hash);

		let player_1_card = player_1_deck.pop_front().unwrap();
		let player_2_card = player_2_deck.pop_front().unwrap();

		let player_1_wins = if player_1_card as usize <= player_1_deck.len()
			&& player_2_card as usize <= player_2_deck.len()
		{
			let mut new_player_1_deck = player_1_deck
				.iter()
				.take(player_1_card as usize)
				.copied()
				.collect::<VecDeque<_>>();
			let mut new_player_2_deck = player_2_deck
				.iter()
				.take(player_2_card as usize)
				.copied()
				.collect::<VecDeque<_>>();
			recursive_combat(&mut new_player_1_deck, &mut new_player_2_deck)
		} else {
			player_1_card > player_2_card
		};

		if player_1_wins {
			player_1_deck.push_back(player_1_card);
			player_1_deck.push_back(player_2_card);
		} else {
			player_2_deck.push_back(player_2_card);
			player_2_deck.push_back(player_1_card);
		}
	}
	!player_1_deck.is_empty()
}
fn calculate_winners_score(player_1_deck: &VecDeque<u8>, player_2_deck: &VecDeque<u8>) -> usize {
	let winning_deck = if !player_1_deck.is_empty() {
		player_1_deck
	} else {
		player_2_deck
	};
	winning_deck
		.iter()
		.rev()
		.enumerate()
		.fold(0, |sum, (i, &card)| sum + (i + 1) * card as usize)
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 32083);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 35495);
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
