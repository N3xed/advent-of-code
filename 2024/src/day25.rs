use itertools::Itertools;

pub fn day25(data: &str, _p1: bool) -> i64 {
    let lines = data.lines().collect_vec();

    let (keys, locks): (Vec<_>, Vec<_>) = lines
        .split(|l| l.is_empty())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let is_key = l[0].chars().all(|c| c == '.');
            let mut cols = [0_u8; 5];

            for l in &l[1..6] {
                for (i, c) in l.chars().enumerate().take(5) {
                    cols[i] += (c == '#') as u8;
                }
            }

            if is_key {
                (Some(cols), None)
            } else {
                (None, Some(cols))
            }
        })
        .unzip();
    let keys = keys.into_iter().flatten().collect_vec();
    let locks = locks.into_iter().flatten().collect_vec();

    let matching_pairs = keys
        .iter()
        .flat_map(|k| locks.iter().map(move |l| (k, l)))
        .filter(|(k, l)| k.iter().zip(l.iter()).all(|(&k, &l)| (k + l) <= 5))
        .collect_vec();

    matching_pairs.len() as i64
}
