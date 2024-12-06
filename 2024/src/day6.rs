use enumflags2::{bitflags, BitFlags};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Loc {
    Nothing { visited: bool, dir: BitFlags<Dir> },
    Obstacle,
    Pos,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[bitflags]
#[repr(u8)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn step(&self, x: i32, y: i32) -> (i32, i32) {
        match *self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }

    fn turn(&mut self) {
        *self = match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }
}

/// Whether the `x`, `y` position is in the map bounds.
fn is_in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as isize) < (width as isize) && (y as isize) < (height as isize)
}

#[derive(Clone)]
struct Map {
    pub map: Vec<Loc>,
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    dir: Dir,
}

impl Map {
    fn new(map: Vec<Loc>, x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            map,
            width,
            height,
            x,
            y,
            dir: Dir::Up,
        }
    }

    /// Get the next position and direction of the agent.
    /// The agent may turn multiple times but only step once.
    fn next_pos(&self) -> Result<(i32, i32, Dir), bool> {
        let mut x = self.x;
        let mut y = self.y;
        let mut dir = self.dir;
        for _ in 0..4 {
            let (x_temp, y_temp) = dir.step(x, y);
            if !is_in_bounds(x_temp, y_temp, self.width, self.height) {
                return Err(false);
            }

            let loc = self.map[(x_temp + y_temp * self.width as i32) as usize];
            if !matches!(loc, Loc::Nothing { .. }) {
                dir.turn();
            } else {
                x = x_temp;
                y = y_temp;

                return Ok((x, y, dir));
            }
        }
        // Detected a cycle by turning four times in a row.
        Err(true)
    }

    /// Step the agent once.
    ///
    /// Return `Some(true)` if a cycle was detected, `Some(false)` if the agent went out of bounds.
    /// Otherwise `None`, the path did not terminate yet.
    fn step(&mut self) -> Option<bool> {
        let idx = (self.x + self.y * self.width as i32) as usize;

        match &mut self.map[idx] {
            Loc::Nothing {
                visited: true,
                dir: d,
            } if d.contains(self.dir) => {
                return Some(true);
            }
            Loc::Nothing { visited, dir } => {
                *visited = true;
                *dir |= self.dir;
            }
            val => {
                *val = Loc::Nothing {
                    visited: true,
                    dir: self.dir.into(),
                }
            }
        };

        (self.x, self.y, self.dir) = match self.next_pos() {
            // Detected termination condition.
            Err(val) => return Some(val),
            // Stepped successfully.
            Ok(val) => val,
        };

        None
    }

    /// Fill out the whole path.
    ///
    /// Return `true` if a cycle was detected, `false` if the agent went out of bounds.
    fn fill(&mut self) -> bool {
        loop {
            let res = self.step();
            if let Some(v) = res {
                return v;
            }
        }
    }

    fn print(&self, start_idx: usize) {
        println!("filled out map:");
        for l in &self.map.iter().enumerate().chunks(self.width) {
            let line = l
                .map(|(i, loc)| {
                    if i == start_idx {
                        return '^';
                    }
                    match loc {
                        Loc::Nothing { visited: false, .. } => '.',
                        Loc::Nothing { visited: true, dir }
                            if (*dir & (Dir::Up | Dir::Down)).is_empty() =>
                        {
                            '-'
                        }
                        Loc::Nothing { visited: true, dir }
                            if (*dir & (Dir::Left | Dir::Right)).is_empty() =>
                        {
                            '|'
                        }
                        Loc::Nothing { visited: true, .. } => '+',
                        Loc::Pos => 'X',
                        Loc::Obstacle => '#',
                    }
                })
                .join("");

            println!("{line}");
        }
    }
}

pub fn day6(data: &str, p1: bool) -> i64 {
    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Loc::Nothing {
                        visited: false,
                        dir: BitFlags::empty(),
                    },
                    '#' => Loc::Obstacle,
                    '^' => Loc::Pos,
                    c => panic!("unexpected char '{c}'"),
                })
                .collect_vec()
        })
        .collect_vec();

    let width = map.get(0).expect("map").len();
    let height = map.len();

    assert!(map.iter().all(|l| l.len() == width));
    let map = map
        .into_iter()
        .map(|l| l.into_iter())
        .flatten()
        .collect_vec();

    let pos_idx = map
        .iter()
        .position(|v| *v == Loc::Pos)
        .expect("position in map");

    let pos_x = pos_idx % width;
    let pos_y = pos_idx / width;

    println!("pos at [{pos_x}, {pos_y}]");

    let result = if p1 {
        let mut map = Map::new(map, pos_x as i32, pos_y as i32, width, height);
        map.fill();
        let result: usize = map
            .map
            .iter()
            .filter(|loc| matches!(*loc, Loc::Nothing { visited: true, .. }))
            .count();
        map.print(pos_idx);

        result
    } else {
        let initial_map = Map::new(map, pos_x as i32, pos_y as i32, width, height);
        let mut map = initial_map.clone();

        let mut obstruction_positions = Vec::new();
        loop {
            let (x, y, _) = match map.next_pos() {
                Err(true) => unimplemented!("turning-cycles are not implemented"),
                Err(false) => {
                    // Agent went outside of bounds, we are finished.
                    break;
                }
                Ok(val) => val,
            };

            let obstruction_idx = (x + y * width as i32) as usize;

            if !obstruction_positions.contains(&obstruction_idx) && obstruction_idx != pos_idx {
                let mut temp_map = initial_map.clone();

                temp_map.map[obstruction_idx] = Loc::Obstacle;
                let is_cycle = temp_map.fill();

                if is_cycle {
                    // temp_map.print(pos_idx);

                    obstruction_positions.push(obstruction_idx);
                }
            }

            if map.step().is_some() {
                break;
            }
        }

        obstruction_positions.len()
    };

    result as i64
}
