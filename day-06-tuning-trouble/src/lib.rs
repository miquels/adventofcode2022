pub fn part1(input: &str) {
    println!("part1: {}", find_marker(input, 4) + 4);
}

pub fn part2(input: &str) {
    println!("part2: {}", find_marker(input, 14) + 14);
}

pub fn find_marker(input: &str, len: usize) -> usize {
    input
        .as_bytes()
        .windows(len)
        .enumerate()
        .find_map(|(i, w)| all_different(w).then(|| i))
        .unwrap()
}

fn all_different(w: &[u8]) -> bool {
    for i in 0 .. w.len() - 1 {
        for j in i + 1 .. w.len() {
            if w[i] == w[j] {
                return false;
            }
        }
    }
    true
}

