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

#[derive(Clone)]
enum Step {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn near(&self, other: &Pos) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

struct Stepper {
    steps: Vec<Step>,
    step_idx: usize,
    visited: CoordHashSet,
    knots: [Pos; 10],
    nknots: usize,
}

impl Stepper {
    fn run(&mut self, nknots: usize) {
        // reset.
        self.step_idx = 0;
        self.visited.clear();
        self.visited.insert(Pos::default());
        self.knots = [Pos::default(); 10];

        self.nknots = nknots;
        let mut done = false;
        while !done {
            let steps;
            (steps, done) = self.step_head();
            self.step_tails(steps);
        }
    }

    fn new(input: &str) -> Stepper {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        let mut steps = Vec::new();
        let mut pos = Pos::default();
        for line in input.lines() {
            let (s, a) = line.split_once(' ').unwrap();
            let a = a.parse::<i32>().unwrap();
            steps.push(match s {
                "L" => {
                    pos.x -= a;
                    Step::L(a)
                },
                "R" => {
                    pos.x += a;
                    Step::R(a)
                },
                "U" => {
                    pos.y += a;
                    Step::U(a)
                },
                "D" => {
                    pos.y -= a;
                    Step::D(a)
                },
                x => panic!("unknown step {}", x),
            });
            if pos.x < min_x { min_x = pos.x }
            if pos.y < min_y { min_y = pos.y }
            if pos.x > max_x { max_x = pos.x }
            if pos.y > max_y { max_y = pos.y }
        }
        Stepper {
            steps,
            step_idx: 0,
            visited: CoordHashSet::new(min_x, min_y, max_x, max_y),
            knots: [Pos::default(); 10],
            nknots: 2,
        }
    }

    fn step_head(&mut self) -> (i32, bool) {
        let a = match self.steps[self.step_idx] {
            Step::L(a) => { self.knots[0].x -= a; a },
            Step::R(a) => { self.knots[0].x += a; a },
            Step::U(a) => { self.knots[0].y += a; a },
            Step::D(a) => { self.knots[0].y -= a; a },
        };
        self.step_idx += 1;
        if self.step_idx == self.steps.len() {
            (a, true)
        } else {
            (a, false)
        }
    }

    fn step_tails(&mut self, nsteps: i32) {
        for _ in 0 .. nsteps {
            for i in 1 .. self.nknots {
                let head = &self.knots[i - 1];
                let mut tail = self.knots[i];
                if !tail.near(&head) {
                    if tail.y > head.y {
                        tail.y -= 1;
                    } else if tail.y < head.y {
                        tail.y += 1;
                    }
                    if tail.x > head.x {
                        tail.x -= 1;
                    } else if tail.x < head.x {
                        tail.x += 1;
                    }
                    self.knots[i] = tail;
                }
            }
            self.visited.insert(self.knots[self.nknots-1]);
        }
    }
}

struct CoordHashSet {
    min_x:  i32,
    min_y:  i32,
    len:    i32,
    set:    Vec<Vec<u64>>,
}

impl CoordHashSet {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> CoordHashSet {
        let set = (min_y .. max_y).map(|_| {
            let mut v = Vec::new();
            v.resize(((max_x - min_x) / 64) as usize + 1, 0);
            v
        }).collect();
        CoordHashSet {
            min_x,
            min_y,
            len: 0,
            set,
        }
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn insert(&mut self, pos: Pos) -> bool {
        let y_idx = (pos.y - self.min_y) as usize;
        let x_idx = (pos.x - self.min_x) as usize / 64;
        let x_bit = 1 << ((pos.x - self.min_x) as u32 & 63);
        let v = &mut self.set[y_idx];
        let v = v.get_mut(x_idx).unwrap();
        if (*v & x_bit) == x_bit {
            return true;
        }
        *v |= x_bit;
        self.len += 1;
        false
    }

    fn clear(&mut self) {
        self.set.iter_mut().for_each(|x| x.fill(0));
        self.len = 0;
    }

}

