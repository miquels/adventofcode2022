# [Day 11: Monkey in the Middle](https://adventofcode.com/2022/day/11)

The puzzle is about monkeys [throwing items around](https://en.wikipedia.org/wiki/Keep_away),
the puzzle input is a list of monkeys, where for every monkey we have a list of
items currently in its possession, and its behaviour. For example:

```
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
```

Monkeys take _turns_ in order of their number (0, 1, ..). A _round_ is when each monkey
has taken a _turn_.

### Part 1.

Run 20 rounds. After 20 rounds, find the two most active monkeys (active ==
how many items have they thrown).

### Part 2.

