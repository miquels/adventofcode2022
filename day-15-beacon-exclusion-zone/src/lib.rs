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
            set.insert(Range{ start: r.0, end: r.1, x: 0 });
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
                return (r.r[0].end + 1) as u64 * 4000000 + y as u64;
            }
        }
        panic!("find_gap: FAIL");
    }
}

// an inclusive from ..= to range, with a extra payload 'x'.
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
    x: i32,
}

// A set of inclusive ranges.
#[derive(Default, Debug)]
struct Ranges {
    r: Vec<Range>,
}

impl Ranges {
    // Insert a range.
    fn insert(&mut self, range: Range) {
        self.r.push(range);
    }

    // Remove a range of length 1.
    fn remove(&mut self, x: i32) {
        let len = self.r.len();
        for idx in 0 .. len {
            if self.r[idx].start <= x && x<= self.r[idx].end {
                let mut r2 = self.r[idx];
                self.r[idx].end = x - 1;
                r2.start = x + 1;
                self.r.push(r2);
            }
        }
    }

    // Clamp ranges. If a range is outside of low ..= high, it will
    // be made a range of length 0 by setting r.start to r.end + 1.
    // We'll filter those out in merge().
    fn clamp(&mut self, low: i32, high: i32) {
        for r in &mut self.r {
            if r.start < low {
                r.start = low;
            }
            if r.start > high {
                r.end = r.start - 1;
            }
            if r.end < low {
                r.end = r.start - 1;
            }
            if r.end > high {
                r.end = high;
            }
        }
    }

    // Remove empty ranges, then sort.
    fn clean(&mut self) {
        self.r.sort_unstable_by(|a, b| a.start.cmp(&b.start));
        self.r.retain(|r| r.start <= r.end);
    }

    // Merge ranges, remove ranges of length 0.
    fn merge(&mut self) {
        self.clean();
        let mut n1 = 0;
        let mut n2 = 1;
        while n2 < self.r.len() {
            if self.r[n1].end >= self.r[n2].end {
                // skip
                n2 += 1;
            } else if self.r[n1].end >= self.r[n2].start {
                // merge
                self.r[n1].end = self.r[n2].end;
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

    // Sum of length of all ranges.
    // Only useful after merge().
    fn len(&self) -> i32 {
        self.r.iter().fold(0i32, |acc, x| acc + (x.end - x.start) + 1)
    }

    // Number of ranges.
    // Only useful after merge().
    fn num_ranges(&self) -> i32 {
        self.r.len() as i32
    }
}
