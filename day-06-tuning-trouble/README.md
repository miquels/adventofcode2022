## [Day 6: Tuning Trouble](https://adventofcode.com/2022/day/6)

### Part 1.

The puzzle is to find, in a string of characters, the first sequence of 4
characters that are all different.

This is very easy to do in rust with the `slice::windows(4)` method,
and a simple functions that checks if all 4 characters are different.
I do wonder if there is a bitbang version of "the next 4 bytes are
all different" instead of 9 compares.

```
 == start ==
part1: 1100
took 12.369µs
```

### Part 2.

The exact same thing but for length 14.

```
day-06: tuning-trouble
part1: == start ==
part1: 1100
part1: took 10.68µs
part2: == start ==
part2: 2421
part2: took 27.332µs
```

### Optimizations.

TBD.
