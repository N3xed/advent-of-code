use std::path::PathBuf;

use clap::Parser;
mod day1;
mod day2;

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
    /// Day 2.
    D2 {
        /// Puzzle input.
        file: PathBuf,

        /// Number of red cubes.
        r: Option<usize>,
        /// Number of green cubes.
        g: Option<usize>,
        /// Number of blue cubes.
        b: Option<usize>,

        /// Run problem two.
        #[clap(long)]
        p2: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::D1 { file, p1 } => {
            day1::day1(&std::fs::read_to_string(file)?, p1);
        }
        Command::D2 { file, r, g, b, p2 } => {
            day2::day2(
                &std::fs::read_to_string(file)?,
                r.unwrap_or_default(),
                g.unwrap_or_default(),
                b.unwrap_or_default(),
                p2,
            );
        }
    }
    Ok(())
}
