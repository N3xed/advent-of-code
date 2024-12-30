use std::collections::HashMap;

use itertools::Itertools;

pub fn step(mut n: u64) -> u64 {
    const P: u64 = 16777216;

    let m = n * 64;
    n = (m ^ n) % P;

    let m = n / 32;
    n = (m ^ n) % P;

    let m = n * 2048;
    (m ^ n) % P
}

pub fn day22(data: &str, p1: bool) -> i64 {
    let nums = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();

    if p1 {
        let result: u64 = nums
            .iter()
            .map(|n| {
                let mut n = *n;
                for _ in 0..2000 {
                    n = step(n);
                }
                n
            })
            .sum();
        return result as i64;
    }

    let nums = nums
        .into_iter()
        .map(|n| {
            let mut nums = Vec::with_capacity(2001);
            let mut n = n;
            nums.push(n);
            for _ in 0..2000 {
                n = step(n);
                nums.push(n);
            }
            nums
        })
        .collect_vec();

    let map = nums.iter().fold(HashMap::new(), |mut res_map, nums| {
        let mut map = HashMap::new();
        let diff = nums
            .iter()
            .map(|n| (n % 10) as i8)
            .tuple_windows()
            .map(|(a, b)| (b - a, b as u8))
            .tuple_windows()
            .map(|((a, _), (b, _), (c, _), (d, n))| ((a, b, c, d), n))
            .collect_vec();

        // Find sequence and amount for this seller.
        for (diff, n) in diff {
            map.entry(diff).or_insert(n);
        }

        // Accumulate the amount for each sequences across sellers.
        for (seq, n) in map {
            *res_map.entry(seq).or_insert(0u64) += n as u64;
        }

        res_map
    });

    let top10 = map.iter().k_largest_by_key(10, |(_, &n)| n).collect_vec();
    println!("top 10 (seqence, amount) pairs:\n{:?}", top10);

    *top10.first().unwrap().1 as i64
}
