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
use std::time::Instant;

pub mod common;
use common::ChallengeT;

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_2;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;
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
				"all" => bench(&all),
				"threaded" => bench(&all_threaded),
				"1" => bench(&day_1::Challenge::print_result),
				"2" => bench(&day_2::Challenge::print_result),
				"3" => bench(&day_3::Challenge::print_result),
				"4" => bench(&day_4::Challenge::print_result),
				"5" => bench(&day_5::Challenge::print_result),
				"6" => bench(&day_6::Challenge::print_result),
				"7" => bench(&day_7::Challenge::print_result),
				"8" => bench(&day_8::Challenge::print_result),
				"9" => bench(&day_9::Challenge::print_result),
				"10" => bench(&day_10::Challenge::print_result),
				"11" => bench(&day_11::Challenge::print_result),
				"12" => bench(&day_12::Challenge::print_result),
				"13" => bench(&day_13::Challenge::print_result),
				"14" => bench(&day_14::Challenge::print_result),
				"15" => bench(&day_15::Challenge::print_result),
				"16" => bench(&day_16::Challenge::print_result),
				"17" => bench(&day_17::Challenge::print_result),
				"18" => bench(&day_18::Challenge::print_result),
				"19" => bench(&day_19::Challenge::print_result),
				"20" => bench(&day_20::Challenge::print_result),
				"21" => bench(&day_21::Challenge::print_result),
				"22" => bench(&day_22::Challenge::print_result),
				"23" => bench(&day_23::Challenge::print_result),
				"24" => bench(&day_24::Challenge::print_result),
				"25" => bench(&day_25::Challenge::print_result),
				_ => println!("ERROR: UNKNOWN ARGUMENT"),
			}
		}
	}
}
fn bench(solution: &dyn Fn()) {
	let now = Instant::now();
	solution();
	let elapsed = now.elapsed();
	println!(
		"Estemated Time: {}ms or {}ns",
		elapsed.as_millis(),
		elapsed.as_nanos()
	);
}
fn all() {
	let now = Instant::now();

	day_1::Challenge::print_result();
	day_2::Challenge::print_result();
	day_3::Challenge::print_result();
	day_4::Challenge::print_result();
	day_5::Challenge::print_result();
	day_6::Challenge::print_result();
	day_7::Challenge::print_result();
	day_8::Challenge::print_result();
	day_9::Challenge::print_result();
	day_10::Challenge::print_result();
	day_11::Challenge::print_result();
	day_12::Challenge::print_result();
	day_13::Challenge::print_result();
	day_14::Challenge::print_result();
	day_15::Challenge::print_result();
	day_16::Challenge::print_result();
	day_17::Challenge::print_result();
	day_18::Challenge::print_result();
	day_19::Challenge::print_result();
	day_20::Challenge::print_result();
	day_21::Challenge::print_result();
	day_22::Challenge::print_result();
	day_23::Challenge::print_result();
	day_24::Challenge::print_result();
	day_25::Challenge::print_result();

	let elapsed = now.elapsed();
	println!(
		"Estemated Time: {}ms or {}us",
		elapsed.as_millis(),
		elapsed.as_micros()
	);
}
fn all_threaded() {
	let result_fns = [
		day_1::Challenge::result_string,
		day_2::Challenge::result_string,
		day_3::Challenge::result_string,
		day_4::Challenge::result_string,
		day_5::Challenge::result_string,
		day_6::Challenge::result_string,
		day_7::Challenge::result_string,
		day_8::Challenge::result_string,
		day_9::Challenge::result_string,
		day_10::Challenge::result_string,
		day_11::Challenge::result_string,
		day_12::Challenge::result_string,
		day_13::Challenge::result_string,
		day_14::Challenge::result_string,
		day_15::Challenge::result_string,
		day_16::Challenge::result_string,
		day_17::Challenge::result_string,
		day_18::Challenge::result_string,
		day_19::Challenge::result_string,
		day_20::Challenge::result_string,
		day_21::Challenge::result_string,
		day_22::Challenge::result_string,
		day_23::Challenge::result_string,
		day_24::Challenge::result_string,
		day_25::Challenge::result_string,
	];
	let do_part = |result_fns: &[fn() -> String]| {
		let now = Instant::now();

		let mut res = String::new();
		for f in result_fns.iter() {
			res.push_str(&f());
			res.push('\n');
		}
		let elapsed = now.elapsed();
		(
			format!(
				"Estemated Time: {}ms or {}us",
				elapsed.as_millis(),
				elapsed.as_micros()
			),
			res,
		)
	};
	let a_handle = std::thread::spawn(move || do_part(&result_fns[0..15]));
	let b_handle = std::thread::spawn(move || do_part(&result_fns[15..21]));
	let c_handle = std::thread::spawn(move || do_part(&result_fns[21..22]));
	let d_handle = std::thread::spawn(move || do_part(&result_fns[22..25]));
	let (time_a, res_a) = a_handle.join().unwrap();
	let (time_b, res_b) = b_handle.join().unwrap();
	let (time_c, res_c) = c_handle.join().unwrap();
	let (time_d, res_d) = d_handle.join().unwrap();
	println!("{}{}{}{}", res_a, res_b, res_c, res_d);
	if true {
		println!(
			"th1: {}\nth2: {}\nth3: {}\nth4: {}",
			time_a, time_b, time_c, time_d
		);
	}
}
