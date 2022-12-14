use std::cmp::Ordering;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let packets = ctx
        .input()
        .trim()
        .split('\n')
        .filter(|l| l.len() != 0)
        .map(|l| Packet::parse(l))
        .collect::<Vec<_>>();
    ctx.update_timer(Timer::Parsing);

    let p1 = packets
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, pair)| (pair[0] <= pair[1]).then(|| idx + 1))
        .sum::<usize>();
    outputln!(ctx, "part1: {}", p1);
    ctx.update_timer(Timer::Part1);

    let two = Packet::parse("[[2]]");
    let six = Packet::parse("[[6]]");
    let under_two = packets.iter().filter(|packet| *packet < &two).count();
    let under_six = packets.iter().filter(|packet| *packet < &six).count();
    outputln!(ctx, "part2: {}", (under_two + 1) * (under_six + 2));
    ctx.update_timer(Timer::Part2);
}

#[derive(Debug, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> Packet {
        Packet::parse2(&line.as_bytes()[1..]).1
    }

    #[inline]
    fn parse2(line: &[u8]) -> (usize, Packet) {
        let mut idx = 0;
        let mut list = Vec::new();
        loop {
            match line[idx] {
                b'[' => {
                    let (nidx, nlist) = Packet::parse2(&line[idx + 1..]);
                    idx += nidx + 2;
                    list.push(nlist);
                },
                b']' => break,
                b',' => idx += 1,
                b if b >= b'0' && b <= b'9' => {
                    let mut n = 0u32;
                    while line[idx] >= b'0' && line[idx] <= b'9' {
                        n = n * 10 + (line[idx] - b'0') as u32;
                        idx += 1;
                    }
                    list.push(Packet::Number(n));
                },
                x => panic!("unexpected: <{}>", x),
            }
        }
        (idx, Packet::List(list))
    }

    fn compare_lists(&self, l1: &[Packet], l2: &[Packet]) -> Ordering {
        let mut idx = 0;
        while idx < l1.len() && idx < l2.len() {
            match l1[idx].compare(&l2[idx]) {
                Ordering::Equal => {},
                other => return other,
            }
            idx += 1;
        }
        if idx < l1.len() {
            Ordering::Greater
        } else if idx < l2.len() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(n2),
            (&Packet::Number(n), Packet::List(l)) => self.compare_lists(&[Packet::Number(n)], l),
            (Packet::List(l), &Packet::Number(n)) => self.compare_lists(l, &[Packet::Number(n)]),
            (Packet::List(l1), Packet::List(l2)) => self.compare_lists(l1, l2),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, right: &Self) -> bool {
        self.compare(right) == Ordering::Equal
    }
}
impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
        Some(self.compare(right))
    }
}

impl Ord for Packet {
    fn cmp(&self, right: &Self) -> Ordering {
        self.compare(right)
    }
}
