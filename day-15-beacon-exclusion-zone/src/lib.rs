use std::ops::RangeInclusive;
use btree_range_map::RangeSet;
use itertools::Itertools;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let grid = Grid::parse(input);
    ctx.update_timer(Ctx::PARSING);

    outputln!(ctx, "part1: {}", grid.no_beacons_at_y(2000000));
    ctx.update_timer(Ctx::PART1);
}

#[derive(Debug)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
    area: i32,
}

impl Sensor {
    fn parse(line: &str) -> Sensor {
        let (sx, sy, bx, by) = line
            .split(|c| c == ' ' || c == ':' || c == ',')
            .filter_map(|n| {
                (n.len() > 0 && &n[1..2] == "=").then(|| n[2..].parse::<i32>().unwrap())
            })
            .collect_tuple()
            .unwrap();
        let md = sx.abs_diff(bx) + sy.abs_diff(by);
        Sensor {
            sx,
            sy,
            bx,
            by,
            area: md as i32,
        }
    }

    fn range_on_y(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let d = self.sy.abs_diff(y) as i32;
        if d > self.area {
            return None;
        }
        let w = self.area - d;
        Some(self.sx - w ..= self.sx + w)
    }
}

#[derive(Debug)]
struct Grid {
    sensors: Vec<Sensor>,
}

impl Grid {
    fn parse(line: &str) -> Grid {
        let sensors = line
            .lines()
            .map(|l| Sensor::parse(l))
            .collect();
        Grid { sensors }
    }

    fn no_beacons_at_y(&self, y: i32) -> u32 {
        let mut set = RangeSet::new();
        for r in self.sensors.iter().filter_map(|s| s.range_on_y(y)) {
            set.insert(r);
        }
        for r in self.sensors.iter().filter(|s| s.by == y).map(|s| (s.bx ..= s.bx)) {
            set.remove(r);
        }
        set.len()
    }
}
