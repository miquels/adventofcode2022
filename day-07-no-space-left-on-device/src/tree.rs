use std::time::Instant;

pub fn part1_2(input: &str) {
    let now = Instant::now();
    let root = Directory::parse(input);
    println!("parsing: {:?}", now.elapsed());

    let now = Instant::now();
    println!("part1: {}", root
        .iter()
        .filter_map(|n| (n.size() < 100000).then(|| n.size()))
        .sum::<u32>()
    );
    println!("part1: {:?}", now.elapsed());

    let unused = 70000000 - root.size();
    let to_delete = 30000000 - unused;

    let now = Instant::now();
    println!("part2: {}", root
        .iter()
        .filter_map(|n| (n.size() > to_delete).then(|| n.size()))
        .min()
        .unwrap()
    );
    println!("part2: {:?}", now.elapsed());
}

enum Mode {
    Cmd,
    Ls,
}

#[derive(Debug, Default)]
struct Directory<'a> {
    entries: Vec<(&'a str, Directory<'a>)>,
    size: u32,
}

impl<'a> Directory<'a> {

    fn parse(input: &'a str) -> Directory<'a> {
        let mut root = Directory::default();
        let _ = root.cd(true, input.lines());
        root
    }

    fn cd<'i: 'a, I>(&mut self, root: bool, mut input: I) -> I
    where
        I: Iterator<Item = &'i str>,
    {
        let mut mode = Mode::Cmd;
        while let Some(mut line) = input.next() {
            if let Some(l) = line.strip_prefix("$ ") {
                line = l;
                mode = Mode::Cmd;
            }
            let mut words = line.trim().split(' ');
            match mode {
                Mode::Cmd => {
                    match words.next().unwrap() {
                        "cd" => {
                            match words.next().unwrap() {
                                ".." => if !root { return input },
                                name => match self.entries.iter().position(|e| e.0 == name) {
                                    Some(idx) => {
                                        let dir = &mut self.entries[idx].1;
                                        input = dir.cd(false, input);
                                        self.size += dir.size;
                                    },
                                    None if root => {},
                                    None => panic!("no such directory: {}", name),
                                },
                            }
                        },
                        "ls" => mode = Mode::Ls,
                        other => panic!("unknown command {}", other),
                    }
                },
                Mode::Ls => {
                    let (w1, w2) = (words.next().unwrap(), words.next().unwrap());
                    match w1.parse::<u32>() {
                        Ok(size) => self.size += size,
                        Err(_) => {
                            self.entries.push((w2, Directory::default()));
                        },
                    }
                },
            }
        }
        input
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn iter(&self) -> DirectoryIterator<'_> {
        DirectoryIterator { entries: self.entries.iter().map(|e| &e.1).collect() }
    }
}

struct DirectoryIterator<'a> {
    entries: Vec<&'a Directory<'a>>,
}

impl<'a> Iterator for DirectoryIterator<'a> {
    type Item = &'a Directory<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir) = self.entries.pop() {
            self.entries.extend(dir.entries.iter().map(|e| &e.1));
            return Some(dir);
        }
        None
    }
}
