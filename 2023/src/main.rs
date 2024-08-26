use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Clone)]
enum Command {
    D1 { file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::D1 { file } => {
            day1(&std::fs::read_to_string(file)?)?;
        }
    }

    Ok(())
}

fn day1(data: &str) -> anyhow::Result<()> {
    struct DigitParser {
        pos: [u8; 9],
    }

    impl DigitParser {
        const DIGITS: [(&'static [u8], usize); 9] = [
            (b"three", 3),
            (b"seven", 7),
            (b"eight", 8),
            (b"four", 4),
            (b"five", 5),
            (b"nine", 9),
            (b"one", 1),
            (b"two", 2),
            (b"six", 6),
        ];

        fn new() -> Self {
            Self { pos: [0; 9] }
        }

        fn parse<const REV: bool>(&mut self, c: char) -> Option<usize> {
            let get_c = if REV {
                |i: usize, p: usize| Self::DIGITS[i].0[Self::DIGITS[i].0.len() - 1 - p] as char
            } else {
                |i: usize, p: usize| Self::DIGITS[i].0[p] as char
            };

            for (i, pos) in self.pos.iter_mut().enumerate() {
                let n = Self::DIGITS[i].1;
                if c == get_c(i, *pos as usize) {
                    *pos += 1;

                    if *pos as usize >= Self::DIGITS[i].0.len() {
                        return Some(n);
                    }
                } else {
                    *pos = 0;
                }
            }
            None
        }
    }

    let sum: usize = data
        .lines()
        .enumerate()
        .map(|(i, l)| {
            if l.is_empty() {
                return 0;
            }

            let mut p = DigitParser::new();
            let (c1, i1) = l
                .char_indices()
                .find_map(|(i, c)| {
                    if c.is_ascii_digit() {
                        Some((c as usize - '0' as usize, i))
                    } else if let Some(d) = p.parse::<false>(c) {
                        Some((d, i))
                    } else {
                        None
                    }
                })
                .expect("first must be present");

            p = DigitParser::new();
            let c2 = l
                .char_indices()
                .rev()
                .find_map(|(i, c)| {
                    if i <= i1 {
                        return None;
                    }
                    if c.is_ascii_digit() {
                        Some(c as usize - '0' as usize)
                    } else if let Some(d) = p.parse::<true>(c) {
                        Some(d)
                    } else {
                        None
                    }
                })
                .unwrap_or(c1);

            let r = c1 * 10 + c2;

            r
        })
        .sum();

    println!("Result: {sum}");
    Ok(())
}
