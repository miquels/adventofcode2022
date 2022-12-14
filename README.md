# Advent of Code 2022

I'm participating in the [adventofcode.com/2022](https://adventofcode.com/2022) challenge.

Each day is a seperate crate in a workspace. The main crate is a CLI application
that links with all other crates:

```
$ cargo run --release -- --help
Usage: aoc2022 [OPTIONS] --day <DAY>

Options:
      --day <DAY>      Day to run, or "all"  for all days
      --part <PART>    Part to run, if not set both parts are run
      --input <INPUT>  Input file to use [default: input.txt]
  -h, --help           Print help information
  -V, --version        Print version information
```

- tip from Cor: use floodfill, so instead of starting every time from the top
  and crossing empty space, we start at the bottom and go up:

