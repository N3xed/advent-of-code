use std::{num::NonZeroUsize, path::PathBuf};

use anyhow::Context;
use clap::Parser;

macro_rules! days {
    ($($d:literal),*; $last_d:literal) => {
        paste::paste!{
            $(
                mod [<day $d>];
            )*
            mod [<day $last_d>];
        }

        fn run_day(day: Option<usize>, data: &str, p1: bool) -> anyhow::Result<i64> {
            paste::paste!{
                Ok(match day {
                    $(
                        Some($d) => [<day $d>]::[<day $d>] (data, p1),
                    )*
                    Some($last_d) | None => [<day $last_d>]::[<day $last_d>] (data, p1),
                    Some(d) => anyhow::bail!("day {d} not implemented")
                })
            }
        }
    };
}

days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18; 19);

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
    let result = run_day(args.day.map(Into::into), &data, args.p1)?;
    println!("result = {result}");
    Ok(())
}
