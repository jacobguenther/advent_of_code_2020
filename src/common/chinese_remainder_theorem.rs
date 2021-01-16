// File: lib.rs
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

// https://en.wikipedia.org/wiki/Chinese_remainder_theorem
pub fn chinese_remainder_theorem(residues: &[u128], modulii: &[u128]) -> (u128, u128) {
	let product = modulii.iter().product();
	let x = residues
		.iter()
		.zip(modulii)
		.fold(0, |acc, (residue, modulus)| {
			let m: u128 = product / modulus;
			let inverse = multiplicative_inverse(m as i128, *modulus as i128).unwrap() as u128;
			acc + *residue * m * inverse
		});
	(x % product, product)
}

// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
pub fn multiplicative_inverse(x: i128, n: i128) -> Option<i128> {
	let (g, x, _) = egcd(x, n)?;
	match g {
		1 => Some((x % n + n) % n),
		_ => None,
	}
}

// extended euclidean algorithm
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn egcd(a: i128, b: i128) -> Option<(i128, i128, i128)> {
	match (a, b) {
		(0, 0) => None,
		(0, _) => Some((b, 0, 1)),
		(_, 0) => Some((a, 1, 0)),
		_ => {
			let (g, x, y) = egcd(b % a, a).unwrap();
			Some((g, y - (b / a) * x, x))
		}
	}
}

pub struct BitIterator {
	n: u64,
	count: usize,
}
impl BitIterator {
	pub fn new(n: u64) -> Self {
		Self { n, count: 64 }
	}
}
impl Iterator for BitIterator {
	type Item = u64;
	fn next(&mut self) -> Option<Self::Item> {
		if self.count == 0 {
			None
		} else {
			self.count -= 1;
			let res = Some(self.n % 2);
			self.n >>= 1;
			res
		}
	}
}
