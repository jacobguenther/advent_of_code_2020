# Welcome
This repository is for my efforts at completing [Advent of Code](https://adventofcode.com/2020) 2020 using Rust.

## Building
`git clone git://github.com/jacobguenther/advent_of_code_2020.git`

`cd advent_of_code_2020.git`

Make sure you have cargo and rustc installed. This project uses nightly features. So also make sure you are using the nightly branch then run

`cargo build --release`

## Viewing the Answers
I recommend using the "--release" flag as day 15 can take over a minute on some hardware without it.

`cargo run --release <day_number>`

To view the answers for all the challenges so far enter.

`cargo run --release`

or

`cargo run --release threaded`


## Running tests
`cargo test --release day_<number>`

`cargo bench --release day_<number>`
