use std::fs::read_to_string;
use std::time::{Duration, Instant};
use clap::Parser;

const DAYS: &'static [(u32, &str, fn(&str), fn(&str))] = &[
    ( 1, "day-01-calorie-counting", day_01::part1, day_01::part2 ),
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
}

fn day_parser(s: &str) -> Result<u32, String> {
    if s == "all" {
        return Ok(0);
    }
    let day = s.parse::<u32>().map_err(|e| format!("{}", e))?;
    if day < 1 || day > 25 {
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

    for (day, dir, part1, part2) in DAYS {
        if opts.day == 0 || opts.day == *day {
            eprintln!("{}: {}", &dir[..6], &dir[7..]);
            let file = format!("{}/input/{}", dir, opts.input);
            let input = read_to_string(&file).expect(&file);
            for part in 1..=2 {
                if opts.part.is_none() || opts.part == Some(part) {
                    eprintln!("part{}: == start ==", part);
                    let start = Instant::now();
                    if part == 1 { part1(&input) } else { part2(&input) }
                    let elapsed = start.elapsed();
                    eprintln!("part{}: took {:?}", part, elapsed);
                    tot_elapsed += elapsed;
                }
            }
        }
    }

    if opts.day == 0 {
        eprintln!("\nTotal time: {:?}", tot_elapsed);
    }
}
