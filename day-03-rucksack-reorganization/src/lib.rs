use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    part1(ctx, input);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, input);
    ctx.update_timer(Ctx::PART2);
}

fn index(b: u8) -> usize {
    // a-z = 0-25, A-Z = 26-51.
    let r = (b as usize & 31) + 26 * ((b & 32) == 0) as usize - 1;
    assert!(r < 52);
    r
}

pub fn part1(ctx: &mut Ctx, input: &str) {
    let r = input
        .trim()
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut table = [0u8; 52];
            let middle = line.len() / 2;
            for i in 0 .. line.len() {
                let p = index(line[i]);
                table[p] |= 1 + (i >= middle) as u8;
            }
            (0..52).find_map(|i| (table[i] == 3).then(|| i)).unwrap() + 1
        })
        .sum::<usize>();
    outputln!(ctx, "part1: {}", r);
}

pub fn part2(ctx: &mut Ctx, input: &str) {
    let mut table = [0u8; 52];
    let r = input
        .trim()
        .as_bytes()
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(i, line)| {
            let g = i % 3;

            // First of the group: zero the table.
            if g == 0 {
                table[0..52].fill(0);
            }

            // Update item table.
            for i in 0 .. line.len() {
                let p = index(line[i]);
                table[p] |= 1 << g;
            }

            if g == 2 {
                // Last of the group: find item common in all 3.
                (0..52).find_map(|i| (table[i] == 7).then(|| i)).unwrap() + 1
            } else {
                0
            }
        })
        .sum::<usize>();
    outputln!(ctx, "part2: {}", r);
}
