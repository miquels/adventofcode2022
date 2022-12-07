## [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)

The input of the puzzle is a list of files and directories, formatted
as unix "cd" and "ls" commands, and "ls" output. That's parsed into a
struct Directory, which contains a hashmap with Nodes which are either
a File or a Directory. Using recursion, ofcourse. The Directory has
2 methods:

- size: get the total size of the dir and all subdirs
- iter: returns an iterator over all directory nodes.

### Part 1.

Calculate the total size of all directories that have size < 100000.

```
dir.iter().filter_map(|n| (n.size() < 100000).then(|| n.size())).sum()
```

### Part 2.

```
dir.iter().filter_map(|n| (n.size() > to_delete).then(|| n.size())).min()
```

### Optimizations.

We use a `HashMap` for the directory, but actually, the names are not needed
for the puzzle, and the puzzle input never visits a directory twice.
So a `Vec` should suffice.

