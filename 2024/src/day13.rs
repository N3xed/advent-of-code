use super::day12::Dir;
use itertools::Itertools;
use tqdm::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec2(pub i64, pub i64);

#[allow(dead_code)]
impl Vec2 {
    pub fn is_in_bounds(self, width: usize, height: usize) -> bool {
        (self.x() as i64) < (width as i64)
            && (self.y() as i64) < (height as i64)
            && self.x() >= 0
            && self.y() >= 0
    }

    pub fn to_idx(self, width: usize) -> usize {
        (self.0 as usize) + (self.1 as usize) * width
    }

    pub fn offset(&self, x: i64, y: i64) -> Vec2 {
        Vec2(self.x() + x, self.y() + y)
    }

    pub fn x(&self) -> i64 {
        self.0
    }
    pub fn y(&self) -> i64 {
        self.1
    }

    pub fn neighbors(&self) -> impl Iterator<Item = (Self, Dir)> {
        [
            (self.offset(0, -1), Dir::Up),
            (self.offset(1, 0), Dir::Right),
            (self.offset(0, 1), Dir::Down),
            (self.offset(-1, 0), Dir::Left),
        ]
        .into_iter()
    }
}

#[derive(Debug)]
struct ClawCfg {
    button_a: Vec2,
    button_b: Vec2,
    prize_pos: Vec2,
}

impl ClawCfg {
    fn find_min_cost(&self, a_cost: i64, b_cost: i64) -> Option<(i64, i64, i64)> {
        let max_cnt = self.prize_pos.x().max(self.prize_pos.y())
            / self
                .button_a
                .x()
                .min(self.button_b.x())
                .min(self.button_a.y())
                .min(self.button_b.y());
        let min_cnt = self.prize_pos.x().min(self.prize_pos.y())
            / (self
                .button_a
                .x()
                .max(self.button_a.y())
                .max(self.button_b.x())
                .max(self.button_b.y()));

        (min_cnt..=max_cnt)
            .flat_map(|u| {
                (1..=u).filter_map(move |a| {
                    let b = u - a;

                    let x = a * self.button_a.x() + b * self.button_b.x();
                    let y = a * self.button_a.y() + b * self.button_b.y();

                    if x == self.prize_pos.x() && y == self.prize_pos.y() {
                        let cost = a_cost * a + b_cost * b;

                        return Some((a, b, cost));
                    }
                    None
                })
            })
            .min_by_key(|(_, _, c)| *c)
    }
}

pub fn day13(data: &str, p1: bool) -> i64 {
    let lines = data.lines().map(str::trim).collect_vec();
    let mut cfgs = lines
        .split(|l| l.is_empty())
        .map(|lines| {
            let (btn_a_line, btn_b_line, prize_line) =
                lines.into_iter().collect_tuple().expect("3 text lines");

            fn parse_button(l: &str, btn: &str) -> Vec2 {
                let (_btn, c, x, y) = l.split(' ').collect_tuple().expect("button format");
                assert_eq!(c, btn);
                let x = x.strip_suffix(',').expect("comma after `X+<num>`");

                let (_, x) = x.trim().split_once("+").expect("X+<number> format");
                let (_, y) = y.trim().split_once("+").expect("X+<number> format");

                let x = x.parse::<i64>().expect("x is number");
                let y = y.parse::<i64>().expect("y is number");
                Vec2(x, y)
            }

            let button_a = parse_button(btn_a_line, "A:");
            let button_b = parse_button(btn_b_line, "B:");

            let (prize, prize_x, prize_y) =
                prize_line.split(' ').collect_tuple().expect("prize line");
            assert_eq!(prize, "Prize:");
            let prize_x = prize_x
                .strip_suffix(',')
                .unwrap()
                .strip_prefix("X=")
                .unwrap();
            let prize_y = prize_y.strip_prefix("Y=").unwrap();

            let prize_x = prize_x.parse::<i64>().expect("prize x must be number");
            let prize_y = prize_y.parse::<i64>().expect("prize y must be number");
            let prize_pos = Vec2(prize_x, prize_y);

            ClawCfg {
                button_a,
                button_b,
                prize_pos,
            }
        })
        .collect_vec();

    if !p1 {
        for cfg in cfgs.iter_mut() {
            cfg.prize_pos = cfg.prize_pos.offset(10000000000000, 10000000000000);
        }
        unimplemented!("problem 2");
    }

    let result: i64 = cfgs
        .iter()
        .tqdm()
        .filter_map(|cfg| cfg.find_min_cost(3, 1))
        .map(|(_, _, c)| c)
        .sum();

    result as i64
}
