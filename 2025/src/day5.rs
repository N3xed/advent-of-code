use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let mut lines = data.lines();
    let mut ranges_overlapping = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').expect("range does not contain `-`");
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            a..=b
        })
        .collect_vec();

    // Sort ranges by start position to simplify merging.
    ranges_overlapping.sort_by_key(|r| *r.start());

    // Merge overlapping ranges into a disjoint set.
    let mut ranges = Vec::<RangeInclusive<usize>>::new();
    for r in ranges_overlapping {
        if let Some(last) = ranges.last_mut() {
            // Since ranges are sorted by start, we only need to check if the current
            // range starts before the last one ends to determine overlap.
            if *r.start() <= *last.end() {
                let new_end = (*last.end()).max(*r.end());
                *last = *last.start()..=new_end;
            } else {
                ranges.push(r);
            }
        } else {
            ranges.push(r);
        }
    }

    let ids = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    if p1 {
        return ids
            .into_iter()
            .filter(|id| {
                // Use binary search to find if the id is contained in any range.
                ranges
                    .binary_search_by(|r| {
                        if r.contains(id) {
                            std::cmp::Ordering::Equal
                        } else if *r.start() > *id {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    })
                    .is_ok()
            })
            .count();
    }

    // Since the `ranges` are already disjoint, all we have to do is sum up the ranges.
    // Note the ranges are inclusive hence the plus one.
    ranges.into_iter().map(|r| *r.end() - *r.start() + 1).sum()
}
