use std::char;
use std::num::ParseIntError;

use crossterm::event::KeyCode;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::FromExternalError;
use nom::{IResult, Parser};

use super::day12::Vec2;

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2,
    velocity: Vec2,
}

pub fn parse_i32<
    'a,
    E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, i32, E> {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::{map_res, opt, recognize};
    use nom::sequence::preceded;

    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)?;

    Ok((i, number))
}

impl Robot {
    fn parse(line: &str) -> anyhow::Result<Robot> {
        fn parse_vec(key: &str) -> impl FnMut(&str) -> IResult<&str, Vec2> + use<'_> {
            move |input: &str| {
                let (input, (x, y)) = nom::sequence::preceded(
                    nom::sequence::tuple((tag(key), tag("="))),
                    nom::sequence::separated_pair(parse_i32, tag(","), parse_i32),
                )(input)?;

                Ok((input, Vec2(x, y)))
            }
        }

        let (_, (pos, velocity)) = nom::sequence::terminated(
            nom::sequence::separated_pair(parse_vec("p"), tag(" "), parse_vec("v")),
            nom::combinator::eof,
        )
        .parse(line)
        .map_err(|e| e.to_owned())?;

        Ok(Robot { pos, velocity })
    }

    fn tick(&mut self, time: i32, width: u32, height: u32) {
        let (v_x, v_y) = if time < 0 {
            (-self.velocity.x(), -self.velocity.y())
        } else {
            (self.velocity.x(), self.velocity.y())
        };
        let time = time.abs() as u32;

        let w_1 = width - 1;
        let h_1 = height - 1;

        let (mut x, x_neg) = if v_x < 0 {
            let v_x = (-v_x) as u32;
            ((w_1 - self.pos.x() as u32) + time * v_x, true)
        } else {
            ((self.pos.x() as u32) + time * (v_x as u32), false)
        };
        let (mut y, y_neg) = if v_y < 0 {
            let v_y = (-v_y) as u32;
            ((h_1 - self.pos.y() as u32) + time * v_y, true)
        } else {
            ((self.pos.y() as u32) + time * (v_y as u32), false)
        };

        x %= width;
        y %= height;

        if x_neg {
            x = w_1 - x;
        }
        if y_neg {
            y = h_1 - y;
        }

        self.pos.0 = x as i32;
        self.pos.1 = y as i32;
    }
}

struct RobotsMap {
    width: u32,
    height: u32,
    pub map: Vec<u32>,
}

impl RobotsMap {
    pub fn new(robots: &[Robot], width: u32, height: u32) -> RobotsMap {
        let mut arr = vec![0_u32; (width * height) as usize];
        for r in robots {
            arr[(r.pos.x() + r.pos.y() * width as i32) as usize] += 1;
        }

        RobotsMap {
            map: arr,
            width,
            height,
        }
    }
}

impl std::fmt::Display for RobotsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_skip = if self.width % 2 == 0 {
            u32::MAX
        } else {
            self.width / 2
        };
        let y_skip = if self.height % 2 == 0 {
            u32::MAX
        } else {
            self.height / 2
        };

        for y in 0..self.height {
            if y == y_skip {
                continue;
            }
            let l = (0..self.width)
                .filter(|x| *x != x_skip)
                .map(|x| {
                    let v = self.map[(x + y * self.width) as usize];
                    if v != 0 {
                        char::from_digit(v as u32, 10).unwrap_or('?')
                    } else {
                        '.'
                    }
                })
                .join("");
            writeln!(f, "{l}")?;
        }
        Ok(())
    }
}

pub fn day14(data: &str, p1: bool) -> i64 {
    let mut robots: Vec<Robot> = data.lines().map(|l| Robot::parse(l)).try_collect().unwrap();

    let width = 101;
    let height = 103;

    if p1 {
        for r in robots.iter_mut() {
            r.tick(100, width, height);
        }
        fn count_where(robots: &[Robot], mut f: impl FnMut(&Vec2) -> bool) -> usize {
            robots.iter().filter(|r| f(&r.pos)).count()
        }

        println!("{}", RobotsMap::new(&robots, width, height));

        let x_l = width / 2;
        let x_r = if width % 2 == 0 { x_l } else { x_l + 1 };
        let y_l = height / 2;
        let y_r = if height % 2 == 0 { y_l } else { y_l + 1 };

        let top_left = count_where(&robots, |p| p.x() < x_l as i32 && p.y() < y_l as i32);
        let top_right = count_where(&robots, |p| p.x() >= x_r as i32 && p.y() < y_l as i32);
        let bottom_left = count_where(&robots, |p| p.x() < x_l as i32 && p.y() >= y_r as i32);
        let bottom_right = count_where(&robots, |p| p.x() >= x_r as i32 && p.y() >= y_r as i32);

        (dbg!(top_left) * dbg!(top_right) * dbg!(bottom_left) * dbg!(bottom_right)) as i64
    } else {
        let mut t = 7285_usize;
        for r in robots.iter_mut() {
            r.tick(t as i32, width, height);
        }
        loop {
            t += 1;
            for r in robots.iter_mut() {
                r.tick(1, width, height);
            }

            let ms1 = RobotsMap::new(&robots, width, height).to_string();
            let mut robots2 = robots.clone();

            for r in robots2.iter_mut() {
                r.tick(1, width, height);
            }
            let ms2 = RobotsMap::new(&robots2, width, height).to_string();
            for r in robots2.iter_mut() {
                r.tick(1, width, height);
            }

            let ms3 = RobotsMap::new(&robots2, width, height).to_string();

            println!(
                "{}",
                ms1.lines()
                    .zip(ms2.lines())
                    .zip(ms3.lines())
                    .map(|((l_1, l_2), l_3)| { [l_1, l_2, l_3].join("    ") })
                    .join("\n")
            );
            println!("time = {}", t);

            crossterm::terminal::enable_raw_mode().unwrap();
            while let Ok(evt) = crossterm::event::read() {
                match evt {
                    crossterm::event::Event::Key(crossterm::event::KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    }) => break,
                    crossterm::event::Event::Key(crossterm::event::KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    }) => {
                        for r in robots.iter_mut() {
                            r.tick(-2, width, height);
                        }
                        t -= 2;
                        break;
                    }
                    crossterm::event::Event::Key(crossterm::event::KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    }) => {
                        crossterm::terminal::disable_raw_mode().unwrap();
                        return t as i64;
                    }
                    e => println!("{e:?}"),
                }
            }
            crossterm::terminal::disable_raw_mode().unwrap();
        }
    }
}
