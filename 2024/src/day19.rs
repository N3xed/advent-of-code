use std::collections::HashMap;

use itertools::Itertools;

pub fn day19(data: &str, p1: bool) -> i64 {
    let mut lines = data.lines();
    let mut proto = lines
        .next()
        .unwrap()
        .split(',')
        .map(|p| p.trim())
        .collect_vec();

    let patterns = lines
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() {
                return None;
            }
            Some(p)
        })
        .collect_vec();

    // Sort by maximum length and then alphabetically.
    proto.sort_by(|a, b| (b.len(), a).cmp(&(a.len(), b)));
    proto.dedup();

    fn count_combs<'a>(
        des: &'a str,
        protos: &[&str],
        stop_at_first: bool,
        mem: &mut HashMap<&'a [u8], usize>,
    ) -> usize {
        let mut result = 0_usize;
        let mut indices = vec![0_usize];
        let mut des_stack = Vec::with_capacity(protos.len());
        let mut counts = vec![0_usize];
        des_stack.push(des.as_bytes());
        while let (Some(des), Some(idx)) = (des_stack.last(), indices.last_mut()) {
            if let Some(v) = mem.get(des) {
                result += v;
                des_stack.pop();
                indices.pop();
                counts.pop();
                continue;
            }

            let found = protos[*idx..].iter().enumerate().find_map(|(i, p)| {
                let rest = des.strip_prefix(p.as_bytes())?;
                Some((i, rest))
            });

            match found {
                Some((i, rest)) => {
                    *idx += i + 1;

                    if rest.is_empty() {
                        if stop_at_first {
                            return 1;
                        } else {
                            result += 1;
                            continue;
                        }
                    } else {
                        des_stack.push(rest);
                        indices.push(0);
                        counts.push(result);
                    }
                }
                None => {
                    let diff_count = result - counts.pop().unwrap();
                    mem.insert(des, diff_count);

                    des_stack.pop();
                    indices.pop();
                }
            }
        }
        result
    }

    let result = if p1 {
        patterns
            .iter()
            .filter(|p| count_combs(p, &proto, true, &mut Default::default()) != 0)
            .count()
    } else {
        patterns
            .iter()
            .scan(Default::default(), |state, p| {
                Some(count_combs(p, &proto, false, state))
            })
            .sum()
    };

    result as i64
}
