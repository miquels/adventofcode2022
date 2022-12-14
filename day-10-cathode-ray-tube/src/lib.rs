use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();
    let xv = run_program(input);
    ctx.update_timer(Ctx::PARSING);

    part1(ctx, &xv);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, &xv);
    ctx.update_timer(Ctx::PART2);
}

fn part1(ctx: &mut Ctx, xv: &[i32]) {
    let p1 = (20..=220).step_by(40).map(|c| c * xv[c as usize]).sum::<i32>();
    outputln!(ctx, "part1: {}", p1);
}

fn part2(ctx: &mut Ctx, xv: &[i32]) {
    let mut crt = String::new();
    for p in 0 .. 240 {
        let x = xv[(p + 1) as usize];
        let c = p % 40;
        crt.push_str(if c >= x - 1 && c <= x + 1 { "#" } else { " " });
        if c == 39 {
            crt.push_str("\n");
        }
    }
    outputln!(ctx, "{}", crt);
}

fn run_program(input: &str) -> Vec<i32> {
    let (mut x, mut xv) = (1, vec![ 1, 1 ]);
    for line in input.trim().split("\n") {
        xv.push(x);
        if line.starts_with("a") {
            x += line[5..].parse::<i32>().unwrap();
            xv.push(x);
        }
    }
    xv
}
