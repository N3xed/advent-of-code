use std::usize;

use itertools::Itertools;

use crate::day12::{Dir, Vec2};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

impl Tile {
    fn as_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Box => 'O',
            Tile::Wall => '#',
        }
    }
}

impl Dir {
    fn to_offset(self) -> Vec2 {
        match self {
            Dir::Up => Vec2(0, -1),
            Dir::Right => Vec2(1, 0),
            Dir::Down => Vec2(0, 1),
            Dir::Left => Vec2(-1, 0),
        }
    }
}

struct Map {
    map: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Map {
    fn move_stack(&mut self, pos: Vec2, offset: Vec2) -> bool {
        let width = self.width as usize;
        let height = self.height as usize;

        let pos = pos.offset_vec(offset);
        if !pos.is_in_bounds(width, height) {
            return false;
        }

        let initial_tile_idx = pos.to_idx(width);
        let initial_tile = self.map[initial_tile_idx];
        match initial_tile {
            Tile::Wall => return false,
            Tile::Empty => return true,
            Tile::Box => (),
        };

        let mut curr = pos;
        let empty_tile = loop {
            curr = curr.offset_vec(offset);
            if !curr.is_in_bounds(width, height) {
                return false;
            }
            let tile = &mut self.map[curr.to_idx(width)];
            match tile {
                Tile::Wall => return false,
                Tile::Box => (),
                Tile::Empty => break tile,
            }
        };
        *empty_tile = Tile::Box;
        self.map[initial_tile_idx] = Tile::Empty;

        true
    }

    fn print(&self) {
        for y in 0..self.height {
            let l = (0..self.width)
                .map(move |x| self.map[(x + y * self.width) as usize].as_char())
                .join("");
            println!("{l}");
        }
    }
}

pub fn day15(data: &str, _p1: bool) -> i64 {
    let lines = data.lines().map(|l| l.trim()).collect_vec();
    let (map, instructions) = lines
        .split(|l| l.is_empty())
        .collect_tuple()
        .expect("map then instructions");

    let mut pos = Vec2(0, 0);
    let map = map
        .into_iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {
                        pos = Vec2(x as i32, y as i32);
                        Tile::Empty
                    }
                    _ => unreachable!("unexpected char '{c}' at ({x}, {y})"),
                })
                .collect_vec()
        })
        .collect_vec();

    let width = map.first().unwrap().len();
    assert!(map.iter().all(|l| l.len() == width));
    let height = map.len();
    let map = map.into_iter().flatten().collect_vec();

    let instructions = instructions
        .into_iter()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '<' => Dir::Left,
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            _ => unreachable!("unexptected char in instructions: '{c}'"),
        })
        .collect_vec();

    let mut map = Map {
        map,
        width: width as u32,
        height: height as u32,
    };
    for inst in instructions {
        let offset = inst.to_offset();
        if map.move_stack(pos, offset) {
            pos = pos.offset_vec(offset);
        }
    }

    map.print();

    let result: u64 = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            if map.map[(x + y * width) as usize] == Tile::Box {
                Some((x + y * 100) as u64)
            } else {
                None
            }
        })
        .sum();

    result as i64
}
