[Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)

The input is a set of instructions:

- noop: does nothing. takes one cycle to complete.
- addx V: increases register X by V. takes two cycles to complete.
  the value is visible in X _after_ addx completes, not during.

X starts at one. Clock cycles start at 1.

To run the program and find out values of X, we can replace an addx that
takes two cycles by a noop + addx that takes one cycle. We take a Vec
that we initially fill with two dummy elements - one because cycles start at
1, not 0, so we don't have to take the offset into account. And another one because
the result of an instruction is known _after_ the cycle, not during.
Then we run the program and push the value of X onto the Vec for each cycle.
Vec[n] wil have the X value for cycle N.

Note that we could implement this as an iterator, but I think we're going
to need the same data in part2 so storing it in a Vec is better.

### Part 1.

Find signal strengths during cycles 20, 60, 100, 140, 180, 220 and add 'm up.

### Part 2.

