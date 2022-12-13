use std::time::Duration;
use clap::Parser;
use runner::Runner;

const DAYS: &'static [(u32, &str, fn(&mut runner::Ctx))] = &[
    ( 1, "day-01-calorie-counting", day_01::start ),
    ( 13, "day-13-distress-signal", day_13::start ),
    /*
    ( 2, "day-02-rock-paper-scissors", 2, day_02::part1a, day_02::part2a ),
    ( 3, "day-03-rucksack-reorganization", 2, day_03::part1, day_03::part2 ),
    ( 4, "day-04-camp-cleanup", 1, day_04::part1_2, day_04::part1_2 ),
    ( 5, "day-05-supply-stacks", 1, day_05::part1_2, day_05::part1_2 ),
    ( 6, "day-06-tuning-trouble", 2, day_06::part1, day_06::part2 ),
    ( 7, "day-07-no-space-left-on-device", 1, day_07::part1_2_notree, day_07::part1_2 ),
    ( 8, "day-08-treetop-tree-house", 2, day_08::part1, day_08::part2 ),
    ( 9, "day-09-rope-bridge", 1, day_09::part1_2, day_09::part1_2 ),
    ( 10, "day-10-cathode-ray-tube", 1, day_10::part1_2, day_10::part1_2 ),
    ( 11, "day-11-monkey-in-the-middle", 1, day_11::part1_2, day_11::part1_2 ),
    ( 12, "day-12-hill-climbing-algorithm", 1, day_12::part1, day_12::part1 ),
    ( 105, "day-05-supply-stacks", 1, day_05::part1_2_heavy_duty, day_05::part1_2 ),
    */
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
