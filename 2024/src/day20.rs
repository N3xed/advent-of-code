use std::collections::HashMap;

use itertools::Itertools;

use crate::{day12::Vec2, day18::shortest_path::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ext {
    Start,
    End,
    ShortCut,
}
impl std::fmt::Display for Ext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Start => 'S',
            Self::End => 'E',
            Self::ShortCut => 'X',
        };
        write!(f, "{c}")
    }
}
type Loc = super::day18::Loc<Ext>;

struct Cheat {
    path: [Vec2; 3],
    saved_steps: i32,
}

pub fn day20(data: &str, p1: bool) -> i64 {
    let mut start = Vec2(0, 0);
    let mut end = Vec2(0, 0);

    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Loc::Obstacle,
                    '.' => Loc::Empty,
                    'S' => {
                        start = Vec2(x as i32, y as i32);
                        Loc::Empty
                    }
                    'E' => {
                        end = Vec2(x as i32, y as i32);
                        Loc::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let height = map.len();
    let width = map.first().unwrap().len();
    assert!(map.iter().all(|l| l.len() == width));
    let map = map.into_iter().flatten().collect_vec();

    let mut sp = ShortestPath::new(start, width, height);
    sp.calc(&map);
    let normal_steps = sp.steps_to(end).unwrap();

    println!("steps without cheats: {normal_steps}");

    let mut path_map = map.clone();
    sp.fill_path(&mut path_map, end);

    // Print path map with start and end markers.
    {
        let mut path_map = path_map.clone();
        path_map[start.to_idx(width)] = Loc::Custom(Ext::Start);
        path_map[end.to_idx(width)] = Loc::Custom(Ext::End);
        print_map(&path_map, width, height);
    }

    let mut cheats = HashMap::<i32, Vec<Cheat>>::new();
    let path = sp.get_path(end);

    let result = if p1 {
        for (p_start, steps_start) in path {
            let viable_cheats = p_start
                .neighbors()
                .flat_map(|(p_middle, _)| p_middle.neighbors().map(move |(p, _)| (p_middle, p)))
                .filter_map(|(p_middle, p_end)| {
                    let steps = sp.steps_to(p_end)?;
                    // Saved steps are: the amount of steps from start -> end,
                    // minus the 2 steps needed for the short-cut.
                    let saved_steps = (steps as i32) - (steps_start as i32) - 2;
                    Some((p_middle, p_end, saved_steps))
                })
                .map(|(p_middle, p_end, saved_steps)| Cheat {
                    path: [p_start, p_middle, p_end],
                    saved_steps,
                });

            for c in viable_cheats {
                cheats.entry(c.saved_steps).or_default().push(c);
            }
        }

        let overview = cheats
            .keys()
            .map(|k| (*k, cheats.get(k).unwrap().len()))
            .sorted_by_key(|(s, _)| *s)
            .collect_vec();

        let c = &cheats[&overview.last().unwrap().0][0];
        println!();
        println!(
            "best cheat: {} steps ({} saved)",
            normal_steps as i32 - c.saved_steps,
            c.saved_steps
        );
        show_cheat(&map, &c, width, height, start, end);

        let result: usize = overview
            .iter()
            .skip_while(|(s, _)| *s != 100)
            .map(|(_, count)| *count)
            .sum();
        result
    } else {
        unimplemented!("problem 2")
    };

    result as i64
}

fn show_cheat(map: &Vec<Loc>, c: &Cheat, width: usize, height: usize, start: Vec2, end: Vec2) {
    let mut path_map = map.clone();
    for p in &c.path {
        path_map[p.to_idx(width)] = Loc::Empty;
    }

    let mut sp = ShortestPath::new(start, width, height);
    sp.calc(&path_map);
    sp.fill_path(&mut path_map, end);
    for p in &c.path {
        path_map[p.to_idx(width)] = Loc::Custom(Ext::ShortCut);
    }
    path_map[start.to_idx(width)] = Loc::Custom(Ext::Start);
    path_map[end.to_idx(width)] = Loc::Custom(Ext::End);

    print_map(&path_map, width, height);
}
