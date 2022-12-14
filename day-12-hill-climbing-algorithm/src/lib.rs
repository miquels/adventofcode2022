use itertools::iproduct;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let map = HeightMap::parse(input);
    ctx.update_timer(Ctx::PARSING);

    let steps = map.dijkstra(map.start, true);
    outputln!(ctx, "part1: {}", steps[map.end.y][map.end.x]);
    ctx.update_timer(Ctx::PART1);

    let steps = map.dijkstra(map.end, false);
    let s = iproduct!(0 ..= map.max_y, 0 ..= map.max_x)
        .filter_map(|(y, x)| {
            (map.grid[y][x] == 0).then(|| steps[y][x])
        })
        .min()
        .unwrap();
    outputln!(ctx, "part2: {}", s);
    ctx.update_timer(Ctx::PART2);
}

type Grid = Vec<Vec<u32>>;

struct HeightMap {
    grid: Grid,
    max_x: usize,
    max_y: usize,
    start: Pos,
    end: Pos,
}

impl HeightMap {
    #[inline]
    fn neighbors(&self, p: Pos, up: bool) -> impl Iterator<Item = (Pos, u32)> + '_ {
        let h = self.grid[p.y][p.x];
        let Pos { x, y } = p;
        [
            (y > 0).then(|| (x, y - 1)),
            (x > 0).then(|| (x - 1, y)),
            (x < self.max_x).then(|| (x + 1, y)),
            (y < self.max_y).then(|| (x, y + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(move |(x, y)| if up {
                self.grid[*y][*x] <= h + 1
            } else {
                h <= self.grid[*y][*x]+ 1
            }
        )
        .map(|(x, y)| (Pos { x, y }, 1))
    }

    fn dijkstra(&self, start: Pos, up: bool) -> Grid {
        let mut nodes = (0..=self.max_y)
            .map(|_| {
                let mut row = Vec::new();
                row.resize(self.max_x + 1, u32::MAX);
                row
            })
            .collect::<Vec<_>>();
        nodes[start.y][start.x] = 0;

        let mut visited = (0..=self.max_y)
            .map(|_| {
                let mut row = Vec::new();
                row.resize(self.max_x + 1, false);
                row
            })
            .collect::<Vec<_>>();

        let mut to_visit = std::collections::BinaryHeap::new();
        to_visit.push(PositionSteps {
            pos: start,
            steps: 0,
        });

        while let Some(PositionSteps { pos, steps }) = to_visit.pop() {
            if visited[pos.y][pos.x] {
                continue;
            }
            visited[pos.y][pos.x] = true;

            for (npos, nsteps) in self.neighbors(pos, up) {
                let n = steps + nsteps;
                if n < nodes[npos.y][npos.x] {
                    nodes[npos.y][npos.x] = n;
                    to_visit.push(PositionSteps { pos: npos, steps: n });
                }
            }
        }
        nodes
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
                        (b - b'a') as u32
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

