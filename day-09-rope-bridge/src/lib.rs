use std::collections::HashSet;
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
    curstep: Step,
    visited: HashSet<Pos>,
    knots: [Pos; 10],
    nknots: usize,
}

impl Stepper {
    fn run(&mut self, nknots: usize) {
        // reset.
        self.step_idx = 0;
        self.curstep = self.steps[0].clone();
        self.visited = HashSet::from([Pos::default()]);
        self.knots = [Pos::default(); 10];

        self.nknots = nknots;
        let mut stepping = true;
        while stepping {
            stepping = self.step_head();
            self.step_tails();
        }
    }

    fn new(input: &str) -> Stepper {
        let mut steps = Vec::new();
        for line in input.lines() {
            let (s, a) = line.split_once(' ').unwrap();
            let a = a.parse::<i32>().unwrap();
            steps.push(match s {
                "L" => Step::L(a),
                "R" => Step::R(a),
                "U" => Step::U(a),
                "D" => Step::D(a),
                x => panic!("unknown step {}", x),
            });
        }
        Stepper {
            curstep: steps[0].clone(),
            visited: HashSet::from([Pos::default()]),
            steps,
            step_idx: 0,
            knots: [Pos::default(); 10],
            nknots: 2,
        }
    }

    fn step_head(&mut self) -> bool {
        let a = match &mut self.curstep {
            Step::L(a) => { self.knots[0].x -= 1; a },
            Step::R(a) => { self.knots[0].x += 1; a },
            Step::U(a) => { self.knots[0].y += 1; a },
            Step::D(a) => { self.knots[0].y -= 1; a },
        };
        *a -= 1;
        if *a == 0 {
            self.step_idx += 1;
            if self.step_idx == self.steps.len() {
                return false;
            }
            self.curstep = self.steps[self.step_idx].clone();
        }
        true
    }

    fn step_tails(&mut self) {
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
