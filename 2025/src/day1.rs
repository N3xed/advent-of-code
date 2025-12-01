#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let inst = data
        .lines()
        .filter_map(|line| (!line.is_empty()).then_some(line))
        .map(|s| {
            let (dir, rest) = s.split_at(1);
            let dir = match dir {
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("invalid direction"),
            };
            let steps = rest.parse::<i32>().expect("invalid step count");
            (dir, steps)
        })
        .collect::<Vec<_>>();

    if p1 {
        return inst
            .into_iter()
            .scan(50_i32, |curr_step, (dir, steps)| {
                *curr_step = match dir {
                    Dir::Left => (*curr_step - steps) % 100,
                    Dir::Right => (*curr_step + steps) % 100,
                };
                Some(*curr_step)
            })
            .filter(|v| *v == 0)
            .count();
    }

    /// Make the negative inverse number modulo 100.
    ///
    /// E.g. if starting with 10, 10 - 30 = 80 and 10 + 30 = 40.
    /// And the negative inverse, -inv(10) = -90, so -90 - 30 = -20 and -90 + 30 = -60,
    /// where -inv(-20) = 80 and -inv(-60) = 40.
    fn make_inv(val: i32) -> i32 {
        if val == 0 {
            return val;
        }
        if val > 0 { -(100 - val) } else { 100 + val }
    }

    return inst
        .into_iter()
        .scan(50_i32, |curr_step, (dir, steps)| {
            // Turn the current step into its negative inverse if the sign doesn't match.
            // E.g. L30 (-30), therefore we need to have `curr_step < 0` so that we can divide
            // the number of wrapping out below.
            let curr = match (dir, *curr_step > 0) {
                // The signs match, leave it as-is.
                (Dir::Left, false) | (Dir::Right, true) => *curr_step,
                // The signs are opposite, get the negative inverse so that the signs match.
                _ => make_inv(*curr_step),
            };
            let next = match dir {
                Dir::Left => curr - steps,
                Dir::Right => curr + steps,
            };

            // Since we only have the cases neg - num, or pos + num (where num > 0) because of the logic above,
            // we can never end up with 0, therefore dividing by the modulo gives us the times
            // we would have wrapped (neg or positive).
            let amount_wraps = (next / 100).abs() as usize;

            println!("({dir:?}, {steps}), {curr} -> {next} ({amount_wraps})");

            // Map into the range -99 to 99 again, for the next steps.
            *curr_step = next % 100;

            Some(amount_wraps)
        })
        .sum();
}
