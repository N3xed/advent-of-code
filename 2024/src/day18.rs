use itertools::Itertools;

use crate::day12::{Dir, Vec2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Loc {
    Empty = 0,
    Obstacle,
    Path,
    DeadEnd,
}

pub fn day18(data: &str, p1: bool) -> i64 {
    let mut positions = data
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some((x.trim().parse::<u32>().ok()?, y.trim().parse::<u32>().ok()?))
        })
        .collect_vec();

    let size: u32 = 71;

    let mut obst_map: Vec<Loc> =
        std::iter::repeat_n(Loc::Empty, (size * size) as usize).collect_vec();
    for (x, y) in positions.drain(..1024) {
        obst_map[(x + y * size) as usize] = Loc::Obstacle;
    }

    fn find_shortest_path(obst_map: &mut Vec<Loc>, size: u32) -> Option<u32> {
        #[derive(Debug, Clone)]
        struct Node([(Dir, u32); 4]);
        impl Node {
            fn new() -> Self {
                Node([
                    (Dir::Up, u32::MAX),
                    (Dir::Right, u32::MAX),
                    (Dir::Down, u32::MAX),
                    (Dir::Left, u32::MAX),
                ])
            }

            /// Returns `true` if the path from `went_to` was shorter than previously best path.
            /// Otherwise returns `false`.
            fn update_with(&mut self, went_to: Dir, steps: u32) -> bool {
                let val = &mut self.0[went_to.opposite() as usize];
                if val.1 > steps {
                    val.1 = steps;
                    true
                } else {
                    false
                }
            }
        }

        let mut map = vec![Node::new(); (size * size) as usize];
        let mut paths = vec![(Vec2(0, 0), 0_u32)];
        let mut new_paths = Vec::<(Vec2, u32)>::new();

        while !paths.is_empty() {
            paths.retain_mut(|(pos, steps)| {
                *steps += 1;
                let mut found_one = false;
                for (next_pos, dir) in pos
                    .neighbors()
                    .filter(|(pos, _)| pos.is_in_bounds(size as usize, size as usize))
                    .filter(|(pos, _)| obst_map[pos.to_idx(size as usize)] != Loc::Obstacle)
                {
                    if map[next_pos.to_idx(size as usize)].update_with(dir, *steps) {
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

        let bottom_right_idx = ((size - 1) + (size - 1) * size) as usize;
        let Some(min_steps) = map[bottom_right_idx]
            .0
            .iter()
            .map(|(_, s)| *s)
            .filter(|&s| s != u32::MAX)
            .min()
        else {
            return None;
        };

        // Fill the map with the found path.
        let mut pos = Vec2((size - 1) as i32, (size - 1) as i32);
        while (pos.x() != 0 || pos.y() != 0) && pos.is_in_bounds(size as usize, size as usize) {
            let idx = pos.to_idx(size as usize);
            obst_map[idx] = Loc::Path;
            let (dir, _) = map[idx].0.iter().min_by_key(|(_, s)| s).unwrap();
            pos = pos.offset_vec(dir.into_vec2());
        }
        obst_map[0] = Loc::Path;

        Some(min_steps)
    }

    fn print_map(m: &Vec<Loc>, size: u32) {
        for y in 0..size as usize {
            for x in 0..size as usize {
                let c = match m[x + y * size as usize] {
                    Loc::Obstacle => '#',
                    Loc::Path => '.',
                    Loc::DeadEnd => 'X',
                    _ => ' ',
                };
                print!("{c}");
            }
            println!();
        }
    }

    let mut path_map = obst_map.clone();
    let steps = find_shortest_path(&mut path_map, size).unwrap();

    if p1 {
        print_map(&path_map, size);
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
                path_map = obst_map.clone();
                let maybe_steps = find_shortest_path(&mut path_map, size);
                if maybe_steps.is_none() {
                    prev_path_map[idx] = Loc::DeadEnd;
                    final_pos = Some((x, y));
                    break;
                }
            }
        }

        print_map(&prev_path_map, size);
        println!("answer = {final_pos:?}");

        return 0;
    };
}
