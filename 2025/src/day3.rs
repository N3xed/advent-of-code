use itertools::Itertools;

fn largest_numerical_subsequence(nums: &[u8], len: usize) -> (usize, Vec<usize>) {
    let mut result_indices = Vec::<usize>::with_capacity(len);
    let mut result = 0_usize;

    let end_idx = nums.len() - 1;
    let mut nums_needed = len;

    let mut largest = Vec::with_capacity(len);
    while nums_needed > 0 {
        let (nums_range, start_idx) = if let Some(&last_idx) = result_indices.last() {
            let start_idx = last_idx + 1;
            (&nums[start_idx..], start_idx)
        } else {
            (nums, 0)
        };

        largest.clear();
        largest.extend(
            (start_idx..)
                .zip(nums_range.iter().copied())
                .k_largest_by_key(nums_needed, |(_, n)| *n),
        );

        // Find the biggest digit the furthest to left with the constraint that to the right of that digit
        // there are at least (nums_needed - 1) digits.

        // Sort by max digit, and if equal by index.
        largest.sort_unstable_by_key(|&(i, n)| (u8::MAX - n, i));

        let best = largest
            .iter()
            .find(|&(i, _)| (end_idx - i) >= (nums_needed - 1))
            .unwrap();
        result_indices.push(best.0);
        result = result * 10 + (best.1 as usize);
        nums_needed -= 1;
    }

    (result, result_indices)
}

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let banks = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums = l
                .as_bytes()
                .iter()
                .map(|b| (b - b'0') as u8)
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();

    if p1 {
        return banks
            .into_iter()
            .map(move |bank| {
                let mut largest = bank
                    .iter()
                    .copied()
                    .enumerate()
                    .k_largest_by_key(2, |(_, n)| *n);

                let mut n0 = largest.next().expect("must not be empty");
                let Some(mut n1) = largest.next() else {
                    return (n0.1 as usize, (n0.0, n0.0), bank);
                };

                // Sort in order of the bank.
                if n1.0 < n0.0 {
                    std::mem::swap(&mut n0, &mut n1);
                }

                fn make_n(
                    n0: (usize, u8), // (idx of the number in the bank, number)
                    n1: (usize, u8), // (idx of the number in the bank, number)
                    bank: Vec<u8>,
                ) -> (usize, (usize, usize), Vec<u8>) {
                    ((n0.1 as usize) * 10 + (n1.1 as usize), (n0.0, n1.0), bank)
                }

                // If the second digit is at the end or the first digit is the bigger of the two,
                // we have found the best pair of digits.
                if n1.0 == bank.len() - 1 || n0.1 >= n1.1 {
                    return make_n(n0, n1, bank);
                }
                // Otherwise use n1 as the first digit, and use the biggest digit to the right of
                // it as the second digit.
                let one_past = n1.0 + 1;
                let n2 = (one_past..)
                    .zip(bank[one_past..].iter().copied())
                    .max_by_key(|(_, n)| *n)
                    .unwrap();

                make_n(n1, n2, bank)
            })
            .map(|(n, (i0, i1), bank)| {
                // Pretty print.
                let s = bank
                    .iter()
                    .enumerate()
                    .map(|(idx, n)| {
                        if idx == i0 || idx == i1 {
                            format!("[{n}]")
                        } else {
                            format!("{n}")
                        }
                    })
                    .join("");
                println!("{s} -> {n}");

                n
            })
            .sum::<usize>();
    }

    banks
        .iter()
        .map(|bank| (largest_numerical_subsequence(bank, 12), bank))
        .map(|((n, idx), bank)| {
            // Pretty print.
            let s = bank
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    if idx.iter().any(|j| i == *j) {
                        format!("[{n}]")
                    } else {
                        format!("{n}")
                    }
                })
                .join("");
            println!("{s} -> {n}");

            n
        })
        .sum::<usize>()
}
