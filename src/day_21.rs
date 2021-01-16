// File: day_21.rs
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
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: String,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = String;

	fn day() -> u8 {
		21
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_21.txt");
		let products = parse_input(input);

		let mut possible_ingredients_for_allergens = HashMap::<&str, HashSet<&str>>::new();
		for product in products.iter() {
			for &allergen in product.allergens.iter() {
				let ingredients = match possible_ingredients_for_allergens.get(allergen) {
					Some(already_added) => product
						.ingredients
						.intersection(already_added)
						.cloned()
						.collect(),
					None => product.ingredients.clone(),
				};
				possible_ingredients_for_allergens.insert(allergen, ingredients);
			}
		}

		let unsafe_ingredients: HashSet<&str> = possible_ingredients_for_allergens
			.values()
			.flatten()
			.cloned()
			.collect();

		let part_1_result = products
			.iter()
			.map(|product| {
				product
					.ingredients
					.iter()
					.filter(|&&ingredient| !unsafe_ingredients.contains(ingredient))
					.count()
			})
			.sum::<usize>();

		while possible_ingredients_for_allergens
			.values()
			.any(|set| set.len() > 1)
		{
			let allergen = possible_ingredients_for_allergens
				.values()
				.find(|set| set.len() == 1)
				.unwrap()
				.to_owned();
			possible_ingredients_for_allergens = possible_ingredients_for_allergens
				.into_iter()
				.map(|(i, s)| match s.len() {
					1 => (i, s),
					_ => (i, s.difference(&allergen).copied().collect()),
				})
				.collect();
		}

		let mut ingredient_allergen_pairs = possible_ingredients_for_allergens
			.iter()
			.map(|(allergen, ingredients)| (*allergen, *ingredients.iter().next().unwrap()))
			.collect::<Vec<(&str, &str)>>();
		ingredient_allergen_pairs.sort_unstable();

		let part_2_result = ingredient_allergen_pairs.iter().fold(
			String::new(),
			|mut canonical, (_, ingredient)| {
				canonical = format!("{},{}", canonical, ingredient);
				canonical
			},
		)[1..]
			.to_owned();

		Self {
			part_1_result,
			part_2_result,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result.clone()
	}
}

fn parse_input(input: &str) -> Vec<Product> {
	input
		.lines()
		.map(|line| {
			let mut parts = line[..(line.len() - 1)].split(" (contains ");
			let ingredients = parts.next().unwrap().split_ascii_whitespace().collect();
			let allergens = parts.next().unwrap().split(", ").collect();
			Product {
				ingredients,
				allergens,
			}
		})
		.collect()
}

struct Product<'a> {
	ingredients: HashSet<&'a str>,
	allergens: HashSet<&'a str>,
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::ChallengeT;
	use test::Bencher;

	#[test]
	fn part_1_test() {
		assert_eq!(Challenge::new().part_1(), 2280);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(
			&Challenge::new().part_2(),
			"vfvvnm,bvgm,rdksxt,xknb,hxntcz,bktzrz,srzqtccv,gbtmdb"
		);
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
