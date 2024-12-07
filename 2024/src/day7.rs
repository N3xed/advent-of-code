use itertools::Itertools;

#[derive(Clone, Debug)]
struct Equation<'n> {
    ans: i64,
    nums: &'n [i64],
}

impl Equation<'_> {
    fn print_eq(&self, ops: impl IntoIterator<Item = Op>, ans: Option<i64>) {
        let eq = ops
            .into_iter()
            .zip(&self.nums[0..self.nums.len() - 1])
            .map(|(op, n)| format!("{n}{op}"))
            .chain([self.nums.last().unwrap().to_string()])
            .join("");

        if let Some(ans) = ans {
            println!("{eq} = {ans}{}", if ans == self.ans { " ✓" } else { "" });
        } else {
            println!("{}: {eq}", self.ans);
        }
    }

    /// Whether it is possible to arrive at `self.ans` with at least one combination of the
    /// set of operators `ops` applied to the numbers `self.nums`.
    ///
    /// Returns the list of operators used if it is possible, otherwise returns `None`.
    fn is_possible(&self, ops: &[Op]) -> Option<Vec<Op>> {
        (0..self.nums.len() - 1)
            .map(|_| ops.iter())
            .multi_cartesian_product()
            .filter_map(|v| {
                let ans = self
                    .nums
                    .iter()
                    .skip(1)
                    .zip(&v)
                    .fold(self.nums[0], |lhs, (rhs, op)| op.apply(lhs, *rhs));

                if ans == self.ans {
                    Some(v.into_iter().copied().collect_vec())
                } else {
                    None
                }
            })
            .next()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
            Self::Concat => {
                let rhs_digits = if rhs == 0 {
                    1
                } else {
                    (rhs.abs() as f64).log10().floor() as u32 + 1
                };

                lhs * 10_i64.pow(rhs_digits) + rhs
            }
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Add => "+",
            Self::Multiply => "*",
            Self::Concat => "||",
        };
        f.write_str(s)
    }
}

pub fn day7(data: &str, p1: bool) -> i64 {
    let (ans, nums): (Vec<(i64, usize)>, Vec<Vec<i64>>) = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (ans, nums) = l.split_once(':').expect("equation on each line");

            let ans = ans.trim().parse::<i64>().expect("answer must be a number");
            let nums = nums
                .trim()
                .split(' ')
                .map(|n| n.parse::<i64>().expect("numbers"))
                .collect_vec();

            ((ans, nums.len()), nums)
        })
        .unzip();

    let nums = nums
        .into_iter()
        .map(|n| n.into_iter())
        .flatten()
        .collect_vec();
    let eqs = ans
        .into_iter()
        .scan(0, |sum, (ans, len)| {
            let idx = *sum;
            *sum += len;

            let range = idx..*sum;
            Some(Equation {
                ans,
                nums: &nums[range],
            })
        })
        .collect_vec();

    let operators: &[Op] = if p1 {
        &[Op::Add, Op::Multiply]
    } else {
        &[Op::Add, Op::Multiply, Op::Concat]
    };

    let result: i64 = eqs
        .into_iter()
        .filter_map(|eq| {
            let ops = eq.is_possible(operators)?;
            eq.print_eq(ops, Some(eq.ans));
            Some(eq.ans)
        })
        .sum();

    result as i64
}
