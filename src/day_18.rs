// File: day_18.rs
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

pub struct Challenge {
	part_1_result: u64,
	part_2_result: u64,
}
impl ChallengeT for Challenge {
	type Output1 = u64;
	type Output2 = u64;

	fn day() -> u8 {
		18
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_18.txt");
		let [part_1_result, part_2_result] = input
			.lines()
			.map(|line| {
				let mut parser = Parser::new(line.as_bytes());
				[parser.parse_1().eval(), parser.parse_2().eval()]
			})
			.fold([0, 0], |[sum_1, sum_2], [partial_1, partial_2]| {
				[sum_1 + partial_1, sum_2 + partial_2]
			});

		Self {
			part_1_result: part_1_result,
			part_2_result: part_2_result,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}

struct Parser {
	lexemes: &'static [u8],
	current_i: usize,
}
impl Parser {
	pub fn new(lexemes: &'static [u8]) -> Self {
		Self {
			lexemes: lexemes,
			current_i: 0,
		}
	}
	fn parse_1(&mut self) -> Expression1 {
		let ast = self.parse_expression_1().unwrap();
		self.current_i = 0;
		ast
	}
	fn parse_2(&mut self) -> Expression2 {
		let ast = self.parse_expression_2().unwrap();
		self.current_i = 0;
		ast
	}
	fn parse_expression_1(&mut self) -> Option<Expression1> {
		let mut expression =
			Expression1::Atom(self.parse_atom::<Expression1>(&Self::parse_expression_1)?);
		loop {
			let is_add = match self.current_lexeme() {
				Some(op) => match op {
					p if *p == '+' as u8 => true,
					m if *m == '*' as u8 => false,
					_ => break,
				},
				None => break,
			};
			self.step_lexemes();
			let next_atom = self
				.parse_atom::<Expression1>(&Self::parse_expression_1)
				.unwrap();
			expression = match is_add {
				true => Expression1::Add(Box::new(expression), next_atom),
				false => Expression1::Mul(Box::new(expression), next_atom),
			};
		}
		Some(expression)
	}
	fn parse_expression_2(&mut self) -> Option<Expression2> {
		let mut expression = Expression2::Factor(self.parse_factor_2()?);
		loop {
			match self.current_lexeme() {
				Some(op) => match op {
					m if *m == '*' as u8 => false,
					_ => break,
				},
				None => break,
			};
			self.step_lexemes();
			expression = Expression2::Mul(Box::new(expression), self.parse_factor_2().unwrap());
		}
		Some(expression)
	}
	fn parse_factor_2(&mut self) -> Option<Factor> {
		let mut factor = Factor::Atom(self.parse_atom::<Expression2>(&Self::parse_expression_2)?);
		loop {
			match self.current_lexeme() {
				Some(op) => match op {
					a if *a == '+' as u8 => false,
					_ => break,
				},
				None => break,
			};
			self.step_lexemes();
			factor = Factor::Add(
				Box::new(factor),
				self.parse_atom::<Expression2>(&Self::parse_expression_2)
					.unwrap(),
			);
		}
		Some(factor)
	}
	fn parse_atom<T>(
		&mut self,
		parse_expression: &dyn Fn(&mut Self) -> Option<T>,
	) -> Option<Atom<T>> {
		match self.current_lexeme()? {
			p if *p == '(' as u8 => {
				self.step_lexemes();
				let expresssion = parse_expression(self)?;
				self.step_lexemes();
				Some(Atom::Paren(Box::new(expresssion)))
			}
			n => {
				let num = (n - 48) as u64;
				self.step_lexemes();
				Some(Atom::Number(num))
			}
		}
	}

	fn current_lexeme(&mut self) -> Option<&u8> {
		let l = self.lexemes.get(self.current_i)?;
		if *l == ' ' as u8 {
			self.step_lexemes();
			self.current_lexeme()
		} else {
			Some(l)
		}
	}
	fn step_lexemes(&mut self) {
		self.current_i += 1;
	}
}

trait EvalT {
	fn eval(&self) -> u64;
}
#[derive(Debug, Clone)]
enum Expression1 {
	Add(Box<Expression1>, Atom<Expression1>),
	Mul(Box<Expression1>, Atom<Expression1>),
	Atom(Atom<Expression1>),
}
impl EvalT for Expression1 {
	fn eval(&self) -> u64 {
		match self {
			Expression1::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
			Expression1::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
			Expression1::Atom(atom) => atom.eval(),
		}
	}
}
#[derive(Debug, Clone)]
enum Expression2 {
	Mul(Box<Expression2>, Factor),
	Factor(Factor),
}
impl EvalT for Expression2 {
	fn eval(&self) -> u64 {
		match self {
			Expression2::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
			Expression2::Factor(factor) => factor.eval(),
		}
	}
}
#[derive(Debug, Clone)]
enum Factor {
	Add(Box<Factor>, Atom<Expression2>),
	Atom(Atom<Expression2>),
}
impl EvalT for Factor {
	fn eval(&self) -> u64 {
		match self {
			Factor::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
			Factor::Atom(atom) => atom.eval(),
		}
	}
}
#[derive(Debug, Clone)]
enum Atom<T> {
	Number(u64),
	Paren(Box<T>),
}
impl<T> EvalT for Atom<T>
where
	T: EvalT,
{
	fn eval(&self) -> u64 {
		match self {
			Atom::Paren(expression) => expression.eval(),
			Atom::Number(n) => *n,
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
		assert_eq!(Challenge::new().part_1(), 14006719520523);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(Challenge::new().part_2(), 545115449981968);
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
