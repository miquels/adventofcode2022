use std::collections::VecDeque;

pub fn part1(input: &str) {
    let mut m = parse(input);
    for _ in 0 .. 20 {
        round(&mut m);
    }
    // println!("{:#?}", m);
    println!("part1: {}", monkey_business(&mut m));
}

#[derive(Default, Debug)]
enum Op {
    #[default]
    Add,
    Mul,
}

#[derive(Default, Debug)]
enum Val {
    #[default]
    Old,
    Num(u32),
}

#[derive(Default, Debug)]
struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    val: Val,
    test_div: u32,
    next_true: usize,
    next_false: usize,
    inspected: u32,
}

fn round(monkeys: &mut [Monkey]) {
    for i in 0 .. monkeys.len() {
        while let Some(mut item) = monkeys[i].items.pop_front() {
            let m = &mut monkeys[i];
            let val = match m.val {
                Val::Old => item,
                Val::Num(num) => num,
            };
            item = match m.op {
                Op::Add => item + val,
                Op::Mul => item * val,
            };
            m.inspected += 1;
            item /= 3;
            let next = if item % m.test_div == 0 { m.next_true } else { m.next_false };
            monkeys[next].items.push_back(item);
        }
    }
}

fn monkey_business(m: &mut [Monkey]) -> u32 {
    m.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    m[0].inspected * m[1].inspected
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
                m.op = if &line[23..24] == "+" { Op::Add } else { Op::Mul };
                m.val = line[25..].parse::<u32>().map(Val::Num).unwrap_or(Val::Old);
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
