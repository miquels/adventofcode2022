use std::time::Instant;

pub fn part1_2_notree(input: &str) {
    let now = Instant::now();
    let dirs = Directory::parse(input);
    println!("parsing: {:?}", now.elapsed());

    let now = Instant::now();
    println!("part1: {}", dirs.dirs
        .iter()
        .filter_map(|&n| (n < 100000).then(|| n))
        .sum::<u32>()
    );
    println!("part1: {:?}", now.elapsed());

    let unused = 70000000 - dirs.dirs[0];
    let to_delete = 30000000 - unused;

    let now = Instant::now();
    println!("part2: {}", dirs.dirs
        .iter()
        .filter_map(|&n| (n > to_delete).then(|| n))
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
struct Directory {
    dirs: Vec<u32>,
    stack: Vec<usize>,
}

impl Directory {
    fn parse(input: &str) -> Directory {
        let mut dir = Directory::default();
        dir.scan(input);
        dir
    }

    fn scan(&mut self, input: &str) {
        self.dirs.push(0);
        let mut curdir = 0;
        let mut mode = Mode::Cmd;

        for mut line in input.trim().split('\n') {
            if line.as_bytes()[0] == b'$' {
                line = &line[2..];
                mode = Mode::Cmd;
            }
            let mut words = line.split(' ');
            match mode {
                Mode::Cmd => {
                    match words.next().unwrap() {
                        "cd" => {
                            match words.next().unwrap() {
                                ".." => if self.stack.len() > 0 {
                                    let total_subdir = self.dirs[curdir];
                                    curdir = self.stack.pop().unwrap();
                                    self.dirs[curdir] += total_subdir;
                                },
                                "/" => {},
                                _ => {
                                    self.stack.push(curdir);
                                    self.dirs.push(0);
                                    curdir = self.dirs.len() - 1;
                                }
                            }
                        },
                        "ls" => mode = Mode::Ls,
                        other => panic!("unknown command {}", other),
                    }
                },
                Mode::Ls => {
                    let w1 = words.next().unwrap();

                    if let Ok(size) = w1.parse::<u32>() {
                        self.dirs[curdir] += size;
                    }
                },
            }
        }

        while self.stack.len() > 0 {
            let total_subdir = self.dirs[curdir];
            curdir = self.stack.pop().unwrap();
            self.dirs[curdir] += total_subdir;
        }
    }
}
