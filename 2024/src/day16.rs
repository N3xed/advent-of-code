use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    day12::{Dir, Vec2},
    day18::{self, shortest_path::print_map},
    day21::dijkstra,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartEnd {
    Start,
    End,
}
impl std::fmt::Display for StartEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => 'S',
                Self::End => 'E',
            }
        )
    }
}
type Loc = day18::Loc<StartEnd>;

pub fn day16(data: &str, p1: bool) -> i64 {
    let mut start = Vec2(0, 0);
    let mut end = Vec2(0, 0);

    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .inspect(|(x, c)| {
                    if *c == 'S' {
                        start = Vec2(*x as i32, y as i32);
                    } else if *c == 'E' {
                        end = Vec2(*x as i32, y as i32);
                    }
                })
                .map(|(_, c)| c)
                .collect_vec()
        })
        .collect_vec();

    let height = map.len();
    let width = map.first().unwrap().len();
    assert!(map.iter().all(|l| l.len() == width));

    let map = map
        .into_iter()
        .flatten()
        .map(|c| match c {
            '#' => Loc::Obstacle,
            '.' | 'S' | 'E' => Loc::Empty,
            _ => unreachable!("invalid char {c}"),
        })
        .collect_vec();

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MapNode(Vec2, Dir);

    #[derive(Debug)]
    struct Context<'m> {
        width: usize,
        height: usize,
        map: &'m [Loc],
    }

    impl<'a> dijkstra::Node<Context<'a>> for MapNode {
        fn neighbors(&self, ctx: &Context<'a>) -> impl Iterator<Item = (Self, i32)> {
            let n0 = {
                let next_pos = self.0.offset_vec(self.1.into_vec2());
                if next_pos.is_in_bounds(ctx.width, ctx.height)
                    && ctx.map[next_pos.to_idx(ctx.width)] == Loc::Empty
                {
                    Some((MapNode(next_pos, self.1), 1))
                } else {
                    None
                }
            };
            n0.into_iter().chain([
                (MapNode(self.0, self.1.turn_n(1)), 1000),
                (MapNode(self.0, self.1.turn_n(-1)), 1000),
            ])
        }
    }

    let mut sp = dijkstra::ShortestPath::new(MapNode(start, Dir::Right));

    let end_nodes = sp.calc(
        &Context {
            width,
            height,
            map: &map,
        },
        |n: &MapNode, _| n.0 == end,
    );

    if p1 {
        let (path, dist) = sp.paths(&end_nodes).next().unwrap();

        let mut path_map = map.clone();
        for n in path {
            path_map[n.0.to_idx(width)] = Loc::Path;
        }
        path_map[start.to_idx(width)] = Loc::Custom(StartEnd::Start);
        path_map[end.to_idx(width)] = Loc::Custom(StartEnd::End);

        print_map(&path_map, width, height);

        return dist as i64;
    }

    let mut last_dist = None;
    let paths = sp
        .paths(&end_nodes)
        .sorted_by_key(|(_, dist)| *dist)
        .take_while(|(_, dist)| {
            if let Some(last_dist) = last_dist {
                if last_dist < *dist {
                    return false;
                }
            }
            last_dist = Some(*dist);
            true
        })
        .collect_vec();

    println!("{} paths", paths.len());

    let mut path_map = map.clone();
    let mut cells = HashSet::new();
    for (path, dist) in paths {
        println!("dist: {dist}");
        for n in path {
            cells.insert(n.0);
            path_map[n.0.to_idx(width)] = Loc::Path;
        }
    }
    path_map[start.to_idx(width)] = Loc::Custom(StartEnd::Start);
    path_map[end.to_idx(width)] = Loc::Custom(StartEnd::End);

    print_map(&path_map, width, height);

    cells.len() as i64
}
