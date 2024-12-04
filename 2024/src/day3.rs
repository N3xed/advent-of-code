#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Inst {
    Do,
    Dont,
    Mul,
}

const KEYWORDS: [&'static str; 3] = ["don't", "do", "mul"];

pub fn day3(data: &str, p1: bool) -> i64 {
    if p1 {
        let result: isize = data
            .split("mul")
            .skip(1)
            .filter_map(|after_mul| {
                let (before_brace, after_brace) = after_mul.split_once('(')?;
                if !before_brace.is_empty() {
                    return None;
                }
                let (lhs, after_comma) = after_brace.split_once(',')?;
                let (rhs, _) = after_comma.split_once(')')?;

                if !lhs.chars().all(|c| c.is_ascii_digit()) {
                    return None;
                }
                if !rhs.chars().all(|c| c.is_ascii_digit()) {
                    return None;
                }
                let lhs = lhs.parse::<isize>().expect("must be number");
                let rhs = rhs.parse::<isize>().expect("must be number");

                Some(lhs * rhs)
            })
            .sum();
        return result as i64;
    }

    let mut chunks = Vec::<(Inst, &str)>::new();
    let mut slice = data;
    let mut last_inst = None;

    loop {
        let Some((next_inst, chunk, rest)) = KEYWORDS
            .iter()
            .enumerate()
            .filter_map(|(i, kw)| {
                let Some((chunk, rest)) = slice.split_once(kw) else {
                    return None;
                };

                let inst = match i {
                    0 => Inst::Dont,
                    1 => Inst::Do,
                    2 => Inst::Mul,
                    _ => unreachable!(),
                };

                Some((inst, chunk, rest))
            })
            .min_by_key(|(_, chunk, _)| chunk.len())
        else {
            if let Some(inst) = last_inst.take() {
                chunks.push((inst, slice));
            }
            break;
        };
        slice = rest;
        if let Some(inst) = last_inst.take() {
            chunks.push((inst, chunk));
        }
        last_inst = Some(next_inst);
    }

    let mut mul_enabled = true;
    let result: isize = chunks
        .into_iter()
        .filter_map(|(inst, chunk)| match inst {
            Inst::Mul => {
                let (before_brace, after_brace) = chunk.split_once('(')?;
                if !before_brace.is_empty() {
                    return None;
                }
                let (lhs, after_comma) = after_brace.split_once(',')?;
                let (rhs, _) = after_comma.split_once(')')?;

                if !mul_enabled {
                    return None;
                }
                if !lhs.chars().all(|c| c.is_ascii_digit()) {
                    return None;
                }
                if !rhs.chars().all(|c| c.is_ascii_digit()) {
                    return None;
                }

                let lhs = lhs.parse::<isize>().expect("must be number");
                let rhs = rhs.parse::<isize>().expect("must be number");

                Some(lhs * rhs)
            }
            Inst::Dont | Inst::Do => {
                if let Some(_) = chunk.strip_prefix("()") {
                    mul_enabled = inst == Inst::Do;
                }
                None
            }
        })
        .sum();
    result as i64
}
