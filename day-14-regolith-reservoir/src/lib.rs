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
    //ctx.update_timer(Ctx::PART2);
}

#[derive(Default)]
struct Cave {
    grid: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let mut cave = Cave::default();
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

    // Sand units enter at 500,0.
    // Run the simulation for one unit until the unit comes to rest or exits.
    fn drop_sand(&mut self) -> bool {
        let (mut x, mut y) = (500, 0);
        loop {
            if y + 1 > self.max_y {
                return false;
            }
            if self.grid[y+1][x] == b'.' {
                y += 1;
                continue;
            }
            if x > 0 && self.grid[y+1][x-1] == b'.' {
                y += 1;
                x -= 1;
                continue;
            }
            if x < self.max_x && self.grid[y+1][x+1] == b'.' {
                y += 1;
                x += 1;
                continue;
            }
            if y == 0 {
                panic!("cave full, cannot drop more units of sand");
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
