use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::day23::Set;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn from_str(s: &str) -> Option<Op> {
        Some(match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => return None,
        })
    }

    fn apply(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Self::And => lhs && rhs,
            Self::Or => lhs || rhs,
            Self::Xor => lhs != rhs,
        }
    }
}

pub fn day24(data: &str, p1: bool) -> i64 {
    let mut lines = data.lines();

    let mut states: HashMap<&str, bool> = (&mut lines)
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            let (name, num) = l.split_once(":")?;
            Some((name.trim(), num.trim().parse::<u8>().ok().unwrap() != 0))
        })
        .collect();

    let ops = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (lhs, op, rhs, _arrow, out) = l.split(' ').collect_tuple().unwrap();
            (lhs, Op::from_str(op.trim()).unwrap(), rhs, out)
        })
        .collect_vec();

    let mut map = HashMap::<&str, Vec<(&str, (Op, &str))>>::new();
    for (lhs, op, rhs, out) in ops {
        map.entry(lhs).or_default().push((rhs, (op, out)));
        map.entry(rhs).or_default().push((lhs, (op, out)));
    }

    if p1 {
        let mut paths = VecDeque::<(Set<&str, 2>, Op, &str)>::new();
        paths.extend(
            states
                .keys()
                .filter_map(|k| {
                    Some(
                        map.get(k)?
                            .iter()
                            .map(|(k2, (op, out))| (Set::new([*k, *k2]), *op, *out)),
                    )
                })
                .flatten(),
        );

        while let Some(ref p @ (ref item, ref op, out)) = paths.pop_front() {
            if states.contains_key(out) {
                continue;
            }

            let [a, b] = item.clone().into_inner();
            let Some(&a_state) = states.get(a) else {
                paths.push_back(p.clone());
                continue;
            };
            let Some(&b_state) = states.get(b) else {
                paths.push_back(p.clone());
                continue;
            };

            let out_state = op.apply(a_state, b_state);
            states.insert(out, out_state);
            println!("{item:?} -> {out} = {out_state}");

            let Some(m) = map.get(out) else { continue };

            for (k, (op, next_out)) in m {
                paths.push_back((Set::new([out, *k]), *op, *next_out));
            }
        }

        let zs = states
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .sorted()
            .collect_vec();

        for (z, val) in &zs {
            println!("{z} = {val}");
        }

        let result: u64 = zs
            .iter()
            .map(|(k, v)| {
                (
                    k.trim_matches(|c: char| !c.is_digit(10))
                        .parse::<u32>()
                        .unwrap(),
                    v,
                )
            })
            .map(|(k, &&v)| (v as u64) << (k as u64))
            .sum();

        return result as i64;
    }

    unimplemented!("part 2");
}
