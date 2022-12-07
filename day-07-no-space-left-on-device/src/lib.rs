use std::collections::HashMap;
use std::time::Instant;

pub fn part1_2(input: &str) {
    let now = Instant::now();
    let mut root = Directory::default();
    let _ = root.cd(true, input.lines());
    println!("part1: parsing: {:?}", now.elapsed());

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
    println!("part2 iter: {}",
        root
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

#[derive(Debug)]
enum Node {
    File(u32),
    Directory(Directory),
}

impl Node {
    fn new(type_: &str) -> Node {
        match type_.parse::<u32>() {
            Ok(size) => Node::File(size),
            Err(_) => Node::Directory(Directory::default()),
        }
    }

    fn size(&self) -> u32 {
        match self {
            Node::File(size) => *size,
            Node::Directory(dir) => dir.size(),
        }
    }
}

#[derive(Default, Debug)]
struct Directory {
    entries: HashMap<String, Node>,
}

impl Directory {

    fn cd<'a, I>(&mut self, root: bool, mut input: I) -> I
    where
        I: Iterator<Item = &'a str>,
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
                                name => match self.entries.get_mut(name) {
                                    Some(Node::Directory(dir)) => input = dir.cd(false, input),
                                    Some(_) => panic!("cd into a file: {}", name),
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
                    if self.entries.insert(w2.to_string(), Node::new(w1)).is_some() {
                        println!("DUP");
                    }
                },
            }
        }
        input
    }

    fn size(&self) -> u32 {
        self.entries.values().map(|n| n.size()).sum()
    }

    fn iter(&self) -> Walker<'_> {
        Walker { entries: self.entries.values().collect() }
    }
}

struct Walker<'a> {
    entries: Vec<&'a Node>,
}

impl<'a> Iterator for Walker<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.entries.pop() {
            if let Node::Directory(ref d) = node {
                self.entries.extend(d.entries.values());
                return Some(node);
            }
        }
        None
    }
}
