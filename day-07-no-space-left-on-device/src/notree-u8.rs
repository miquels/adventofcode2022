use std::time::Instant;

pub fn part1_2_notree(input: &str) {
    //let now = Instant::now();
    let dirs = Directory::parse(input);
    //println!("parsing: {:?}", now.elapsed());

    //let now = Instant::now();
    println!("part1: {}", dirs.dirs
        .iter()
        .filter_map(|&n| (n < 100000).then(|| n))
        .sum::<u32>()
    );
    //println!("part1: {:?}", now.elapsed());

    let unused = 70000000 - dirs.dirs[0];
    let to_delete = 30000000 - unused;

    //let now = Instant::now();
    println!("part2: {}", dirs.dirs
        .iter()
        .filter_map(|&n| (n > to_delete).then(|| n))
        .min()
        .unwrap()
    );
    //println!("part2: {:?}", now.elapsed());
}

enum Mode {
    Cmd,
    Ls,
}

#[derive(Debug)]
struct Directory {
    dirs: Vec<u32>,
}

impl Directory {
    fn parse(input: &str) -> Directory {
        let mut d = Directory {
            dirs: Vec::new(),
        };
        let lines = input.trim().as_bytes().split(|&b| b == b'\n');
        let _ = d.scan(lines);
        d
    }

    fn scan<'a, I>(&mut self, mut lines: I) -> I
    where
        I: Iterator<Item = &'a [u8]>
    {
        let curdir = self.dirs.len();
        self.dirs.push(0);
        let mut mode = Mode::Cmd;

        while let Some(mut line) = lines.next() {
            if line[0] == b'$' {
                line = &line[2..];
                mode = Mode::Cmd;
            }
            let mut words = line.split(|&b| b == b' ');
            match mode {
                Mode::Cmd => {
                    match words.next().unwrap() {
                        b"cd" => {
                            match words.next().unwrap() {
                                b".." => if curdir != 0 {
                                    return lines;
                                },
                                b"/" => {},
                                _ => {
                                    let top = self.dirs.len();
                                    lines = self.scan(lines);
                                    self.dirs[curdir] += self.dirs[top];
                                },
                            }
                        },
                        b"ls" => mode = Mode::Ls,
                        _ => panic!("unknown command"),
                    }
                },
                Mode::Ls => {
                    let w1 = words.next().unwrap();
                    if w1 != b"dir" {
                        self.dirs[curdir] += atou(w1);
                    }
                },
            }
        }
        lines
    }
}

fn atou(r: &[u8]) -> u32 {
    let mut n = 0;
    for i in 0 .. r.len() {
        n = n * 10 + (r[i] - b'0') as u32;
    }
    n
}
