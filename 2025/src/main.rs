use std::{num::NonZeroUsize, path::PathBuf};

use anyhow::Context;
use clap::Parser;

mod day1;

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

    let result: Box<dyn std::fmt::Display> = match args.day.map(|v| v.into()) {
        Some(1) | None => Box::new(day1::run(&data, args.p1)),
        Some(d) => anyhow::bail!("day {d} not implemented"),
    };

    println!("result = {result}");
    Ok(())
}
