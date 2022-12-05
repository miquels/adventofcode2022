
pub fn part1_2(input: &str) {
    let mut lines = input.trim_end().split("\n");
    let mut stacks = Stacks::parse_stacks(&mut lines);
    let cmds = Cmd::parse_cmds(&mut lines);

    let mut sclone = stacks.clone();
    sclone.cratemover_9000(&cmds);
    println!("day-05: part1: {}", sclone.top_crates());

    stacks.cratemover_9001(&cmds);
    println!("day-05: part2: {}", stacks.top_crates());
}

struct Cmd {
    num:    usize,
    from:   usize,
    to:     usize,
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

#[derive(Clone)]
struct Stacks(Vec<Vec<u8>>);

impl Stacks {
    fn cratemover_9000(&mut self, cmds: &[Cmd]) {
        for cmd in cmds {
            for _ in 0 .. cmd.num {
                let c = self.0[cmd.from - 1].pop().unwrap();
                self.0[cmd.to - 1].push(c);
            }
        }
    }

    fn cratemover_9001(&mut self, cmds: &[Cmd]) {
        for cmd in cmds {
            let mut tmp = std::mem::replace(&mut self.0[cmd.to - 1], Vec::new());
            let i = self.0[cmd.from-1].len() - cmd.num;
            let c = &self.0[cmd.from-1][i..];
            tmp.extend_from_slice(c);
            self.0[cmd.from-1].truncate(i);
            self.0[cmd.to - 1] = tmp;
        }
    }

    fn top_crates(&self) -> String {
        self.0.iter().map(|s| s[s.len()-1] as char).collect()
    }

    fn parse_stacks<'a>(mut lines: impl Iterator<Item = &'a str>) -> Stacks {
        let mut stacks = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.as_bytes();
            if line[1] == b'1' {
                lines.next();
                break;
            }
            let mut idx = 0;
            while idx * 4 + 1 < line.len() {
                let c = line[idx * 4 + 1];
                if stacks.len() <= idx {
                    stacks.push(Vec::new());
                }
                if c >= b'A' && c <= b'Z' {
                    stacks[idx].push(c);
                }
                idx += 1;
            }
        }
        stacks.iter_mut().for_each(|s| s.reverse());
        Stacks(stacks)
    }
}

fn atou(r: &str) -> usize {
    let mut n = 0;
    let r = r.as_bytes();
    for i in 0 .. r.len() {
        n = n * 10 + (r[i] - b'0') as usize;
    }
    n
}
