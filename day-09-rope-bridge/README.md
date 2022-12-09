[Day9: Rope Bridge](https://adventofcode.com/2022/day/9)

The puzzle plays out on a grid, start position bottom left. The object
is to move around a "rope", which has two knots in it: a Head and a Tail,
with somewhat elastic string in between. Both the Head and the Tail start
out overlapped on the start position.

The input is a list of moves for the head. This drags the tail along.

### Part 1.

Find out on how many unique places the tail has been.

We don't need to draw a grid. We just need to step the head, then
calculate the next position for the tail, and step to that. Keep
a HashSet of tail positions. Step through the moves. When done,
the answer is the size of the HashSet.

### Part 2.

Same problem, but with ten knots. Each knot follows the knot before it.

Same solution, but we run the tail stepper algorithm 9 times, each time
with as head the knot in front and as tail the knot behind it.

### Optimization.

All that stepping isn't very fast:

```
parsing: 102.937µs
part1: 5874
part1: 696.678µs
part2: 2467
part2: 654.306µs
took 1.468628ms
```

Interestingly, part2 isn't slower than one, which you would expect.

The first optimization we can do is to step the head all at once instead
of one step at a time. That makes a difference, if not much:

```
parsing: 105.633µs
part1: 5874
part1: 677.689µs
part2: 2467
part2: 655.186µs
took 1.453947ms
```

Still it looks like much of the CPU time is going somewhere else. What if we
comment out the `HashSet.insert` ? The result will be wrong but it's just
to get an idea of the time taken bij the HashSet inserts:

Well:

```
parsing: 102.947µs
part1: 1
part1: 75.21µs
part2: 1
part2: 366.287µs
```

Woah. Look at the reduction in runtime for part1. Let's replace the `Hasher`
used by `HashSet` with a faster one from the `FxHash` crate.

```
parsing: 89.635µs
part1: 5874
part1: 357.323µs
part2: 2467
part2: 433.539µs
took 893.669µs
```

Okay, that helps. Still not very convincing but hey. Let's implement our
own CoordHashSet based on a `Vec<u64>`:

```
parsing: 120.96µs
part1: 5874
part1: 124.36µs
part2: 2467
part2: 335.268µs
took 597.1µs
```

Significantly better.

There are still two things to optimize:

- the parser phase, as usual, by using &[u8] and a hand-made number parser
- `step_tails()` could probably take larger steps than one-by-one

However, it's friday afternoon and I'm going to socialize!
