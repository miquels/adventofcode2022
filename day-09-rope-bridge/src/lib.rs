use std::time::Instant;

pub fn part1_2(input: &str) {
    let now = Instant::now();
    let mut stepper = Stepper::new(input);
    println!("parsing: {:?}", now.elapsed());

    let now = Instant::now();
    stepper.run(2);
    println!("part1: {}", stepper.visited.len());
    println!("part1: {:?}", now.elapsed());

    let now = Instant::now();
    stepper.run(10);
    println!("part2: {}", stepper.visited.len());
    println!("part2: {:?}", now.elapsed());
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn near(&self, other: &Coord) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

struct Stepper {
    steps: Vec<(i32, i32)>,
    step_idx: usize,
    visited: CoordSet,
    knots: [Coord; 10],
    nknots: usize,
}

impl Stepper {
    fn run(&mut self, nknots: usize) {
        self.step_idx = 0;
        self.visited.clear();
        self.knots.fill(Coord::default());
        self.nknots = nknots;

        while {
            let done = self.step_head();
            self.step_tails();
            !done
        } {}
    }

    fn step_head(&mut self) -> bool {
        let (dx, dy) = self.steps[self.step_idx];
        self.knots[0].x += dx;
        self.knots[0].y += dy;
        self.step_idx += 1;
        self.step_idx == self.steps.len()
    }

    fn step_tails(&mut self) {
        while {
            let mut stepped = false;
            for i in 1 .. self.nknots {
                let head = &self.knots[i - 1];
                let mut tail = self.knots[i];
                if !tail.near(&head) {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                    self.knots[i] = tail;
                    stepped = true;
                }
            }
            self.visited.insert(self.knots[self.nknots-1]);
            stepped
        } {}
    }

    fn new(input: &str) -> Stepper {
        let mut steps = Vec::new();
        let mut visited = CoordSet::default();
        for line in input.lines() {
            let (s, a) = line.split_once(' ').unwrap();
            let a = a.parse::<i32>().unwrap();
            let (dx, dy) = match s {
                "L" => (-a, 0),
                "R" => (a, 0),
                "U" => (0, -a),
                "D" => (0, a),
                x => panic!("unknown step {}", x),
            };
            steps.push((dx, dy));
            visited.expand(dx, dy);
        }
        Stepper {
            steps,
            visited,
            step_idx: 0,
            knots: [Coord::default(); 10],
            nknots: 2,
        }
    }
}

// This is a _lot_ faster than HashSet<(i32, i32)>.
#[derive(Default)]
struct CoordSet {
    min_x:  i32,
    min_y:  i32,
    max_x:  i32,
    max_y:  i32,
    coord:  Coord,
    len:    i32,
    set:    Vec<Vec<u64>>,
}

impl CoordSet {
    fn insert(&mut self, coord: Coord) -> bool {
        let y_idx = (coord.y - self.min_y) as usize;
        let x_idx = (coord.x - self.min_x) as usize / 64;
        let x_bit = 1u64 << ((coord.x - self.min_x) as u32 & 63);
        let v = self.set[y_idx].get_mut(x_idx).unwrap();
        if (*v & x_bit) == x_bit {
            return true;
        }
        *v |= x_bit;
        self.len += 1;
        false
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn clear(&mut self) {
        let xz = ((self.max_x - self.min_x) / 64) as usize + 1;
        self.set.resize((self.max_y - self.min_x) as usize, Vec::new());
        self.set.iter_mut().for_each(|x| { x.truncate(0); x.resize(xz, 0) });
        self.len = 0;
    }

    fn expand(&mut self, dx: i32, dy: i32) {
        self.coord.x += dx;
        self.coord.y += dy;
        if self.coord.x < self.min_x { self.min_x = self.coord.x }
        if self.coord.y < self.min_y { self.min_y = self.coord.y }
        if self.coord.x > self.max_x { self.max_x = self.coord.x }
        if self.coord.y > self.max_y { self.max_y = self.coord.y }
    }
}

