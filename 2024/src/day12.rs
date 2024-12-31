use itertools::Itertools;

#[derive(Clone, Debug)]
struct Square {
    id: char,
    visited: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Vec2(pub i32, pub i32);
impl Vec2 {
    pub fn is_in_bounds(self, width: usize, height: usize) -> bool {
        (self.x() as i64) < (width as i64)
            && (self.y() as i64) < (height as i64)
            && self.x() >= 0
            && self.y() >= 0
    }

    pub fn to_idx(self, width: usize) -> usize {
        (self.0 as usize).saturating_add((self.1 as usize).saturating_mul(width))
    }

    pub fn offset(&self, x: i32, y: i32) -> Vec2 {
        Vec2(self.x() + x, self.y() + y)
    }

    pub fn offset_vec(mut self, offset: Vec2) -> Self {
        self.0 += offset.0;
        self.1 += offset.1;
        self
    }

    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
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
struct Plot {
    perimeter: u32,
    area: u32,
    sides: u32,

    #[allow(dead_code)]
    id: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Dir {
    /// Turn this direction 90° clockwise.
    pub fn turn(self) -> Dir {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub const ALL: [Dir; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];
    pub const N: usize = 4;

    pub fn turn_n(self, times: i32) -> Dir {
        let mut idx = ((self as i32) + times) % (Self::N as i32);
        if idx < 0 {
            idx = (Self::N as i32) + idx;
        }
        Self::ALL[idx as usize]
    }

    pub fn opposite(self) -> Dir {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn into_vec2(self) -> Vec2 {
        match self {
            Self::Up => Vec2(0, -1),
            Self::Right => Vec2(1, 0),
            Self::Down => Vec2(0, 1),
            Self::Left => Vec2(-1, 0),
        }
    }
}

#[derive(Debug)]
struct Side {
    pos: Vec2,
    dir: Dir,
}

fn find_plots(squares: &[Square], width: usize, height: usize) -> Vec<Plot> {
    let mut squares = squares.iter().cloned().collect_vec();
    let mut result = Vec::new();

    let mut plot_squares = Vec::new();
    let mut sides = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pos = Vec2(x as i32, y as i32);
            let idx = pos.to_idx(width);

            let sq = unsafe { squares.get_unchecked(idx).clone() };
            if sq.visited {
                continue;
            }

            let mut plot = Plot {
                perimeter: 0,
                area: 0,
                sides: 0,
                id: sq.id,
            };
            plot_squares.clear();
            sides.clear();

            // Find all `Square`s in a 4-neighbor connected component starting from `pos`.
            // During this flood-fill, also calculate area, perimiter and collect all
            // boundary edges.
            plot_squares.push((pos, sq, idx));
            while let Some((pos, sq, idx)) = plot_squares.pop() {
                let sq_ref = unsafe { squares.get_unchecked_mut(idx) };
                if sq_ref.visited {
                    continue;
                }
                sq_ref.visited = true;
                plot.area += 1;

                for (n_pos, n_dir) in pos.neighbors() {
                    if !n_pos.is_in_bounds(width, height) {
                        plot.perimeter += 1;
                        sides.push(Side {
                            pos: n_pos,
                            dir: n_dir,
                        });
                        continue;
                    }

                    let n_idx = n_pos.to_idx(width);
                    let n_sq = unsafe { squares.get_unchecked(n_idx).clone() };

                    let is_other = n_sq.id != sq.id;
                    plot.perimeter += is_other as u32;
                    if is_other {
                        sides.push(Side {
                            pos: n_pos,
                            dir: n_dir,
                        });
                    }

                    if !n_sq.visited && !is_other {
                        plot_squares.push((n_pos, n_sq, n_idx));
                    }
                }
            }
            sides.sort_unstable_by_key(|s| s.dir);
            fn pos_dir_coord_predicate(dir: Dir) -> fn(&Side) -> i32 {
                match dir {
                    Dir::Up | Dir::Down => |s: &Side| s.pos.y(),
                    Dir::Left | Dir::Right => |s: &Side| s.pos.x(),
                }
            }
            plot.sides = sides
                .drain(..)
                // First group by side direction (i.e. up = the side is on the top the of square).
                .sorted_unstable_by_key(|s| s.dir)
                .chunk_by(|s| s.dir)
                .into_iter()
                .map(|(dir, group)| {
                    let mut group = group.collect_vec();

                    // Then, for each direction, group on the coordinate of that direction:
                    // When dir = Dir::Up, the sides are on top, so we want all lines.
                    // The resulting groups have all a common y coordinate.
                    let pred = pos_dir_coord_predicate(dir);
                    group.sort_unstable_by_key(pred);
                    let groups = group
                        .into_iter()
                        .chunk_by(pred)
                        .into_iter()
                        .map(|(_, g)| g.collect_vec())
                        .collect_vec();
                    groups
                        .into_iter()
                        .map(move |mut chunk| {
                            // Finally, group consecutive coordinates opposite of the previous
                            // direction into one side.
                            // E.g. when dir = Dir::Up, `chunk` contains all `Side`s with
                            // the same y coordinate, group all consecutive x coordinates into
                            // one side, and count the resulting sides: `0, 1, 2, 4, 5, 10` =>
                            // `[0, 1, 2], [4, 5], [10]` => 3 sides.
                            let pred = pos_dir_coord_predicate(dir.turn());
                            chunk.sort_unstable_by_key(pred);
                            chunk
                                .into_iter()
                                .zip(0_i32..)
                                .chunk_by(|(s, i)| pred(s) - i)
                                .into_iter()
                                .count()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>() as u32;

            result.push(plot);
        }
    }

    result
}

pub fn day12(data: &str, _p1: bool) -> i64 {
    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| Square {
                    id: c,
                    visited: false,
                })
                .collect_vec()
        })
        .collect_vec();

    let width = map.first().expect("at least one line").len();
    assert!(map.iter().all(|l| l.len() == width));
    let height = map.len();

    let map = map.into_iter().flatten().collect_vec();
    let plots = find_plots(&map, width, height);

    let result: u64 = if _p1 {
        plots.iter().map(|p| (p.perimeter * p.area) as u64).sum()
    } else {
        plots.iter().map(|p| (p.sides * p.area) as u64).sum()
    };

    // println!("{plots:#?}");

    result as i64
}
