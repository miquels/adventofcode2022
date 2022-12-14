use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let mut p1 = 0u32;
    let mut p2 = 0u32;
    for line in input.trim().as_bytes().split(|b| *b == b'\n') {
        let mut idx = 0;
        let l = parse_range(line, &mut idx);
        let r = parse_range(line, &mut idx);
        p1 += ((l & r) == l || (l & r) == r) as u32;
        p2 += ((l & r) > 0) as u32;
    }
    outputln!(ctx, "part1: {}", p1);
    outputln!(ctx, "part2: {}", p2);
    ctx.update_timer(Ctx::PART12);
}

fn parse_range(range: &[u8], idx: &mut usize) -> u128 {
    let l = next_num(range, idx);
    let r = next_num(range, idx);
    (1u128 << (r + 1)) - (1u128 << l)
}

fn next_num(r: &[u8], idx: &mut usize) -> u8 {
    let mut n = 0;
    while *idx < r.len() && r[*idx] >= b'0' && r[*idx] <= b'9' {
        n = n * 10 + r[*idx] - b'0';
        *idx += 1;
    }
    *idx += 1;
    n
}

// Code below is from the initial solution before much optimization.
fn parse_range1(range: &str) -> u128 {
    let (l, r) = range.split_once('-').unwrap();
    let (l, r) = (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap());
    (1u128 << (r + 1)) - (1u128 << l)
}

pub fn part1(input: &str) {
    let r = input
        .trim()
        .split("\n")
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let l = parse_range1(l);
            let r = parse_range1(r);
            ((l & r) == l || (l & r) == r) as u32
        })
        .sum::<u32>();
    println!("part1: {}", r);
}

pub fn part2(input: &str) {
    let r = input
        .trim()
        .split("\n")
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let l = parse_range1(l);
            let r = parse_range1(r);
            ((l & r) > 0) as u32
        })
        .sum::<u32>();
    println!("part2: {}", r);
}

