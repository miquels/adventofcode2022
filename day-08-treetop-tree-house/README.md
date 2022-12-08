
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
