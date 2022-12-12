
pub fn part1(input: &str) {
    let map = HeightMap::parse(input);
    let res = map.dijkstra();
    println!("part1: {}", res);
}

#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

struct PositionSteps {
    steps: u32,
    pos: Pos,
}

impl Ord for PositionSteps {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for PositionSteps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PositionSteps {
    fn eq(&self, other: &Self) -> bool {
        self.steps.eq(&other.steps)
    }
}

impl Eq for PositionSteps {}

struct HeightMap {
    grid: Vec<Vec<u16>>,
    max_x: usize,
    max_y: usize,
    start: Pos,
    end: Pos,
}

impl HeightMap {
    #[inline]
    fn neighbors(&self, p: Pos) -> impl Iterator<Item = (Pos, u16)> + '_ {
        let maxh = self.grid[p.y][p.x] + 1;
        let Pos { x, y } = p;
        [
            (y > 0 && self.grid[y-1][x] <= maxh).then(|| (x, y - 1)),
            (x > 0 && self.grid[y][x-1] <= maxh).then(|| (x - 1, y)),
            (x < self.max_x && self.grid[y][x+1] <= maxh).then(|| (x + 1, y)),
            (y < self.max_y && self.grid[y+1][x] <= maxh).then(|| (x, y + 1)),
        ]
        .into_iter()
        .flatten()
        .map(move |(x, y)| (Pos { x, y }, 1))
    }

    fn dijkstra(&self) -> u16 {
        let mut nodes = (0..=self.max_y)
            .map(|_| {
                let mut row = Vec::new();
                row.resize(self.max_x + 1, i32::MAX as u32);
                row
            })
            .collect::<Vec<_>>();
        nodes[self.start.y][self.start.x] = 0;

        let mut to_visit = std::collections::BinaryHeap::new();
        to_visit.push(PositionSteps {
            pos: self.start,
            steps: 0,
        });

        while let Some(PositionSteps { pos, steps }) = to_visit.pop() {
            if (nodes[pos.y][pos.x] & 0x80000000) > 0 {
                continue;
            }
            nodes[pos.y][pos.x] |= 0x80000000;

            for (npos, nsteps) in self.neighbors(pos) {
                let n = steps + nsteps as u32;
                if n < (nodes[npos.y][npos.x] & 0x7fffffff) {
                    nodes[npos.y][npos.x] &= 0x80000000;
                    nodes[npos.y][npos.x] |= n;
                    to_visit.push(PositionSteps { pos: npos, steps: n });
                }
            }
        }
        (nodes[self.end.y][self.end.x] & 0x7fffffff) as u16
    }

    fn parse(input: &str) -> HeightMap {
        let mut start = Pos::default();
        let mut end = Pos::default();
        let g = input
            .lines()
            .enumerate()
            .map(|(y, line)| line
                .bytes()
                .enumerate()
                .map(|(x, b)|
                    if b == b'S' {
                        start = Pos { x, y };
                        0
                    } else if b == b'E' {
                        end = Pos { x, y };
                        25
                    } else {
                        (b - b'a') as u16
                    })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        HeightMap {
            max_x: g[0].len() - 1,
            max_y: g.len() - 1,
            grid: g,
            start,
            end,
        }
    }
}
