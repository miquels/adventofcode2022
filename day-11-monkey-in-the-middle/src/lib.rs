use std::collections::VecDeque;
use num::integer::lcm;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let mut monkeys = parse(input);
    let mut clones = monkeys.clone();
    let modulo = monkeys.iter().map(|m| m.test_div).fold(1u64, |acc, x| lcm(acc, x));
    ctx.update_timer(Ctx::PARSING);

    for _ in 0 .. 20 {
        round::<3>(&mut monkeys, modulo);
    }
    outputln!(ctx, "part1: {}", monkey_business(&mut monkeys));
    ctx.update_timer(Ctx::PART1);

    for _ in 0 .. 10000 {
        round::<1>(&mut clones, modulo);
    }
    outputln!(ctx, "part2: {}", monkey_business(&mut clones));
    ctx.update_timer(Ctx::PART2);
}

#[derive(Default, Debug, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    #[default]
    Sq,
    Double,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test_div: u64,
    next_true: usize,
    next_false: usize,
    inspected: u64,
}

fn round<const RELIEF: u64>(monkeys: &mut [Monkey], modulo: u64) {
    for i in 0 .. monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let m = &mut monkeys[i];
            let mut item = match m.op {
                Op::Sq => item as u64 * item as u64,
                Op::Double => item as u64 * 2,
                Op::Add(val) => (item + val) as u64,
                Op::Mul(val) => (item * val) as u64,
            };
            item = (item / RELIEF) as u64;
            if item > modulo {
                item %= modulo;
            }
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
        // println!("{idx} {line}");
        match idx % 7 {
            0 => {},
            1 => {
                m.items = line[18..].split(", ").map(|n| n.parse::<u64>().unwrap()).collect();
            },
            2 => {
                m.op = if &line[23..] == "* old" {
                    Op::Sq
                } else if &line[23..] == "+ old" {
                    Op::Double
                } else {
                    let val = line[25..].parse::<u64>().unwrap();
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
