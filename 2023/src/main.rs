use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Clone)]
enum Command {
    D1 {
        file: PathBuf,
        #[clap(long)]
        p1: bool,
    },
    D2 {
        file: PathBuf,
        r: Option<usize>,
        g: Option<usize>,
        b: Option<usize>,

        #[clap(long)]
        p2: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::D1 { file, p1 } => {
            day1(&std::fs::read_to_string(file)?, p1);
        }
        Command::D2 { file, r, g, b, p2 } => {
            day2(
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

fn day1(data: &str, p1: bool) {
    struct DigitParser {
        pos: [u8; 9],
    }

    impl DigitParser {
        const DIGITS: [&'static [u8]; 9] = [
            b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
        ];

        fn new() -> Self {
            Self { pos: [0; 9] }
        }

        fn parse(&mut self, c: char) -> Option<usize> {
            let mut res = None;

            for (i, pos) in self.pos.iter_mut().enumerate() {
                let num_c = Self::DIGITS[i][*pos as usize] as char;
                if c == num_c {
                    *pos += 1;

                    if *pos as usize >= Self::DIGITS[i].len() {
                        *pos = 0;
                        assert_eq!(res, None);
                        res = Some(i + 1);
                    }
                } else {
                    let num_c = Self::DIGITS[i][0] as char;
                    *pos = (c == num_c) as u8;
                }
            }
            return res;
        }
    }

    let sum: usize = data
        .lines()
        .map(|l| {
            if l.is_empty() {
                return 0;
            }

            let mut p = DigitParser::new();
            let mut iter = l.chars().filter_map(move |c: char| {
                if p1 {
                    return if c.is_ascii_digit() {
                        Some(c as usize - '0' as usize)
                    } else {
                        None
                    };
                }

                if c.is_ascii_digit() {
                    p.parse(c);
                    Some(c as usize - '0' as usize)
                } else if let Some(d) = p.parse(c) {
                    Some(d)
                } else {
                    None
                }
            });

            let d1 = iter.next().expect("at least one digit per line");
            let d2 = iter.last().unwrap_or(d1);

            let r = 10 * d1 + d2;
            r
        })
        .sum();

    println!("Result: {sum}");
}

fn day2(data: &str, r: usize, g: usize, b: usize, p2: bool) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct Count {
        r: usize,
        g: usize,
        b: usize,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Game {
        id: usize,
        counts: Vec<Count>,
        max_counts: Count,
    }

    impl Game {
        fn new(id: usize, counts: Vec<Count>) -> Self {
            let max_counts = counts.iter().fold(Count::default(), |accu, e| Count {
                r: accu.r.max(e.r),
                g: accu.g.max(e.g),
                b: accu.b.max(e.b),
            });
            Self {
                id,
                counts,
                max_counts,
            }
        }

        fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
            self.max_counts.r <= r && self.max_counts.g <= g && self.max_counts.b <= b
        }
        fn power(&self) -> usize {
            self.max_counts.r * self.max_counts.g * self.max_counts.b
        }
    }

    let games = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (left, right) = l.split_once(':').unwrap();
            let (_, id) = left.split_once(' ').unwrap();
            let id: usize = id.parse().expect("id is always positive number");

            let counts = right
                .split(';')
                .map(|count| {
                    let mut r = None;
                    let mut g = None;
                    let mut b = None;
                    for c in count.split(',') {
                        let Some((n, name)) = c.trim().split_once(" ") else {
                            continue;
                        };
                        match name.trim() {
                            "red" => {
                                assert_eq!(r, None);
                                r = Some(n.parse::<usize>().expect("number"));
                            }
                            "green" => {
                                assert_eq!(g, None);
                                g = Some(n.parse::<usize>().expect("number"));
                            }
                            "blue" => {
                                assert_eq!(b, None);
                                b = Some(n.parse::<usize>().expect("number"));
                            }
                            c => unreachable!("invalid color {c}"),
                        }
                    }
                    Count {
                        r: r.unwrap_or(0),
                        g: g.unwrap_or(0),
                        b: b.unwrap_or(0),
                    }
                })
                .collect();

            Game::new(id, counts)
        })
        .collect::<Vec<_>>();

    // for game in games.iter() {
    //     println!(
    //         "{game:?}, possible={}, power={}",
    //         game.is_possible(r, g, b),
    //         game.power()
    //     );
    // }

    let result: usize = if !p2 {
        games
            .iter()
            .map(|game| {
                if game.is_possible(r, g, b) {
                    game.id
                } else {
                    0
                }
            })
            .sum()
    } else {
        games.iter().map(|game| game.power()).sum()
    };

    println!("Result: {result}");
}
