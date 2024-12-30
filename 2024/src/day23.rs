use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use itertools::Itertools;
use tqdm::Iter;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Set<T, const N: usize>([T; N]);

impl<T: Ord, const N: usize> Set<T, N> {
    pub fn new(mut v: [T; N]) -> Self {
        v.sort();
        Self(v)
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Set<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.0.iter()).finish()
    }
}

#[allow(dead_code)]
impl<T, const N: usize> Set<T, N> {
    pub fn into_inner(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> Deref for Set<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const N: usize> DerefMut for Set<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn day23(data: &str, p1: bool) -> i64 {
    let cons = data
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| Some(l.split_once('-').unwrap()))
        .collect_vec();

    let mut map = HashMap::<&str, HashSet<&str>>::new();

    for (a, b) in &cons {
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }

    if p1 {
        let mut threes = HashSet::new();
        for (k, v) in &map {
            for v in v {
                for v2 in map
                    .get(v)
                    .into_iter()
                    .flatten()
                    .filter(|v2| map.get(*v2).is_some_and(|v2_cons| v2_cons.contains(k)))
                {
                    threes.insert(Set::new([*k, *v, *v2]));
                }
            }
        }

        let result = threes
            .iter()
            .filter(|t| t.iter().any(|s| s.starts_with('t')))
            .sorted()
            .collect_vec();

        return result.len() as i64;
    }

    let mut largest_component = HashSet::new();
    for (&a, b) in map.iter().tqdm() {
        let mut comp = HashSet::new();

        comp.insert(a);
        let mut curr_path = vec![b];
        while let Some(b) = curr_path.pop() {
            for b in b {
                let Some(m) = map.get(b) else { continue };

                if comp.iter().all(|a| m.contains(a)) {
                    comp.insert(b);
                    curr_path.push(m)
                }
            }
        }

        if comp.len() > largest_component.len() {
            largest_component = comp;
        }
    }

    let pwd = largest_component.iter().sorted().join(",");
    println!("password: {pwd}");

    largest_component.len() as i64
}
