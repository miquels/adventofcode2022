use std::mem;
use itertools::Itertools;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let mut cave = Cave::parse(ctx.input());
    ctx.update_timer(Ctx::PARSING);

    let mut units = 0u32;
    while cave.drop_sand() {
        units += 1;
    }
    outputln!(ctx, "part1: units: {}", units);
    ctx.update_timer(Ctx::PART1);

    cave.init_part2();
    let mut units = 0u32;
    while cave.drop_sand() {
        units += 1;
    }
    outputln!(ctx, "part2: units: {}", units);
    ctx.update_timer(Ctx::PART2);
}

#[derive(Default)]
struct Cave {
    grid: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
    part: usize,
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let mut cave = Cave::default();
        cave.part = 1;
        input
            .trim()
            .split('\n')
            .for_each(|line| {
                line
                    .split(' ')
                    .step_by(2)
                    .map(|coords| {
                        let c = coords.split_once(',').unwrap();
                        (c.0.parse::<usize>().unwrap(), c.1.parse::<usize>().unwrap())
                    })
                    .tuple_windows::<(_, _)>()
                    .for_each(|(from, to)| cave.draw(from, to));
            });

        for y in 0 ..= cave.max_y {
            cave.grid[y].resize(cave.max_x + 1, b'.');
        }

        cave
    }

    fn init_part2(&mut self) {
        self
            .grid
            .iter_mut()
            .for_each(|y| {
                y.iter_mut().for_each(|x| if *x == b'o' { *x = b'.' });
            });
        let mut l1 = Vec::new();
        l1.resize(self.max_x + 1, b'.');
        self.grid.push(l1);
        self.max_y += 2;
        self.part = 2;
    }

    fn elem_at(&mut self, x: usize, y: usize) -> u8 {
        if self.part == 2 && y == self.max_y {
            return b'#';
        }
        if x > self.max_x {
            if self.part == 1 {
                return b'#';
            }
            if self.grid[y].len() <= x {
                self.grid[y].resize(x + 1, b'.');
            }
        }
        self.grid[y][x]
    }

    // Sand units enter at 500,0.
    // Run the simulation for one unit until the unit comes to rest or exits.
    fn drop_sand(&mut self) -> bool {
        let (mut x, mut y) = (500, 0);
        loop {
            if self.part == 1 && y == self.max_y {
                return false;
            }
            if self.elem_at(x, y + 1) == b'.' {
                y += 1;
                continue;
            }
            if x > 0 && self.elem_at(x-1, y+1) == b'.' {
                y += 1;
                x -= 1;
                continue;
            }
            if self.elem_at(x+1, y+1) == b'.' {
                y += 1;
                x += 1;
                continue;
            }
            if y == 0 && self.grid[y][x] != b'.' {
                if self.part == 1 {
                    panic!("cave full, cannot drop more units of sand");
                } else {
                    return false;
                }
            }
            self.grid[y][x] = b'o';
            break;
        }
        true
    }

    fn draw(&mut self, mut from: (usize, usize), mut to: (usize, usize)) {
        if from.0 > to.0 || from.1 > to.1 {
            mem::swap(&mut from, &mut to);
        }
        if to.0 > self.max_x {
            self.max_x = to.0;
        }
        if to.1 > self.max_y {
            self.max_y = to.1;
        }
        if self.grid.len() <= to.1 {
            self.grid.resize(to.1 + 1, Vec::new());
        }
        for y in from.1 ..= to.1 {
            if self.grid[y].len() <= to.0 {
                self.grid[y].resize(to.0 + 1, b'.');
            }
            self.grid[y][from.0 ..= to.0].fill(b'#');
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for y in 0 .. self.grid.len() {
            let s = self.grid[y].iter().map(|&c| c as char).collect::<String>();
            println!("{}", s);
        }
    }
}
