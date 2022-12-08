
[Day8: Treetop Tree House](https://adventofcode.com/2022/day/8)

The input is a 99x99 grid of numbers 0-9. Each number indicates the height of a tree.

### Part 1.

How many trees are visible from the outside because there are no
or only lower trees in front of it.

The simple solution here is to just walk around the grid and look inside,
counting each tree and keeping a maxheight until a tree with height <= maxheight
is next, or the last tree had height '9' (max possible). We must also mark
which trees we have counted so we don't count them twice.

### Part 2.

Count the amount of trees we can see when we look in 4 directions around us
from any tree.  Then multiple those amounts - this is the "scenic score".
Which tree has the highest scenic score.

Here we must check each tree in succession, count the trees visible etc.

Something that took me a while to discover is that the trees on the edges
are _not_ considered viable! That is not made very clear in the puzzle.

### Optimizations.

There is a lot of indexing going on here and that is always costly. Maybe
the compiler will elide some index overflow checking if we hardcode
the grid size.

Before:

```
part1: == start ==
part1: 1684
part1: took 59.934µs
part2: == start ==
part2: 486540
part2: took 455.925µs
```

After:

```
part1: == start ==
part1: 1684
part1: took 42.572µs
part2: == start ==
part2: 486540
part2: took 484.579µs
```

That does help, but weirdly enough, part2 got a bit slower? Maybe if we make
`look_around()` a bit smarter, it does way too many checks in the hot path.

```
part1: == start ==
part1: 1684
part1: took 44.76µs
part2: == start ==
part2: 486540
part2: took 393.095µs
```

Better.
