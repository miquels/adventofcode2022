use std::collections::HashMap;

pub fn part1(input: &str) {
    let mut root = Directory::default();
    let _ = root.cd(true, input.lines());
    let mut tot = 0;
    root.walk(&mut |node| {
        if node.size() < 100000 {
            tot += node.size();
        }
    });
    println!("part1: {}", tot);
}

pub fn part2(_input: &str) {
    println!("N/A");
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
                    self.entries.insert(w2.to_string(), Node::new(w1));
                },
            }
        }
        input
    }

    fn size(&self) -> u32 {
        self.entries.values().map(|n| n.size()).sum()
    }

    fn walk(&self, f: &mut impl FnMut(&Node)) {
        self.entries.values().for_each(|node| {
            if let Node::Directory(subdir) = node {
                f(node);
                subdir.walk(f);
            }
        });
    }
}
