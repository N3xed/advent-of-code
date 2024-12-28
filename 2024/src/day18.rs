use itertools::Itertools;

use crate::day12::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Loc<T> {
    Empty = 0,
    Obstacle,
    Path,
    Custom(T),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DeadEnd;
impl std::fmt::Display for DeadEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X")
    }
}

pub mod shortest_path {
    pub use super::Loc;
    pub use crate::day12::{Dir, Vec2};

    #[derive(Debug, Clone)]
    pub struct Node([(Dir, u32); 4]);
    impl Node {
        pub fn new() -> Self {
            Node([
                (Dir::Up, u32::MAX),
                (Dir::Right, u32::MAX),
                (Dir::Down, u32::MAX),
                (Dir::Left, u32::MAX),
            ])
        }

        /// Returns `true` if the path from `went_to` was shorter than previously best path.
        /// Otherwise returns `false`.
        pub fn update_with(&mut self, went_to: Dir, steps: u32) -> bool {
            let val = &mut self.0[went_to.opposite() as usize];
            if val.1 > steps {
                val.1 = steps;
                true
            } else {
                false
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ShortestPath {
        map: Vec<Node>,
        width: usize,
        height: usize,
        start: Vec2,
    }

    impl ShortestPath {
        pub fn new(start: Vec2, width: usize, height: usize) -> Self {
            Self {
                map: Vec::new(),
                width,
                height,
                start,
            }
        }

        /// Find all shortest paths between [`Self::start`] and all empty tiles in the map.
        pub fn calc<T>(&mut self, obst_map: &[Loc<T>])
        where
            T: Clone + Eq,
        {
            let Self {
                map,
                width,
                height,
                start,
            } = self;
            let width = *width;
            let height = *height;
            let start = *start;

            *map = vec![Node::new(); width * height];
            let mut paths = vec![(start, 0_u32)];
            let mut new_paths = Vec::<(Vec2, u32)>::new();

            while !paths.is_empty() {
                paths.retain_mut(|(pos, steps)| {
                    *steps += 1;
                    let mut found_one = false;
                    for (next_pos, dir) in pos
                        .neighbors()
                        .filter(|(pos, _)| pos.is_in_bounds(width, height))
                        .filter(|(pos, _)| obst_map[pos.to_idx(width)] != Loc::Obstacle)
                    {
                        if map[next_pos.to_idx(width)].update_with(dir, *steps) {
                            if found_one {
                                new_paths.push((next_pos, *steps));
                            } else {
                                found_one = true;
                                *pos = next_pos;
                            }
                        }
                    }
                    found_one
                });
                paths.extend(new_paths.drain(..));
            }
        }

        /// Get the amount of steps needed to `end`, return [`None`] if there is no path to `end`.
        pub fn steps_to(&self, end: Vec2) -> Option<u32> {
            self.map
                .get(end.to_idx(self.width))?
                .0
                .iter()
                .map(|(_, s)| *s)
                .filter(|&s| s != u32::MAX)
                .min()
        }

        /// Get the path from [`Self::start`] to `end` as a series (point, number of steps at that point) pairs.
        pub fn get_path(&self, end: Vec2) -> Vec<(Vec2, u32)> {
            let mut pos = end;
            let start = self.start;
            let width = self.width;
            let height = self.height;
            let mut positions = Vec::new();
            while (pos.x() != start.x() || pos.y() != start.y()) && pos.is_in_bounds(width, height)
            {
                let idx = pos.to_idx(width);
                let (dir, steps) = *self.map[idx].0.iter().min_by_key(|(_, s)| s).unwrap();
                if steps == u32::MAX {
                    return positions;
                }
                positions.push((pos, steps));
                pos = pos.offset_vec(dir.into_vec2());
            }
            positions.push((self.start, 0));
            positions.reverse();
            positions
        }

        /// Get the path from [`Self::start`] to `end` as a series
        /// (point, number of steps at that point, next travel direction) pairs.
        pub fn get_path_and_dirs(&self, end: Vec2) -> Option<Vec<(Vec2, u32, Dir)>> {
            let mut pos = end;
            let start = self.start;
            let width = self.width;
            let height = self.height;
            let mut last_dir = Dir::Up;
            let mut positions = Vec::new();
            while (pos.x() != start.x() || pos.y() != start.y()) && pos.is_in_bounds(width, height)
            {
                let idx = pos.to_idx(width);
                let (dir, steps) = *self.map[idx].0.iter().min_by_key(|(_, s)| s).unwrap();
                if steps == u32::MAX {
                    return None;
                }
                positions.push((pos, steps, last_dir));
                last_dir = dir.opposite();
                pos = pos.offset_vec(dir.into_vec2());
            }
            positions.push((self.start, 0, last_dir));
            positions.reverse();
            Some(positions)
        }

        /// Fill the map with the found path from [`Self::start`] to `end`.
        pub fn fill_path<T>(&self, path_map: &mut Vec<Loc<T>>, end: Vec2) {
            let mut pos = end;
            let start = self.start;
            let width = self.width;
            let height = self.height;
            while (pos.x() != start.x() || pos.y() != start.y()) && pos.is_in_bounds(width, height)
            {
                let idx = pos.to_idx(width);
                let (dir, steps) = *self.map[idx].0.iter().min_by_key(|(_, s)| s).unwrap();
                path_map[idx] = Loc::Path;
                if steps == u32::MAX {
                    return;
                }
                pos = pos.offset_vec(dir.into_vec2());
            }
            path_map[start.to_idx(width)] = Loc::Path;
        }
    }

    pub fn print_map<T>(m: &Vec<Loc<T>>, width: usize, height: usize)
    where
        T: std::fmt::Display,
    {
        for y in 0..height as usize {
            for x in 0..width as usize {
                let c = match &m[x + y * width] {
                    Loc::Obstacle => '#',
                    Loc::Path => '.',
                    Loc::Custom(v) => {
                        print!("{v}");
                        continue;
                    }
                    _ => ' ',
                };
                print!("{c}");
            }
            println!();
        }
    }
}
use shortest_path::*;

pub fn day18(data: &str, p1: bool) -> i64 {
    let mut positions = data
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some((x.trim().parse::<u32>().ok()?, y.trim().parse::<u32>().ok()?))
        })
        .collect_vec();

    let size: u32 = 71;
    let start = Vec2(0, 0);
    let end = Vec2(size as i32 - 1, size as i32 - 1);

    let mut obst_map: Vec<Loc<DeadEnd>> =
        std::iter::repeat_n(Loc::Empty, (size * size) as usize).collect_vec();
    for (x, y) in positions.drain(..1024) {
        obst_map[(x + y * size) as usize] = Loc::Obstacle;
    }

    let mut sp = ShortestPath::new(start, size as usize, size as usize);
    sp.calc(&obst_map);
    let steps = sp.steps_to(end).unwrap();

    let mut path_map = obst_map.clone();
    sp.fill_path(&mut path_map, end);
    if p1 {
        print_map(&path_map, size as usize, size as usize);
        return steps as i64;
    } else {
        let mut prev_path_map = Vec::new();
        let mut final_pos = None;
        for (x, y) in positions.drain(..) {
            let idx = (x + y * size) as usize;
            obst_map[idx] = Loc::Obstacle;

            // If the previous path gets obstructed, find a new path.
            // When there is no path anymore, that obstruction position is the answer.
            if path_map[idx] == Loc::Path {
                prev_path_map = path_map;

                sp.calc(&obst_map);
                path_map = obst_map.clone();
                sp.fill_path(&mut path_map, end);

                let maybe_steps = sp.steps_to(end);
                if maybe_steps.is_none() {
                    prev_path_map[idx] = Loc::Custom(DeadEnd);
                    final_pos = Some((x, y));
                    break;
                }
            }
        }

        print_map(&prev_path_map, size as usize, size as usize);
        println!("answer = {final_pos:?}");

        return 0;
    };
}
