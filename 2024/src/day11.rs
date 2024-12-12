use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use tqdm::Iter;

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

/// Apply rules to stones list.
///
/// - If the stone is engraved with the number 0, it is replaced by a stone engraved with the
///     number 1.
/// - If the stone is engraved with a number that has an even number of digits, it is replaced by
///     two stones. The left half of the digits are engraved on the new left stone, and the right
///     half of the digits are engraved on the new right stone. (The new numbers don't keep extra
///     leading zeroes: 1000 would become stones 10 and 0.)
/// - If none of the other rules apply, the stone is replaced by a new stone; the old stone's
///     number multiplied by 2024 is engraved on the new stone.
fn apply_rules_once(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        return (1, None);
    }

    let n_digits = count_digits(n);
    if n_digits % 2 == 0 {
        let divider = 10_u64.pow(n_digits / 2);
        let lhs = n / divider;
        let rhs = n % divider;

        return (lhs, Some(rhs));
    }

    (n * 2024, None)
}

fn apply_rules(mut nums: Vec<u64>, iters: u8) -> Vec<u64> {
    for _ in 0..iters {
        for i in 0..nums.len() {
            let n = unsafe { nums.get_unchecked_mut(i) };
            let (n_update, new_n) = apply_rules_once(*n);
            *n = n_update;
            if let Some(n) = new_n {
                nums.push(n);
            }
        }
    }
    nums
}

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Stone {
    num: u64,
    iters: u8,
}

impl Stone {
    /// Apply rules until [`Self::iters`] is equal to `n_iters`.
    ///
    /// Returns the amount of stones after the final iteration.
    fn apply_rules_and_count(
        self,
        containers: &mut (Vec<u64>, Vec<Stone>, HashMap<Stone, usize>),
        n_iters: u8,
        mut max_nums_per_batch: usize,
        max_nums_final: usize,
    ) -> usize {
        let (ref mut nums, ref mut unfinished, map) = containers;
        let mut finished = 0;

        unfinished.clear();
        unfinished.push(self);
        while let Some(mut stone) = unfinished.pop() {
            if let Some(&count) = map.get(&stone) {
                finished += count;
                continue;
            }

            nums.clear();
            nums.push(stone.num);

            while (stone.iters < n_iters) && (nums.len() <= max_nums_per_batch) {
                for i in 0..nums.len() {
                    let n = unsafe { nums.get_unchecked_mut(i) };
                    let (n_update, new_n) = apply_rules_once(*n);
                    *n = n_update;
                    if let Some(n) = new_n {
                        nums.push(n);
                    }
                }
                stone.iters += 1;
            }
            max_nums_per_batch = max_nums_final;

            if stone.iters == n_iters {
                finished += nums.len();
                map.entry(stone).or_insert(nums.len());
            } else {
                unfinished.extend(nums.iter().map(|&num| Stone {
                    num,
                    iters: stone.iters,
                }));
            }
        }
        finished
    }
}

pub fn day11(data: &str, p1: bool) -> i64 {
    let nums = data
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().expect("unsigned number"))
        .collect_vec();

    if p1 {
        let nums = apply_rules(nums, 25);
        return nums.len() as i64;
    }

    let nums = apply_rules(nums, 35);
    let result: usize = nums
        .into_iter()
        .tqdm()
        .map(|num| Stone { num, iters: 35 })
        .par_bridge()
        .map_init(
            || (Vec::new(), Vec::new(), HashMap::new()),
            |vecs, stone: Stone| {
                if let Some(&c) = vecs.2.get(&stone) {
                    return c;
                }
                let count = stone.apply_rules_and_count(vecs, 75, 100, 1_000_000);
                vecs.2.entry(stone).or_insert(count);
                count
            },
        )
        .sum();

    result as i64
}

#[test]
fn test_p1() {
    assert_eq!(vec![1, 2], apply_rules(vec![12], 1));
    assert_eq!(55312, day11("125 17", true));
}
