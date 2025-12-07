use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '*' => Self::Mul,
            op => panic!("invalid op '{op}'"),
        }
    }
}

impl Op {
    fn get_char(self) -> char {
        match self {
            Self::Add => '+',
            Self::Mul => '*',
        }
    }

    fn format_numbers(self, nums: &[i64]) -> String {
        format!(
            "{}({})",
            self.get_char(),
            nums.iter().map(ToString::to_string).join(", ")
        )
    }

    fn apply(self, nums: &[i64]) -> i64 {
        match self {
            Self::Add => nums.iter().sum(),
            Self::Mul => nums.iter().copied().fold(1, |acc, v| acc * v),
        }
    }
}

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let lines = data
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .collect_vec();

    // Get operators from the last line.
    let ops = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, c)| (i, Op::from(c)))
        .collect_vec();

    if p1 {
        let mut columns = Vec::new();
        // Each column of numbers in data is a problem, so transpose.
        for (i, n) in lines[..lines.len() - 1].iter().flat_map(|line| {
            line.split_whitespace()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<i64>().unwrap())
                .enumerate()
        }) {
            let v = match columns.get_mut(i) {
                None => {
                    columns.push(Vec::new());
                    columns.last_mut().unwrap()
                }
                Some(v) => v,
            };
            v.push(n);
        }

        let mut total = 0_i64;
        for (col, (_, op)) in columns.into_iter().zip(ops) {
            let res = op.apply(&col);
            println!("{} = {res}", op.format_numbers(&col));
            total += res;
        }
        return total;
    }

    let mut op_indices = ops.iter().map(|(i, _)| *i).collect_vec();
    op_indices.push(lines.iter().map(|l| l.len()).max().unwrap());
    let op_ranges = op_indices
        .iter()
        .copied()
        .tuple_windows::<(_, _)>()
        .collect_vec();

    let mut columns = Vec::new();

    // In addition to the problem columns, each digit is also a column,
    // where the digit in the first row is the most significant digit
    // and the digit in the last (non-empty) row is the least significant digit
    // of a single number.
    // The amount of numbers in a problem is determined by the maximum amount of (non-empty) digit
    // columns.
    for (i, n) in lines[..lines.len() - 1].iter().flat_map(|line| {
        // Slice each number of the current row by the ranges determined by the operators,
        // this is possible since all numbers are aligned (and so are the operators).
        op_ranges
            .iter()
            .copied()
            .enumerate() // enumerate columns
            .map(|(i, (a, b))| (i, &line[a..b.min(line.len())]))
    }) {
        // Get the right column vector.
        let v = match columns.get_mut(i) {
            None => {
                columns.push(Vec::new());
                columns.last_mut().unwrap()
            }
            Some(v) => v,
        };

        // Each digit `j` of the current number `i` within a column in the data with some row index
        // `r` is actually the `r`-th digit of the `j`-th number within that column i.
        for (j, digit) in n.bytes().enumerate() {
            let c = char::from(digit);
            if c.is_ascii_whitespace() {
                continue;
            }
            let digit = (digit - b'0') as i64;

            // The number we have to modify is determined by the character position
            // within the current column.
            let new_len = j + 1;
            if v.len() < new_len {
                v.resize(new_len, 0);
            }
            let val = &mut v[j];

            // Build the number digit by digit.
            *val = *val * 10 + digit;
        }
    }

    let mut total = 0_i64;
    for (col, (_, op)) in columns.into_iter().zip(ops) {
        let res = op.apply(&col);
        println!("{} = {res}", op.format_numbers(&col));
        total += res;
    }
    return total;
}
