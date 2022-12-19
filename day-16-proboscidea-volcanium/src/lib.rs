use std::collections::HashMap;
use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();
    let cave = Cave::parse(input);
    ctx.update_timer(Ctx::PARSING);
    for i in 0 .. cave.valves.len() {
        println!("{} {:?}", i, cave.valves[i]);
    }
    println!("start: {}", cave.start);
    //ctx.update_timer(Ctx::PART1);
    //ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<usize>,
}

#[derive(Debug)]
struct Cave {
    valves: Vec<Valve>,
    start: usize,
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let mut valves = Vec::new();
        let mut start = 0;
        let hm = input
            .lines()
            .map(|l| {
                let l = l.split_once(';').unwrap().0;
                let name = &l[6..8];
                let rate = l[23..].parse::<u32>().unwrap();
                let idx = valves.len();
                valves.push(Valve {
                    name: name.to_string(),
                    rate,
                    tunnels: Vec::default(),
                });
                if name == "AA" {
                    start = idx;
                }
                (name.to_string(), idx)
            })
            .collect::<HashMap<_, _>>();
        input
            .lines()
            .map(|l| l.split_once(';').unwrap().1[23..].trim())
            .enumerate()
            .for_each(|(v, list)| {
                println!("list: {list}");
                valves[v].tunnels = list.split(", ").map(|name| hm[name]).collect();
            });
        Cave { valves, start }
    }
}
