# Advent of Code 2022
[![Rust](https://github.com/nihonjinrxs/adventofcode-2022/actions/workflows/rust.yml/badge.svg)](https://github.com/nihonjinrxs/adventofcode-2022/actions/workflows/rust.yml)

This repo contains the collection of my attempts at solving the Advent of Code 2022 challenges. I'll be starting by attempting these in Rust, but may add other languages later.

Challenge writeups are available at [the Advent of Code website](https://adventofcode.com/), but I've captured [the descriptions](/docs) and [my input files](/data) in this repo too. (The input files differ per user, so don't try and use them for your own submission!)

Hope you get the chance to figure these out too!

## Running the programs

If you want to try running these, you can do so with:

```{sh}
cargo run -- <PROGRAM_NAME> <INPUT_FILE>
```

You can find the list of program names that will work in
the `match` statement [in `src\programs.rs`](/src/programs.rs#L6).
Input files are in the `data` directory, or you can create your own.

## Running the tests

You can run tests via: `cargo test`
