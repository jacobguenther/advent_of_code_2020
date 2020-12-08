// File: main.rs
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

#![feature(test)]
extern crate test;

use std::env::args;

pub mod common;
use common::ChallengeT;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

pub fn main() {
	if args().len() == 1 {
		all();
	} else {
		for arg in args() {
			match arg.as_str() {
				a if a.starts_with("target") => (),
				"all" => all(),
				"1" => day_1::Challenge::print_result(),
				"2" => day_2::Challenge::print_result(),
				"3" => day_3::Challenge::print_result(),
				"4" => day_4::Challenge::print_result(),
				"5" => day_5::Challenge::print_result(),
				"6" => day_6::Challenge::print_result(),
				"7" => day_7::Challenge::print_result(),
				"8" => day_8::Challenge::print_result(),
				"9" => day_9::Challenge::print_result(),
				_ => println!("ERROR: UNKNOWN ARGUMENT"),
			}
		}
	}
}
fn all() {
	use std::time::Instant;
	let now = Instant::now();

	day_1::Challenge::print_result();
	day_2::Challenge::print_result();
	day_3::Challenge::print_result();
	day_4::Challenge::print_result();
	day_4::Challenge::print_result();
	day_5::Challenge::print_result();
	day_6::Challenge::print_result();
	day_7::Challenge::print_result();
	day_8::Challenge::print_result();
	day_9::Challenge::print_result();

	let elapsed = now.elapsed();
	println!(
		"Estemated Time: {}ms or {}ns",
		elapsed.as_millis(),
		elapsed.as_nanos()
	);
}

#[cfg(test)]
mod tests {
	use super::all;
	use test::Bencher;

	#[bench]
	fn bench_all(b: &mut Bencher) {
		b.iter(|| all())
	}
}
