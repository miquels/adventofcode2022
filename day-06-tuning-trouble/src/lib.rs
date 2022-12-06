pub fn part1(input: &str) {
    let o = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find_map(|(i, w)| all_different(w).then(|| i))
        .unwrap();
    println!("part1: {}", o + 4);
}

fn all_different(w: &[u8]) -> bool {
    w[0] != w[1] && w[0] != w[2] && w[0] != w[3] &&
        w[1] != w[2] && w[1] != w[3] &&
        w[2] != w[3]
}

pub fn part2(_input: &str) {
}
