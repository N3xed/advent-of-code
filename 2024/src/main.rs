use std::{num::NonZeroUsize, path::PathBuf};

use anyhow::Context;
use clap::Parser;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;

#[derive(Parser)]
struct Args {
    /// Which day to run. If unspecified, runs the latest day.
    #[clap(short, long)]
    day: Option<NonZeroUsize>,
    /// Run probem one.
    #[clap(long)]
    p1: bool,

    /// The file with the problem input.
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = std::fs::read_to_string(&args.file)
        .with_context(|| format!("file '{}' not found", args.file.display()))?;

    let result = match args.day.map(|v| v.into()) {
        Some(1) => day1(&data, args.p1),
        Some(2) => day2(&data, args.p1),
        Some(3) => day3(&data, args.p1),
        Some(4) => day4(&data, args.p1),
        Some(5) => day5(&data, args.p1),
        Some(6) => day6(&data, args.p1),
        Some(7) => day7(&data, args.p1),
        Some(8) => day8(&data, args.p1),
        Some(9) => day9(&data, args.p1),
        Some(10) => day10(&data, args.p1),
        Some(11) => day11(&data, args.p1),
        Some(12) => day12(&data, args.p1),
        Some(13) => day13(&data, args.p1),
        Some(14) => day14(&data, args.p1),
        Some(15) | None => day15(&data, args.p1),
        Some(d) => anyhow::bail!("day {d} not implemented"),
    };

    println!("result = {result}");
    Ok(())
}
