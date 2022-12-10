pub fn part1_2(input: &str) {
    let xv = run_program(input);
    part1(&xv);
    part2(&xv);
}

fn part1(xv: &[i32]) {
    let p1 = (20..=220).step_by(40).map(|c| c * xv[c as usize]).sum::<i32>();
    println!("part1: {}", p1);
}

fn part2(xv: &[i32]) {
    let mut crt = String::new();
    for p in 0 .. 240 {
        let x = xv[(p + 1) as usize];
        let c = p % 40;
        crt.push_str(if c >= x - 1 && c <= x + 1 { "#" } else { " " });
        if c == 39 {
            crt.push_str("\n");
        }
    }
    print!("{}", crt);
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
