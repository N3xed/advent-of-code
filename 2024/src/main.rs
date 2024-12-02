use std::path::PathBuf;

use clap::Parser;
mod day1;

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
        /// Run problem two.
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
    }
    Ok(())
}
