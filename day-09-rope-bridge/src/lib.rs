use std::collections::HashSet;

pub fn part1(input: &str) {
    let mut stepper = Stepper::new(input);
    stepper.run();
    println!("part1: {}", stepper.visited.len());
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
    head: Pos,
    tail: Pos,
}

impl Stepper {
    fn run(&mut self) {
        let mut stepping = true;
        while stepping {
            stepping = self.step_head();
            self.step_tail();
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
            head: Pos::default(),
            tail: Pos::default(),
        }
    }

    fn step_head(&mut self) -> bool {
        let a = match &mut self.curstep {
            Step::L(a) => { self.head.x -= 1; a },
            Step::R(a) => { self.head.x += 1; a },
            Step::U(a) => { self.head.y += 1; a },
            Step::D(a) => { self.head.y -= 1; a },
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

    fn step_tail(&mut self) {
        if !self.tail.near(&self.head) {
            if self.tail.y > self.head.y {
                self.tail.y -= 1;
            } else if self.tail.y < self.head.y {
                self.tail.y += 1;
            }
            if self.tail.x > self.head.x {
                self.tail.x -= 1;
            } else if self.tail.x < self.head.x {
                self.tail.x += 1;
            }
        }
        self.visited.insert(self.tail);
    }
}
