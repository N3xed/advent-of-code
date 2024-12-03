use std::path::PathBuf;

use clap::Parser;
mod day1;
mod day2;
mod day3;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Clone)]
enum Command {
    /// Day 1.
    D1 {
        /// Puzzle input.
        file: PathBuf,
        /// Run problem one.
        #[clap(long)]
        p1: bool,
    },
    /// Day 2.
    D2 {
        /// Puzzle input.
        file: PathBuf,
        /// Run problem one.
        #[clap(long)]
        p1: bool,
    },
    /// Day 3.
    D3 {
        /// Puzzle input.
        file: PathBuf,
        /// Run problem one.
        #[clap(long)]
        p1: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::D1 { file, p1 } => {
            day1::day1(&std::fs::read_to_string(file)?, p1);
        }
        Command::D2 { file, p1 } => {
            day2::day2(&std::fs::read_to_string(file)?, p1);
        }
        Command::D3 { file, p1 } => {
            day3::day3(&std::fs::read_to_string(file)?, p1);
        }
    }
    Ok(())
}
