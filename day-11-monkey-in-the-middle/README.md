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

Same thing, slightly different algorithm, 10000 rounds.

This got problematic. The numbers got way too big. At first I tried to use the
`num-bigint` crate, but I quickly found out that that was going to take ..
days? years? anyway, a lot of time.

So the test for each item that decides where the monkey is going to
throw the next item is "divisible by N". It occured to me that we could
simply limit the worry level of each item by taking mod(product of all N's).
And in fact, that is the right solution. Which I would have known right
away if I hadn't entered "number of rounds for part 2" as 1000 instead of 10000...

### Optimization.

I don't think there is much to optimize here. The modulo and divide operations
are simply very expensive on  a modern computer. Runtime is:

```
part1: 56120
part2: 24389045529
took 7.251863ms
```

Even simple things like this already help:

```
-   item /= relief;
-   item %= modulo;
+   if relief != 1 {
+       item /= relief;
+   }
+   if item > modulo {
+       item %= modulo;
+   }
```

See:

```
part1: 56120
part2: 24389045529
took 5.210714ms
```

Making `relief` a constant and getting rid of `if relief != `:

```
part1: 56120
part2: 24389045529
took 5.069096ms
```

Not very convincing.

It occurred to me that I can merge `Val` and `Op` into one `Op` enum. This shaves
off another 0.2ms or so...

```
part1: 56120
part2: 24389045529
took 4.871825ms
```

Then, we can keep the worry levels of the items in a u32 instead of u64,
that might help a bit:

```
part1: 56120
part2: 24389045529
took 4.400663ms
```

Well, from 7.2 ms to 4.4ms is not too bad!
