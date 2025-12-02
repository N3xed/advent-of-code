use std::fmt::Write;

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let ranges = data
        .trim()
        .split(',')
        .map(|s| {
            let (a, b) = s.trim().split_once('-').expect("invalid range");

            let a = a.parse::<usize>().expect("invalid number");
            let b = b.parse::<usize>().expect("invalid number");

            a..=b
        })
        .collect::<Vec<_>>();

    let mut temp = String::new();
    if p1 {
        return ranges
            .into_iter()
            .flatten()
            .filter(|n| {
                temp.clear();
                write!(&mut temp, "{n}").unwrap();
                let ndigits = temp.len();

                if ndigits % 2 != 0 {
                    return false;
                }
                // If the first half of the digits match the second half, we have a match.
                temp[..ndigits / 2] == temp[ndigits / 2..]
            })
            .sum::<usize>()
            .to_string();
    }

    ranges
        .into_iter()
        .inspect(|r| println!("{}-{}:", r.start(), r.end()))
        .flatten()
        .filter(|&n| {
            temp.clear();
            write!(&mut temp, "{n}").unwrap();

            let ndigits = temp.len();

            // Try all possible groups of digits (1 to ndigits groups).
            for i in 1..=ndigits {
                let chunks = ndigits / i;
                // We must have 2 or more groups, if not we are sure
                // that n will never match since i is increasing.
                if chunks <= 1 {
                    return false;
                }
                // All groups must be filled completely.
                if ndigits % i != 0 {
                    continue;
                }

                let mut c = temp.as_bytes().chunks_exact(i);
                let first_chunk = c.next().unwrap();

                // If all groups of digits are the same, we have a match.
                if c.all(|c| c == first_chunk) {
                    println!(
                        "    n = {temp}, {:?}, len = {chunks}",
                        temp.as_bytes()
                            .chunks_exact(i)
                            .map(|s| str::from_utf8(s).unwrap())
                            .collect::<Vec<_>>()
                    );
                    return true;
                }
            }
            false
        })
        .sum::<usize>()
        .to_string()
}
