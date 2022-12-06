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
    w.into_iter().fold(0u32, |set, v| set | 1 << (v - b'a')).count_ones() == w.len() as u32
}
