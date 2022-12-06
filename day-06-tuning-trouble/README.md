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

There are indeed bitbang procedures to see if all characters in a string
are different, for example:

```
fn all_different(w: &[u8]) -> bool {
    let mut set = 0u128;
    !w.into_iter().any(|v| if (set & 1 << v) != 0 { true } else { set |= 1 << v; false })
}
```

This is slower.

Maybe:

```
fn all_different2(w: &[u8]) -> bool {
    w.into_iter().fold(0u128, |set, v| set | 1 << v).count_ones() == w.len() as u32
}
```

Also slower.

Maybe if we take advantage of the fact that all characters in the string are
between 'a' and 'z', so we can use a bitset of size 32 instead of 128?

```
fn all_different(w: &[u8]) -> bool {
    w.into_iter().fold(0u32, |set, v| set | 1 << (v - b'a')).count_ones() == w.len() as u32
}
```

Success!

```
day-06: tuning-trouble
part1: == start ==
part1: 1100
part1: took 6.944µs
part2: == start ==
part2: 2421
part2: took 18.336µs
```
