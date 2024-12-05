use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl Rule {
    pub fn parse(line: &str) -> Option<Rule> {
        let (a, b) = line.split_once('|')?;

        let first = a.trim().parse::<u32>().ok()?;
        let second = b.trim().parse::<u32>().ok()?;
        Some(Rule { first, second })
    }
}

pub fn day5(data: &str, p1: bool) -> i64 {
    let lines = data.lines().collect_vec();
    let (empty_line_idx, _) = lines
        .iter()
        .find_position(|a| a.is_empty())
        .expect("an empty line");

    let rules = &lines[..empty_line_idx];
    let rules = rules
        .into_iter()
        .filter_map(|l| Rule::parse(l))
        .collect_vec();
    let pages = &lines[empty_line_idx + 1..];
    let pages = pages
        .into_iter()
        .filter_map(|l| {
            let ns = l
                .split(',')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect_vec();
            if ns.is_empty() {
                None
            } else {
                Some(ns)
            }
        })
        .collect_vec();

    fn is_correct_order(pages: &[u32], rules: &[Rule]) -> bool {
        rules
            .iter()
            .filter_map(|r| {
                let first_pos = pages.iter().position(|p| *p == r.first)?;
                let second_pos = pages.iter().position(|p| *p == r.second)?;
                Some(first_pos < second_pos)
            })
            .all(|v| v == true)
    }

    let result: u32 = if p1 {
        pages
            .iter()
            .filter(|p| is_correct_order(p, &rules))
            .map(|p| {
                let middle_idx = (p.len() - 1) / 2;
                p[middle_idx]
            })
            .sum()
    } else {
        // A HashMap that has for a key, its list of page numbers that are all ordered after the
        // key.
        let mut is_smaller_map = HashMap::<u32, Vec<u32>>::new();
        for rule in rules.iter() {
            let list = is_smaller_map.entry(rule.first).or_default();
            if !list.contains(&rule.second) {
                list.push(rule.second);
            }
        }

        let mut disordered_pages = pages
            .iter()
            .filter(|p| !is_correct_order(p, &rules))
            .cloned()
            .collect_vec();
        for pages in disordered_pages.iter_mut() {
            pages.sort_by(|a, b| {
                if let Some(bigger_list) = is_smaller_map.get(a) {
                    if bigger_list.contains(b) {
                        // b is in the bigger_list of a, so a must be less than b.
                        return Ordering::Less;
                    }
                }
                if let Some(bigger_list) = is_smaller_map.get(b) {
                    if bigger_list.contains(a) {
                        // a is in the bigger_list of b, so a must be greater than b.
                        return Ordering::Greater;
                    }
                }
                // We did not find an ordering for a and b, they must be equally ordered.
                Ordering::Equal
            });

            assert!(is_correct_order(pages, &rules));
        }

        disordered_pages
            .into_iter()
            .map(|p| {
                let middle_idx = (p.len() - 1) / 2;
                p[middle_idx]
            })
            .sum()
    };

    result as i64
}
