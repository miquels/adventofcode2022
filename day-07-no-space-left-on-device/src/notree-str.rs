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

#[derive(Debug)]
struct Directory<'a> {
    dirs: Vec<u32>,
    lines: std::str::Lines<'a>,
}

impl<'a> Directory<'a> {
    fn parse(input: &'a str) -> Directory {
        let mut d = Directory {
            dirs: Vec::new(),
            lines: input.lines(),
        };
        d.scan();
        d
    }

    fn scan(&mut self) -> u32 {
        let curdir = self.dirs.len();
        self.dirs.push(0);
        let mut mode = Mode::Cmd;

        while let Some(mut line) = self.lines.next() {
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
                                ".." => if curdir != 0 {
                                    return self.dirs[curdir];
                                },
                                "/" => {},
                                _ => self.dirs[curdir] += self.scan(),
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
        self.dirs[curdir]
    }
}
