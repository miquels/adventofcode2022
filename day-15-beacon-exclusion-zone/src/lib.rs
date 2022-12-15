use itertools::Itertools;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let grid = Grid::parse(input);
    ctx.update_timer(Ctx::PARSING);

    outputln!(ctx, "part1: {}", grid.no_beacons_at_y(2000000));
    ctx.update_timer(Ctx::PART1);

    outputln!(ctx, "part2: {}", grid.find_gap_brute_force(4000000));
    ctx.update_timer(Ctx::PART2);
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

    fn range_on_y(&self, y: i32) -> Option<(i32, i32)> {
        let d = self.sy.abs_diff(y) as i32;
        if d > self.area {
            return None;
        }
        let w = self.area - d;
        Some((self.sx - w, self.sx + w))
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

    fn get_raw_ranges_at_y(&self, y: i32, rm_beacons: bool) -> Ranges {
        let mut set = Ranges::default();
        for r in self.sensors.iter().filter_map(|s| s.range_on_y(y)) {
            set.insert(r.0, r.1);
        }
        if rm_beacons {
            for r in self.sensors.iter().filter(|s| s.by == y).map(|s| s.bx) {
                set.remove(r);
            }
        }
        set
    }

    fn no_beacons_at_y(&self, y: i32) -> i32 {
        let mut r = self.get_raw_ranges_at_y(y, true);
        r.merge();
        r.len()
    }

    fn find_gap_brute_force(&self, limit: i32) -> u64 {
        for y in 0 ..= limit {
            let mut r = self.get_raw_ranges_at_y(y, false);
            r.clamp(0, limit);
            r.merge();
            if r.num_ranges() != 1 {
                return (r.r[0].1 + 1) as u64 * 4000000 + y as u64;
            }
        }
        panic!("find_gap: FAIL");
    }
}

#[derive(Default, Debug)]
struct Ranges {
    r: Vec<(i32, i32)>
}

impl Ranges {
    fn insert(&mut self, r1: i32, r2: i32) {
        self.r.push((r1, r2));
    }

    fn remove(&mut self, x: i32) {
        let len = self.r.len();
        for idx in 0 .. len {
            if self.r[idx].0 <= x && x<= self.r[idx].1 {
                let e = self.r[idx].1;
                self.r[idx].1 = x - 1;
                self.r.push((x + 1, e));
            }
        }
    }

    fn clamp(&mut self, low: i32, high: i32) {
        for r in &mut self.r {
            if r.0 < low {
                r.0 = low;
            }
            if r.0 > high {
                r.1 = r.0 - 1;
            }
            if r.1 < low {
                r.1 = r.0 - 1;
            }
            if r.1 > high {
                r.1 = high;
            }
        }
    }

    fn merge(&mut self) {
        self.r.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        self.r.retain(|(x, y)| x <= y);
        if self.r.len() <= 1 {
            return;
        }
        let mut n1 = 0;
        let mut n2 = 1;
        while n2 < self.r.len() {
            if self.r[n1].1 >= self.r[n2].1 {
                // skip
                n2 += 1;
            } else if self.r[n1].1 >= self.r[n2].0 {
                // merge
                self.r[n1].1 = self.r[n2].1;
                n2 += 1;
            } else {
                // advance
                self.r[n1 + 1] = self.r[n2];
                n1 += 1;
                n2 += 1;
            }
        }
        self.r.truncate(n1 + 1);
    }

    fn len(&self) -> i32 {
        self.r.iter().fold(0i32, |acc, x| acc + (x.1 - x.0) + 1)
    }

    fn num_ranges(&self) -> i32 {
        self.r.len() as i32
    }
}
