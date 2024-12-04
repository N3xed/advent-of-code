use itertools::Itertools;

fn is_safe_levels(levels: impl IntoIterator<Item = u32>) -> bool {
    let (_, is_safe) = levels
        .into_iter()
        .tuple_windows()
        .map(|(l, r)| (l.abs_diff(r), r > l))
        .fold((None, true), |(last_greater, safe), (e_diff, e_greater)| {
            let always_inc_or_dec = if let Some(last_greater) = last_greater {
                last_greater == e_greater
            } else {
                true
            };
            let is_safe_step = e_diff >= 1 && e_diff <= 3;

            (Some(e_greater), is_safe_step && always_inc_or_dec && safe)
        });
    is_safe
}

pub fn day2(data: &str, p1: bool) -> i64 {
    let result: usize = data
        .lines()
        .filter_map(|l| {
            let levels = l
                .split(char::is_whitespace)
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            if levels.is_empty() {
                return None;
            }

            let is_safe = if p1 {
                is_safe_levels(levels.iter().copied())
            } else {
                let safe = is_safe_levels(levels.iter().copied());
                if levels.len() <= 1 || safe {
                    true
                } else {
                    let mut safe = false;
                    for i in 0..levels.len() {
                        let slice_a = &levels[0..i];
                        let slice_b = &levels[i + 1..];

                        if is_safe_levels(slice_a.iter().chain(slice_b).copied()) {
                            safe = true;
                            break;
                        }
                    }
                    safe
                }
            };
            Some(is_safe as usize)
        })
        .sum();

    result as i64
}
