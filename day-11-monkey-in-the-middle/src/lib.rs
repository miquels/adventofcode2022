use std::collections::VecDeque;

pub fn part1_2(input: &str) {
    let mut monkeys = parse(input);
    let mut clones = monkeys.clone();

    let modulo = monkeys.iter().map(|m| m.test_div).product::<u32>() as u64;

    for _ in 0 .. 20 {
        round::<3>(&mut monkeys, modulo);
    }
    println!("part1: {}", monkey_business(&mut monkeys));

    for _ in 0 .. 10000 {
        round::<1>(&mut clones, modulo);
    }
    println!("part2: {}", monkey_business(&mut clones));
}

#[derive(Default, Debug, Clone)]
enum Op {
    Add(u32),
    Mul(u32),
    #[default]
    Sq,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    test_div: u32,
    next_true: usize,
    next_false: usize,
    inspected: u32,
}

fn round<const RELIEF: u64>(monkeys: &mut [Monkey], modulo: u64) {
    for i in 0 .. monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let m = &mut monkeys[i];
            let mut item = match m.op {
                Op::Sq => item as u64 * item as u64,
                Op::Add(val) => (item + val) as u64,
                Op::Mul(val) => (item * val) as u64,
            };
            if item > modulo {
                item %= modulo;
            }
            let item = (item / RELIEF) as u32;
            m.inspected += 1;
            let next = if item % m.test_div == 0 { m.next_true } else { m.next_false };
            monkeys[next].items.push_back(item);
        }
    }
}

fn monkey_business(m: &mut [Monkey]) -> u64 {
    m.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    m[0].inspected as u64 * m[1].inspected as u64
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut m = Monkey::default();
    for (idx, line) in input.lines().enumerate() {
        match idx % 7 {
            0 => {},
            1 => {
                m.items = line[18..].split(", ").map(|n| n.parse::<u32>().unwrap()).collect();
            },
            2 => {
                m.op = if &line[23..] == "* old" {
                    Op::Sq
                } else {
                    let val = line[25..].parse::<u32>().unwrap();
                    if &line[23..24] == "+" { Op::Add(val) } else { Op::Mul(val) }
                };
            },
            3 => m.test_div = line[21..].parse().unwrap(),
            4 => m.next_true = line[29..].parse().unwrap(),
            5 => {
                m.next_false = line[30..].parse().unwrap();
                monkeys.push(m);
                m = Monkey::default();
            },
            6 => {},
            _ => unreachable!(),
        }
    }
    monkeys
}
