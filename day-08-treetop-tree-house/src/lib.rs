pub fn part1(input: &str) {
    let mut trees = Trees::new(input);
    println!("part1: {}", trees.count_visible());
}

pub fn part2(input: &str) {
    let trees = Trees::new(input);
    println!("part2: {}", trees.count_scenery());
}

struct Trees<'i> {
    seen: [[bool; 99]; 99],
    dim: i32,
    input: &'i [u8],
}

impl<'i> Trees<'i> {
    // part1.
    fn count_visible(&mut self) -> u32 {
        let mut visible = 0;
        for p in 0..self.dim {
            visible += self.look_along(p, 0, 0, 1);
            visible += self.look_along(p, self.dim - 1, 0, -1);
            visible += self.look_along(0, p, 1, 0);
            visible += self.look_along(self.dim - 1, p, -1, 0);
        }
        visible
    }

    fn look_along(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32) -> u32 {
        let mut highest = 0;
        let mut visible = 0;
        for _ in 0 .. self.dim {
            let t = self.height(x, y);
            if t > highest {
                highest = t;
                if !self.seen(x, y) {
                    visible += 1;
                }
                if highest == b'9' {
                    break;
                }
            }
            x += dx;
            y += dy;
        }
        visible
    }

    fn seen(&mut self, x: i32, y: i32) -> bool {
        if self.seen[y as usize][x as usize] {
            true
        } else {
            self.seen[y as usize][x as usize] = true;
            false
        }
    }

    // part2
    fn count_scenery(&self) -> u32 {
        let mut scenic = 0;
        for y in 1 .. self.dim - 1 {
            for x in 1 .. self.dim - 1 {
                scenic = std::cmp::max(self.look_around(x, y), scenic);
            }
        }
        scenic
    }

    fn look_around(&self, mut x: i32, mut y: i32) -> u32 {
        let mut scenic = 1;
        let (ox, oy, t) = (x, y, self.height(x, y));
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            (x, y) = (ox + dx, oy + dy);
            for i in 1 .. self.dim as u32 {
                if x < 0 || x >= self.dim || y < 0 || y >= self.dim {
                    scenic *= std::cmp::max(1, i - 1);
                    break;
                }
                if self.height(x, y) >= t {
                    scenic *= i;
                    break;
                }
                x += dx;
                y += dy;
            }
        }
        scenic
    }

    fn height(&self, x: i32, y: i32) -> u8 {
        self.input[(y*(self.dim + 1) + x) as usize]
    }

    fn new(input: &str) -> Trees {
        let input = input.as_bytes();
        let dim = input.iter().position(|&b| b == b'\n').unwrap() as i32;
        let trees = Trees {
            seen: [[false; 99]; 99],
            dim,
            input,
        };
        trees
    }
}
