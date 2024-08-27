pub fn day1(data: &str, p1: bool) {
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
