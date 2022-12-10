use std::fs::read_to_string;
use std::time::{Duration, Instant};
use clap::Parser;

const DAYS: &'static [(u32, &str, u32, fn(&str), fn(&str))] = &[
    ( 1, "day-01-calorie-counting", 2, day_01::part1, day_01::part2 ),
    ( 2, "day-02-rock-paper-scissors", 2, day_02::part1a, day_02::part2a ),
    ( 3, "day-03-rucksack-reorganization", 2, day_03::part1, day_03::part2 ),
    ( 4, "day-04-camp-cleanup", 1, day_04::part1_2, day_04::part1_2 ),
    ( 5, "day-05-supply-stacks", 1, day_05::part1_2, day_05::part1_2 ),
    ( 6, "day-06-tuning-trouble", 2, day_06::part1, day_06::part2 ),
    ( 7, "day-07-no-space-left-on-device", 1, day_07::part1_2_notree, day_07::part1_2 ),
    ( 8, "day-08-treetop-tree-house", 2, day_08::part1, day_08::part2 ),
    ( 9, "day-09-rope-bridge", 1, day_09::part1_2, day_09::part1_2 ),
    ( 10, "day-10-cathode-ray-tube", 1, day_10::part1_2, day_10::part1_2 ),
    ( 105, "day-05-supply-stacks", 1, day_05::part1_2_heavy_duty, day_05::part1_2 ),
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

    for (day, dir, nparts, part1, part2) in DAYS {
        if (opts.day == 0 && *day < 100) || opts.day == *day {
            eprintln!("{}: {}", &dir[..6], &dir[7..]);
            let file = format!("{}/input/{}", dir, opts.input);
            let input = read_to_string(&file).expect(&file);
            for part in 1 ..= *nparts {
                if opts.part.is_none() || opts.part == Some(part) || *nparts == 1 {
                    if *nparts == 1 {
                        eprintln!(" == start ==");
                    } else {
                        eprintln!("part{}: == start ==", part);
                    }
                    let start = Instant::now();
                    if part == 1 { part1(&input) } else { part2(&input) }
                    let elapsed = start.elapsed();
                    if *nparts == 1 {
                        eprintln!("took {:?}", elapsed);
                    } else {
                        eprintln!("part{}: took {:?}", part, elapsed);
                    }
                    tot_elapsed += elapsed;
                }
            }
        }
    }

    if opts.day == 0 {
        eprintln!("\nTotal time: {:?}", tot_elapsed);
    }
}
