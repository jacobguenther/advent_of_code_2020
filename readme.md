# Welcome
This repository is for my efforts at completing this years(2020) [Advent of Code](https://adventofcode.com/2020) challenges.

## Building
`git clone --recurse-submodules git://github.com/jacobguenther/advent_of_code_2020.git`
`cd advent_of_code_2020.git`
Make sure you have cargo and rustc installed. This project uses nightly features. So also make sure you are using the nightly branch.
`cargo build --release`

## Viewing the Answers
I recommend using the "--release" flag as day 15 can take over a minute on some hardware without it.
`cargo run --release day_<number>`
To view the answers for all the challenges so far enter.
`cargo run --release`

## Running tests
`cargo test --release day_<number>`
`cargo bench --release day_<number>`
