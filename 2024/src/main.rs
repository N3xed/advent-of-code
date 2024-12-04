use std::{num::NonZeroUsize, path::PathBuf};

use anyhow::Context;
use clap::Parser;
mod day1;
mod day2;
mod day3;
mod day4;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;

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
        Some(4) | None => day4(&data, args.p1),
        Some(d) => anyhow::bail!("day {d} not implemented"),
    };

    println!("result = {result}");
    Ok(())
}
