use std::mem;
use itertools::Itertools;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let mut cave = Cave::parse(ctx);
    cave.ctx.update_timer(Ctx::PARSING);

    cave.drop_sand(500, 0);
    outputln!(cave.ctx, "part2: units: {}", cave.units);
    cave.ctx.update_timer(Ctx::PART2);
}

struct Cave<'a, 'b> {
    grid: Vec<Vec<u8>>,
    max_y: usize,
    part: usize,
    units: u32,
    ctx: &'a mut Ctx<'b>,
}

impl<'a, 'b> Cave<'a, 'b> where 'b: 'a {
    // Run recursively.
    fn drop_sand(&mut self, x: usize, y: usize) {
        if self.elem_get(x, y) != b'.' {
            return;
        }

        if self.part == 1 && y == self.max_y + 1 {
            outputln!(self.ctx, "part1: units: {}", self.units);
            self.ctx.update_timer(Ctx::PART1);
            self.max_y += 2;
            self.part = 2;
        }

        self.drop_sand(x, y + 1);
        if x > 0 {
            self.drop_sand(x - 1, y + 1);
        }
        self.drop_sand(x + 1, y + 1);

        self.elem_set(x, y, b'o');
        self.units += 1;
    }

    fn elem_get(&mut self, x: usize, y: usize) -> u8 {
        if self.part == 2 && y == self.max_y {
            return b'#';
        }
        if y >= self.grid.len() || x >= self.grid[y].len() {
            return b'.';
        }
        self.grid[y][x]
    }

    fn elem_set(&mut self, x: usize, y: usize, val: u8) {
        if self.grid.len() <= y {
            self.grid.resize(y + 1, Vec::new());
        }
        if self.grid[y].len() <= x {
            self.grid[y].resize(x + 1, b'.');
        }
        self.grid[y][x] = val;
    }

    fn parse(ctx: &'a mut Ctx<'b>) -> Cave<'a, 'b> {
        let input = ctx.input();
        let mut cave = Cave {
            grid: Vec::new(),
            max_y: 0,
            part: 1,
            units: 0,
            ctx,
        };
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
        cave.max_y = cave.grid.len() - 1;
        cave
    }

    fn draw(&mut self, mut from: (usize, usize), mut to: (usize, usize)) {
        if from.0 > to.0 || from.1 > to.1 {
            mem::swap(&mut from, &mut to);
        }
        for y in from.1 ..= to.1 {
            for x in from.0 ..= to.0 {
                self.elem_set(x, y, b'#');
            }
        }
    }
}
