use runner::Ctx;
use std::fmt::Write;

fn calorie_list(input: &str) -> Vec<u64> {
    let mut cals = input.split("\n\n")
        .map(|grp| grp.split("\n").filter_map(|i| i.parse::<u64>().ok()).sum::<u64>())
        .collect::<Vec<_>>();
    cals.sort_unstable();
    cals
}

pub fn start(ctx: &mut Ctx) {
    let cals = calorie_list(ctx.input());
    ctx.update_timer(Ctx::PARSING);

    let _ = writeln!(ctx, "part1: highest: {}", cals.iter().rev().next().unwrap());
    ctx.update_timer(Ctx::PART1);

    let _ = writeln!(ctx, "sum of 3 higest: {}", cals.iter().rev().take(3).sum::<u64>());
    ctx.update_timer(Ctx::PART2);
}
