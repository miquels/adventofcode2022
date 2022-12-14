use runner::*;

pub fn start_heavy_duty(ctx: &mut Ctx) {
    let input = ctx.input();

    let mut lines = input.trim_end().split("\n");
    let mut stacks = Stacks::parse_stacks(&mut lines);
    let cmds = Cmd::parse_cmds(&mut lines);
    ctx.update_timer(Ctx::PARSING);

    let mut sclone = stacks.clone();
    sclone.cratemover_9000(&cmds);
    outputln!(ctx, "part1: {}", sclone.top_crates());
    ctx.update_timer(Ctx::PART1);

    stacks.cratemover_9001(&cmds);
    outputln!(ctx, "part2: {}", stacks.top_crates());
    ctx.update_timer(Ctx::PART2);
}

struct Cmd {
    num:    u32,
    from:   u32,
    to:     u32,
}

impl Cmd {
    fn parse_cmds<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
        let mut cmds = Vec::new();
        for cmd in lines {
            let mut nums = cmd.split(' ').skip(1).step_by(2).map(|w| atou(w));
            let [num, from, to] = std::array::from_fn(|_| nums.next().unwrap());
            cmds.push(Cmd { num, from, to });
        }
        cmds
    }
}

#[derive(Clone, Debug)]
struct Pos {
    stack: usize,
    depth: usize,
}

#[derive(Clone)]
struct Stacks {
    crates: Vec<Vec<u8>>,
    tops: Vec<Pos>,
}

impl Stacks {
    fn cratemover_9000(&mut self, cmds: &[Cmd]) {
        for cmd in cmds.into_iter().rev() {
            let (from, to) = ((cmd.from - 1) as usize, (cmd.to - 1) as usize);
            let num = cmd.num as usize;
            for top in &mut self.tops {
                if to == top.stack {
                    if num >= top.depth {
                        top.stack = from;
                        top.depth = num - top.depth + 1;
                    } else {
                        top.depth -= num;
                    }
                } else if from == top.stack {
                    top.depth += num;
                }
            }
        }
    }

    fn cratemover_9001(&mut self, cmds: &[Cmd]) {
        for cmd in cmds.into_iter().rev() {
            let (from, to) = ((cmd.from - 1) as usize, (cmd.to - 1) as usize);
            let num = cmd.num as usize;
            for top in &mut self.tops {
                if to == top.stack {
                    if num >= top.depth {
                        top.stack = from;
                    } else {
                        top.depth -= num;
                    }
                } else if from == top.stack {
                    top.depth += num;
                }
            }
        }
    }

    fn top_crates(&self) -> String {
        let mut t = String::new();
        // println!("tops: {:?}", self.tops);
        for top in &self.tops {
            let s = &self.crates[top.stack];
            let c = s[s.len() - top.depth];
            t.push(c as char);
        }
        t
    }

    fn parse_stacks<'a>(mut lines: impl Iterator<Item = &'a str>) -> Stacks {
        let mut crates = Vec::new();
        let mut tops = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.as_bytes();
            if line[1] == b'0' || line[1] == b'1' {
                lines.next();
                break;
            }
            let mut idx = 0;
            while idx * 4 + 1 < line.len() {
                let c = line[idx * 4 + 1];
                if crates.len() <= idx {
                    crates.push(Vec::new());
                }
                if c >= b'A' && c <= b'Z' {
                    crates[idx].push(c);
                }
                idx += 1;
            }
        }
        crates.iter_mut().for_each(|s| s.reverse());

        for i in 0 .. crates.len() {
            tops.push(Pos { stack: i, depth: 1 });
        }

        Stacks { crates, tops }
    }
}

fn atou(r: &str) -> u32 {
    let mut n = 0;
    let r = r.as_bytes();
    for i in 0 .. r.len() {
        n = n * 10 + (r[i] - b'0') as u32;
    }
    n
}
