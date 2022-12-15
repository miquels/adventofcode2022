use std::fmt;
use std::fs::read_to_string;
use std::mem;
use std::sync::atomic::{Ordering, fence};
use std::time::{Duration, Instant};

mod black_box;
use black_box::black_box;

pub use std::fmt::Write;

#[macro_export]
macro_rules! output {
    ($w:expr, $e:expr) => {
        let _ = write!($w, "{}", $e);
    };
    ($w:expr, $($e:expr),+) => {
        let _ = writeln!($w, $($e),+);
    };
}

#[macro_export]
macro_rules! outputln {
    ($w:expr) => {
        let _ = writeln!($w, "");
    };
    ($w:expr, $e:expr) => {
        let _ = writeln!($w, "{}", $e);
    };
    ($w:expr, $($e:expr),+) => {
        let _ = writeln!($w, $($e),+);
    };
}

#[repr(usize)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Timer {
    Parsing = 0,
    Part1,
    Part2,
    Part12,
    Finishing,
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self {
            Timer::Parsing => "parsing",
            Timer::Part1 => "part1",
            Timer::Part2 => "part2",
            Timer::Part12 => "part1+2",
            Timer::Finishing => "finishing",
        };
        write!(f, "{}", id)
    }
}

// The Runner runs the puzzle of one day.
pub struct Runner<F> {
    input: String,
    dir: String,
    func: F,
    elapsed: Duration,
    bench: bool,
    delayed: bool,
}

impl<F> Runner<F>
where
    F: for<'a, 'b> Fn(&'a mut Ctx<'b>),
{
    // Create a new runner.
    //
    // - `dir` _must_ be in the format `day-XX-name-of-puzzle`
    // - `file` is the input file.
    //
    pub fn new(dir: &str, file: &str, bench: bool, delayed: bool, func: F) -> Runner<F> {
        let file = format!("{}/input/{}", dir, file);
        let input = read_to_string(&file).expect(&file);
        Runner {
            dir: dir.to_string(),
            input,
            func,
            elapsed: Duration::default(),
            bench,
            delayed,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    fn day(&self) -> &'_ str {
        &self.dir[..6]
    }

    // Run the function.
    //
    // The function is passed a `struct Ctx` which is the runners context.
    pub fn run(&mut self) {
        let start = Instant::now();
        let duration = Duration::from_millis(250);
        let min_times = 3;
        let mut times = 0;
        let mut fastest_elapsed = Duration::from_secs(11111);
        let mut fastest_output = Some(String::new());
        let mut output = self.delayed.then(|| String::new());

        loop {
            // Initialize.
            output.as_mut().map(|w| w.clear());
            let writer = output.as_mut().map(|w| w as &mut dyn Write);
            let mut ctx = Ctx::new(self.day(), &self.input, self.bench, writer);
            let _ = writeln!(ctx, "{}", &self.dir[7..]);

            // Run.
            black_box((self.func)(&mut ctx));
            ctx.update_timer(Timer::Finishing);

            // Calculate timers.
            let now = Instant::now();
            let t = now.duration_since(ctx.start);
            if t < fastest_elapsed {
                fastest_elapsed = t;
                if self.delayed {
                    mem::swap(&mut fastest_output, &mut output);
                }
            }
            times += 1;
            if !self.bench || (now.duration_since(start) >= duration && times >= min_times) {
                break;
            }
        }

        if let Some(output) = fastest_output.as_ref() {
            print!("{}", output);
        }
        eprintln!("{}: total: {:?}", self.day(), fastest_elapsed);
        self.elapsed = fastest_elapsed;
    }
}

// `struct Ctx` is passed to the function being executed by the Runner.
//
// You must call `ctx.update_timer(Timer)` after every `parse`, `part1` and `part 2`
//
// (if applicable).
//
// `Ctx` also implements the `Write` trait.
pub struct Ctx<'a> {
    writer: Option<&'a mut dyn Write>,
    input:  Option<&'a str>,
    start:  Instant,
    start_part: Instant,
    timers: [Duration; 5],
    bench: bool,
    name: &'a str,
    nlseen: bool,
}

impl<'a> Ctx<'a> {
    pub const PARSING: Timer = Timer::Parsing;
    pub const PART1: Timer = Timer::Part1;
    pub const PART2: Timer = Timer::Part2;
    pub const PART12: Timer = Timer::Part12;

    fn new(name: &'a str, input: &'a str, bench: bool, writer: Option<&'a mut dyn Write>) -> Ctx<'a> {
        let now = Instant::now();
        Ctx {
            writer,
            name,
            start: now,
            start_part: now,
            input: Some(input),
            timers: [Duration::default(); 5],
            bench,
            nlseen: true,
        }
    }

    // Get the input.
    pub fn input(&mut self) -> &'a str {
        self.input.take().unwrap()
    }

    // Update timer.
    pub fn update_timer(&mut self, timer: Timer) {
        fence(Ordering::SeqCst);
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_part);
        self.start_part = now;
        self.timers[timer as usize] = elapsed;
        let _ = writeln!(self, "{}: {:?}", timer, elapsed);
    }

    pub fn elapsed(&self, timer: Timer) -> Option<Duration> {
        let d = self.timers[timer as usize];
        (!d.is_zero()).then(|| d)
    }
}

impl<'a> Write for Ctx<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut p = String::new();
        for line in s.split_inclusive("\n") {
            if self.nlseen {
                let _ = write!(p, "{}: ", self.name);
            }
            p.push_str(line);
            self.nlseen = s.ends_with("\n");
        }
        if let Some(w) = self.writer.as_mut() {
            w.write_str(&p)
        } else {
            if !self.bench {
                print!("{p}");
            }
            Ok(())
        }
    }
}
