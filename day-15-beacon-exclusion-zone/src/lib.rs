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

    fn get_ranges_at_y(&self, y: i32) -> Ranges {
        let mut ranges = Ranges::default();
        for r in self.sensors.iter().filter_map(|s| s.range_on_y(y)) {
            ranges.insert(Range{ start: r.0, end: r.1 });
        }
        ranges
    }

    fn remove_beacons_from_ranges(&self, ranges: &mut Ranges, y: i32) {
        for r in self.sensors.iter().filter(|s| s.by == y).map(|s| s.bx) {
            ranges.remove(r);
        }
    }

    fn no_beacons_at_y(&self, y: i32) -> i32 {
        let mut r = self.get_ranges_at_y(y);
        self.remove_beacons_from_ranges(&mut r, y);
        r.sort();
        r.len()
    }

    fn find_gap_brute_force(&self, limit: i32) -> u64 {
        for y in 0 ..= limit {
            let mut r = self.get_ranges_at_y(y);
            r.clamp(0, limit);
            r.sort();
            if let Some(pos) = r.find_gap_pos() {
                return pos as u64 * 4000000 + y as u64;
            }
        }
        panic!("find_gap: FAIL");
    }
}

// an inclusive from ..= to range
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
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
            if r.end > high {
                r.end = high;
            }
        }
    }

    // Remove empty ranges, then sort.
    fn sort(&mut self) {
        self.r.sort_unstable_by(|a, b| a.start.cmp(&b.start));
        self.r.retain(|r| r.start <= r.end);
    }

    // Sum of length of all ranges. Parts that overlap are not counted twice.
    // - Only useful after clean().
    fn len(&self) -> i32 {
        if self.r.len() == 0 {
            return 0;
        }
        let mut len = 0;
        let mut start = self.r[0].start;
        for r in &self.r {
            if r.start > start {
                len += r.end - r.start + 1;
                start = r.end + 1;
            } else if r.end >= start {
                len += r.end - start + 1;
                start = r.end + 1;
            }
        }
        len
    }

    // See if there is a gap somewhere.
    fn find_gap_pos(&self) -> Option<i32> {
        if self.r.len() == 0 {
            return None;
        }
        let mut start = self.r[0].start;
        for r in &self.r {
            if r.start > start {
                return Some(start);
            } else if r.end >= start {
                start = r.end + 1;
            }
        }
        None
    }
}
