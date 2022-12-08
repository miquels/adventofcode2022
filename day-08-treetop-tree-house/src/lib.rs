pub fn part1(input: &str) {
    let mut trees = Trees::<99>::new(input);
    println!("part1: {}", trees.count_visible());
}

pub fn part2(input: &str) {
    let trees = Trees::<99>::new(input);
    println!("part2: {}", trees.count_scenery());
}

struct Trees<'i, const DIM: usize> {
    seen: [[bool; DIM]; DIM],
    input: &'i [u8],
}

impl<'i, const DIM: usize> Trees<'i, DIM> {
    // part1.
    fn count_visible(&mut self) -> i32 {
        let mut visible = 0;
        for p in 0..self.dim() {
            visible += self.look_along(p, 0, 0, 1);
            visible += self.look_along(p, self.dim() - 1, 0, -1);
            visible += self.look_along(0, p, 1, 0);
            visible += self.look_along(self.dim() - 1, p, -1, 0);
        }
        visible
    }

    fn look_along(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
        let mut highest = 0;
        let mut visible = 0;
        for _ in 0 .. self.dim() {
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
    fn count_scenery(&self) -> i32 {
        (1 .. self.dim() - 1).map(|y| {
            (1 .. self.dim() - 1).map(move |x| self.look_around(x, y))
        })
        .flatten()
        .max()
        .unwrap()
    }

    fn look_around(&self, x: i32, y: i32) -> i32 {
        let (h, mut scenic) = (self.height(x, y), 1);
        for (dx, dy, max) in [
            (0, -1, y + 1),
            (-1, 0, x + 1),
            (1, 0, self.dim() - x),
            (0, 1, self.dim() - y)
        ] {
            let (mut x, mut y, mut i) = (x + dx, y + dy, 1);
            while i < max && self.height(x, y) < h {
                x += dx;
                y += dy;
                i += 1;
            }
            scenic *= std::cmp::max(1, i - (i == max) as i32);
        }
        scenic
    }

    fn height(&self, x: i32, y: i32) -> u8 {
        self.input[(y*(self.dim() + 1) + x) as usize]
    }

    fn new(input: &str) -> Trees<DIM> {
        let input = input.as_bytes();
        let trees = Trees {
            seen: [[false; DIM]; DIM],
            input,
        };
        trees
    }

    const fn dim(&self) -> i32 {
        DIM as i32
    }
}
