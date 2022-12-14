## [Regolith Reservoir](https://adventofcode.com/2022/day/14)

There's a cave, and from one point on the roof sand falls in. There's
some structures in the middle of the cave.

### Part 1.

The cave has no floor. Calculate after how many units of sand the structures
are covered and sands starts to fall into the infinite depths.

We run a simple step-by-step algorithm that tries down, down-left, down-right
for every unit of sand, until a unit falls down further than the lowest
structure.

### Part 2.

The cave has a floor. Calculate after how many units of sand the input
gets clogged and the sand stops coming.

Same algorithm, but run it until the input is clogged.

### Optimizations.

- tried a hashmap as a grid instead of a 2D array .. waaayy slower.
- started resizing the grid only on writes, reporting out-of-bounds
  reads as '.' (sky). This made it twice as fast, now:

```
day-14: parsing: 134.536µs
day-14: part1: units: 838
day-14: part1: 150.891µs
day-14: part2: units: 27539
day-14: part2: 5.820004ms
day-14: finishing: 8.279µs
day-14: total: 6.114436ms
```
