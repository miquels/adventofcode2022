use itertools::Itertools;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let grid = Grid::parse(input);
    ctx.update_timer(Ctx::PARSING);

    outputln!(ctx, "part1: {}", grid.no_beacons_at_y(2000000));
    ctx.update_timer(Ctx::PART1);

    outputln!(ctx, "part2: {}", grid.find_gap_smart(4000000));
    ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
    dist: i32,
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
            dist: md as i32,
        }
    }

    fn range_on_y(&self, y: i32) -> Option<(i32, i32)> {
        let d = self.sy.abs_diff(y) as i32;
        if d > self.dist {
            return None;
        }
        let w = self.dist - d;
        Some((self.sx - w, self.sx + w))
    }

    // The manhattan distance from the center forms a rectangle around
    // the sensor. This method calculates the Line for every edge.
    // Edge 0 is in the upper left quadrant, rest follows clockwise.
    fn edge(&self, edge: u32) -> Line {
        let (sx, sy, d) = (self.sx, self.sy, self.dist);
        match edge {
            0 => Line { x1: sx-d, y1: sy, x2: sx, y2: sy-d },
            1 => Line { x1: sx, y1: sy-d, x2: sx + d, y2: sy },
            2 => Line { x1: sx, y1: sy+d, x2: sx + d, y2: sy },
            3 => Line { x1: sx-d, y1: sy, x2: sx, y2: sy+d },
            _ => panic!("impossible edge"),
        }
    }

    // The points where two edges of the rectangles around the sensors intersect.
    fn edges_intersect(&self, other: &Sensor) -> impl Iterator<Item=(i32, i32)> {
        [
            self.edge(0).intersect(&other.edge(1)),
            self.edge(0).intersect(&other.edge(3)),
            self.edge(1).intersect(&other.edge(0)),
            self.edge(1).intersect(&other.edge(2)),
            self.edge(2).intersect(&other.edge(1)),
            self.edge(2).intersect(&other.edge(3)),
            self.edge(3).intersect(&other.edge(0)),
            self.edge(3).intersect(&other.edge(2)),
        ]
            .into_iter()
            .flatten()
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

    fn find_gap_on_y(&self, y: i32, limit: i32) -> Option<i32> {
        let mut r = self.get_ranges_at_y(y);
        r.clamp(0, limit);
        r.sort();
        r.find_gap_pos()
    }

    #[allow(dead_code)]
    fn find_gap_brute_force(&self, limit: i32) -> u64 {
        for y in 0 ..= limit {
            if let Some(x) = self.find_gap_on_y(y, limit) {
                return x as u64 * 4000000 + y as u64;
            }
        }
        panic!("find_gap: FAIL");
    }

    fn find_gap_smart(&self, limit: i32) -> u64 {
        let ys = self
            .edge_intersections()
            .into_iter()
            .map(|(_, y)| (y-1 ..= y+1).into_iter())
            .flatten();
        for y in ys {
            if let Some(x) = self.find_gap_on_y(y, limit) {
                return x as u64 * 4000000 + y as u64;
            }
        }
        panic!("find_gap: FAIL");
    }

    fn edge_intersections(&self) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for i in 0 .. self.sensors.len() {
            for j in i .. self.sensors.len() {
                v.extend(self.sensors[i].edges_intersect(&self.sensors[j]));
            }
        }
        v
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

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

// Do ranges l1 ..= r2 and l2 ..= r2 intersect?
fn range_overlap(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
    let (mut l1, mut r1, mut l2, mut r2) = (l1, r1, l2, r2);

    // normalize.
    if l1 > r1 {
        (l1, r1) = (r1, l1);
    }
    if l2 > r2 {
        (l2, r2) = (r2, l2);
    }

    // compare.
    l1 <= r2 && l2 <= r1
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<(i32, i32)> {
        let (mut x1, mut y1, mut x2, mut y2) = (self.x1, self.y1, other.x1, other.y1);

        // They must overlap somewehere.
        if !range_overlap(self.y1, self.y2, other.y1, other.y2) ||
           !range_overlap(self.x1, self.x2, other.x1, other.x2) {
               return None;
        }

        // Align the x-axises by moving on of them to the left to
        // match the other one. Extend the y-axis accordingly.
        let d = x1.abs_diff(x2) as i32;
        if x1 < x2 {
            x2 -= d;
            if y1 < y2 {
                y2 += d;
            } else {
                y2 -= d;
            }
        }
        if x1 > x2 {
            x1 -= d;
            if y2 < y1 {
                y1 += d;
            } else {
                y1 -= d;
            }
        }

        // Now that x is the same and we know y1 and y2, we can
        // calculate where the lines intersect.
        if y1 > y2 {
            let w = (y1 - y2) / 2;
            Some((x1 + w, y2 + w))
        } else {
            let w = (y2 - y1) / 2;
            Some((x1 + w, y1 + w))
        }
    }
}
