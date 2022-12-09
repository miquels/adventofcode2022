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

parsing: 102.937µs
part1: 5874
part1: 696.678µs
part2: 2467
part2: 654.306µs
took 1.468628ms

Interestingly, part2 isn't slower than one, which you would expect.

