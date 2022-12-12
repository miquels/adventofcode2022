# [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

The input is basically a set of nodes, connected to each other by edges
with certain weights.

- the square is a block
- each block has a height 1..26
- adjacent squares up/down/left/right are connected if their height
  is lower _or_ max. one bigger than our heigtht
- the distance to an adjacent square is always 1.

### Part 1.

Find the shortest path from the source to the destination. Dijkstra!

### Part 2.

