use anyhow::Context;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Register {
    A = 0,
    B,
    C,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Literal(u8),
    Register(Register),
    Reserved,
}

impl Operand {
    fn new(op: Op, val: u8) -> Self {
        fn make_combo(val: u8) -> Operand {
            match val {
                0..=3 => Operand::Literal(val),
                4 => Operand::Register(Register::A),
                5 => Operand::Register(Register::B),
                6 => Operand::Register(Register::C),
                7 => Operand::Reserved,
                _ => unreachable!(),
            }
        }

        match op {
            Op::Adv => make_combo(val),
            Op::Bxl => Operand::Literal(val),
            Op::Bst => make_combo(val),
            Op::Jnz => Operand::Literal(val),
            Op::Bxc => Operand::Reserved,
            Op::Out => make_combo(val),
            Op::Bdv => make_combo(val),
            Op::Cdv => make_combo(val),
        }
    }

    fn resolve(self, sys: &System) -> i64 {
        match self {
            Self::Literal(v) => v as i64,
            Self::Register(reg) => sys.regs[reg as usize],
            Self::Reserved => unreachable!("reserved is invalid"),
        }
    }

    fn literal(self) -> u8 {
        let Operand::Literal(val) = self else {
            panic!("not a literal");
        };
        val
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
#[allow(dead_code)]
enum Op {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Op {
    fn from(val: u8) -> Op {
        if val > Self::Cdv as u8 {
            panic!("invalid operation {val}");
        }
        unsafe { std::mem::transmute(val) }
    }

    fn exec(self, sys: &mut System, operand: Operand) {
        match self {
            Self::Adv => {
                let numerator = sys.regs[Register::A as usize];
                let denom = operand.resolve(sys);
                sys.regs[Register::A as usize] = if denom < 0 {
                    numerator << denom.abs()
                } else {
                    numerator >> denom.abs()
                };
            }
            Self::Bxl => {
                sys.regs[Register::B as usize] ^= operand.literal() as i64;
            }
            Self::Bst => {
                sys.regs[Register::B as usize] = ((operand.resolve(sys) as u64) % 8) as i64;
            }
            Self::Jnz => {
                if sys.regs[Register::A as usize] != 0 {
                    sys.pc = operand.resolve(sys).try_into().unwrap();
                    sys.pc /= 2;
                    return;
                }
            }
            Self::Bxc => {
                sys.regs[Register::B as usize] ^= sys.regs[Register::C as usize];
            }
            Self::Out => {
                sys.out.push(((operand.resolve(sys) as u64) % 8) as u8);
            }
            Self::Bdv => {
                let numerator = sys.regs[Register::A as usize];
                let denom = operand.resolve(sys);
                sys.regs[Register::B as usize] = if denom < 0 {
                    numerator << denom.abs()
                } else {
                    numerator >> denom.abs()
                };
            }
            Self::Cdv => {
                let numerator = sys.regs[Register::A as usize];
                let denom = operand.resolve(sys);
                sys.regs[Register::C as usize] = if denom < 0 {
                    numerator << denom.abs()
                } else {
                    numerator >> denom.abs()
                };
            }
        }

        sys.pc += 1;
    }
}

#[derive(Debug)]
struct System {
    pub regs: [i64; 3],
    pub pc: usize,
    pub out: Vec<u8>,
}

pub fn day17(data: &str, p1: bool) -> i64 {
    let lines = data.trim().lines().collect_vec();
    let (regs, program) = lines
        .split(|l| l.is_empty())
        .collect_tuple()
        .expect("registers then program");
    let (reg_a, reg_b, reg_c) = regs
        .into_iter()
        .map(|r| {
            let (_, val) = r.split_once(':').expect("register then `:` then val");
            val.trim()
                .parse::<i64>()
                .with_context(|| format!("invalid '{val}'"))
                .unwrap()
        })
        .collect_tuple()
        .expect("three register values");
    let program = program.join("");
    let (_, program) = program.split_once(':').expect("program then `:` then vals");
    let program_nums = program
        .trim()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect_vec();
    let program = program_nums
        .iter()
        .tuples()
        .map(|(&op, &operand)| {
            let op = Op::from(op);
            let operand = Operand::new(op, operand);
            (op, operand)
        })
        .collect_vec();

    println!("program: {program:?}");
    let mut sys = System {
        regs: [reg_a, reg_b, reg_c],
        pc: 0,
        out: Vec::new(),
    };
    println!("{sys:?}");

    if p1 {
        while let Some((op, operand)) = program.get(sys.pc).copied() {
            op.exec(&mut sys, operand);
        }

        println!("output: {}", sys.out.iter().join(","));
    } else {
        todo!()
    }
    0
}
