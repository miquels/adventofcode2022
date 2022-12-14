use runner::*;

pub fn start_notree(ctx: &mut Ctx) {
    let input = ctx.input();
    let dirs = scan(input);
    ctx.update_timer(Ctx::PARSING);

    outputln!(ctx, "part1: {}", dirs
        .iter()
        .filter_map(|&n| (n < 100000).then(|| n))
        .sum::<u32>()
    );
    ctx.update_timer(Ctx::PART1);

    let unused = 70000000 - dirs[0];
    let to_delete = 30000000 - unused;

    outputln!(ctx, "part2: {}", dirs
        .iter()
        .filter_map(|&n| (n > to_delete).then(|| n))
        .min()
        .unwrap()
    );
    ctx.update_timer(Ctx::PART2);
}

enum Mode {
    Cmd,
    Ls,
}

fn scan(input: &str) -> Vec<u32> {
    let mut dirs = Vec::new();
    let mut stack = Vec::new();

    let mut curdir = 0;
    dirs.push(0);
    let mut mode = Mode::Cmd;

    let lines = input.trim().as_bytes().split(|&b| b == b'\n');

    for mut line in lines {
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
                            b".." => if stack.len() > 0 {
                                let tot_subdir = dirs[curdir];
                                curdir = stack.pop().unwrap();
                                dirs[curdir] += tot_subdir;
                            },
                            b"/" => {},
                            _ => {
                                stack.push(curdir);
                                curdir = dirs.len();
                                dirs.push(0);
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
                    dirs[curdir] += atou(w1);
                }
            },
        }
    }

    while stack.len() > 0 {
        let tot_subdir = dirs[curdir];
        curdir = stack.pop().unwrap();
        dirs[curdir] += tot_subdir;
    }

    dirs
}

fn atou(r: &[u8]) -> u32 {
    let mut n = 0;
    for i in 0 .. r.len() {
        n = n * 10 + (r[i] - b'0') as u32;
    }
    n
}
