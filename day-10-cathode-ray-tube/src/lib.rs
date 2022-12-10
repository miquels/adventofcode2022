pub fn part1_2(input: &str) {
    let xv = run_program(input);
    part1(&xv);
    part2(&xv);
}

fn part1(xv: &[i32]) {
    let p1 = [ 20, 60, 100, 140, 180, 220 ].into_iter().map(|c| c * xv[c as usize]).sum::<i32>();
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
    let mut xv = vec![ 1, 1 ];
    let mut x = 1;
    for line in input.trim().split("\n") {
        if line.starts_with("n") {
            xv.push(x);
        } else {
            xv.push(x);
            x += line[5..].parse::<i32>().unwrap();
            xv.push(x);
        }
    }
    xv
}
