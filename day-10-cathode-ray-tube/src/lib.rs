pub fn part1_2(input: &str) {
    let xv = run_program(input);
    //println!("{:?}", xv);
    let p1 = [ 20, 60, 100, 140, 180, 220 ].into_iter().map(|c| c * xv[c as usize]).sum::<i32>();
    println!("part1: {}", p1);
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
