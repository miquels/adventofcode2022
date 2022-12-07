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

We use a `HashMap` for the directory, but the map from `std` doesn't use
the fastest hashing algoritm there is (for good reasons). We can swap
the algorithm with one from the `rustc-hash` crate:

Before:

```
parsing: 197.575µs
part1: 1886043
part1: 20.538µs
part2: 3842121
part2: 22.826µs
took 288.656µs
```

After:

```
parsing: 152.918µs
part1: 1886043
part1: 21.581µs
part2: 3842121
part2: 24.214µs
took 243.471µs
```

It is slightly faster. Now, perhaps we can get rid of String allocations,
and refer to the input directly.

```
parsing: 138.581µs
part1: 1886043
part1: 21.412µs
part2: 3842121
part2: 26.911µs
took 224.577µs
```

Wait a minute. This isn't a real filesystem, we don't need to remember
any file itself .. only the total size of all files in a directory.

So now:

```
parsing: 98.953µs
part1: 1886043
part1: 9.365µs
part2: 3842121
part2: 3.55µs
took 129.38µs
```

Better.
