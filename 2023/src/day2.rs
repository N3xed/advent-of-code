pub fn day2(data: &str, r: usize, g: usize, b: usize, p2: bool) {
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
