use std::collections::HashMap;

pub fn day1(data: &str, p1: bool) -> i64 {
    let (mut l_nums, mut r_nums): (Vec<_>, Vec<_>) = data
        .lines()
        .filter_map(|l| {
            let (lhs, rhs) = l.split_once(char::is_whitespace)?;
            let lhs = lhs.trim();
            let rhs = rhs.trim();
            if lhs.is_empty() || rhs.is_empty() {
                return None;
            }
            Some((lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap()))
        })
        .unzip();

    if p1 {
        l_nums.sort_unstable();
        r_nums.sort_unstable();

        let result: u32 = l_nums
            .into_iter()
            .zip(r_nums)
            .map(|(l, r)| l.abs_diff(r))
            .sum();

        return result as i64;
    } else {
        let mut r_nums_hash = HashMap::<u32, usize>::new();
        for n in r_nums {
            let entry = r_nums_hash.entry(n);
            *entry.or_insert(0) += 1;
        }

        let result: usize = l_nums
            .into_iter()
            .map(|n| (n as usize) * r_nums_hash.get(&n).unwrap_or(&0))
            .sum();

        return result as i64;
    }
}
