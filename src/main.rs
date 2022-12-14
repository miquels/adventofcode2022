use std::time::Duration;
use clap::Parser;
use runner::Runner;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const DAYS: &'static [(u32, &str, fn(&mut runner::Ctx))] = &[
    ( 1, "day-01-calorie-counting", day_01::start ),
    ( 2, "day-02-rock-paper-scissors", day_02::start ),
    ( 3, "day-03-rucksack-reorganization", day_03::start ),
    ( 4, "day-04-camp-cleanup", day_04::start ),
    ( 5, "day-05-supply-stacks", day_05::start ),
    ( 6, "day-06-tuning-trouble", day_06::start ),
    ( 7, "day-07-no-space-left-on-device", day_07::start_notree ),
    ( 8, "day-08-treetop-tree-house", day_08::start ),
    ( 9, "day-09-rope-bridge", day_09::start ),
    ( 10, "day-10-cathode-ray-tube", day_10::start ),
    ( 11, "day-11-monkey-in-the-middle", day_11::start ),
    ( 12, "day-12-hill-climbing-algorithm", day_12::start ),
    ( 13, "day-13-distress-signal", day_13::start ),
    ( 14, "day-14-regolith-reservoir", day_14::start ),
    ( 105, "day-05-supply-stacks", day_05::start_heavy_duty ),
];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Day to run, or "all"  for all days.
    #[arg(long, value_parser = day_parser)]
    day: u32,
    /// Part to run, if not set both parts are run.
    #[arg(long, value_parser = part_parser)]
    part: Option<u32>,
    /// Input file to use
    #[arg(long, default_value_t = String::from("input.txt"))]
    input: String,
    /// Benchmark.
    #[arg(long)]
    bench: bool,
    /// Buffer output instead of printing to stdout right away
    #[arg(long)]
    buffered: bool,
}

fn day_parser(s: &str) -> Result<u32, String> {
    if s == "all" {
        return Ok(0);
    }
    let day = s.parse::<u32>().map_err(|e| format!("{}", e))?;
    if day < 1 {
        return Err(format!("invalid day: {}", s));
    }
    Ok(day)
}

fn part_parser(s: &str) -> Result<u32, String> {
    let part = s.parse::<u32>().map_err(|e| format!("{}", e))?;
    if part < 1 || part > 2 {
        return Err(format!("invalid part: {}", s));
    }
    Ok(part)
}

fn main() {
    let opts = Options::parse();
    let mut tot_elapsed = Duration::from_secs(0);

    for (day, dir, func) in DAYS {
        if (opts.day == 0 && *day < 100) || opts.day == *day {
            let mut runner = Runner::new(dir, &opts.input, opts.bench, opts.buffered, func);
            runner.run();
            tot_elapsed += runner.elapsed();
        }
    }

    if opts.day == 0 {
        eprintln!("\nTotal time: {:?}", tot_elapsed);
    }
}
