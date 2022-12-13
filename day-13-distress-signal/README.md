# [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)

The input is arrays of (array | number), one per line, like this:

```
[[1],[2,3,4], 5]
```

This is easy to parse with a recursive descent parser. The only recursion is at `[`.
We parse it into an enum:
```
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}
```

### Part 1.

Interpret the input packets as pairs of two, and see if the pairs are listed
"in the right order". An algorithm is given that specifies how to compare
the two arrays. We can implement `PartialOrd` for `Packet` and then
"the right order" is `packet1 <= packet2`.

### Part 2.

Put all packet in one list and add two divider packets `[[2]]` and `[[6]]`.
Order the list. Find the poition of the divider packets.

We already have the packets in a `Vec`. Just implement `Ord` for `Packet`
and we can simply sort the `Vec`.

### Optimizations.

The bottleneck is the parser, as we can see:

```
parsing: 687.334µs
part1: 6072
part1: 11.42µs
part2: 22184
part2: 112.968µs
took 1.180971ms
```

Let's see if we can make it faster:

- it's probably all the index operations in the parser.
  - replace indexing with an `Iterator` over `u8` bytes.
  - same speed.

- use `serde_json` to deserialize
  - 3.2ms !

- use `simd_json` to deserialize
  - 3.2ms ! (line-by-line)
  - even if we change the entire input to one big json array and parse it all at once:
    - 3.0ms !

Okay, we're not going to get this any faster.

