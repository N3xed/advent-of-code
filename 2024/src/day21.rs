use super::day18::shortest_path::{self, Dir, Vec2};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NumPadBtn {
    Num(u8),
    A,
}

pub mod dijkstra {
    use std::collections::{hash_map::Entry, HashMap};

    pub trait Node: Sized {
        fn neighbors(&self) -> impl Iterator<Item = (Self, i32)>;
    }

    pub struct ShortestPath<N: Eq + Clone + std::hash::Hash> {
        // Graph of Node and (distance, visited status).
        graph: HashMap<N, (i32, bool, N)>,
        start: N,
    }

    impl<N: Eq + Clone + std::hash::Hash> ShortestPath<N> {
        pub fn new(start: N) -> Self {
            Self {
                graph: HashMap::new(),
                start,
            }
        }

        pub fn calc<'a>(&mut self, mut is_end: Option<impl FnMut(&N) -> bool>) -> Option<N>
        where
            N: 'a,
            N: Node,
            N: std::fmt::Debug,
        {
            let Self { graph, start } = self;

            let mut candidates = vec![(start.clone(), 0_i32)];
            graph.insert(start.clone(), (0, false, start.clone()));

            loop {
                let Some((idx, _)) = candidates
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, (_, dist))| dist)
                else {
                    break None;
                };
                let (node, dist) = candidates.swap_remove(idx);
                // The node must be already in the graph, since it is a candidate.
                let (_, visited, _) = graph.get_mut(&node).unwrap();
                if *visited {
                    continue;
                }
                *visited = true;

                // If we're considering the end node, we're done.
                if is_end.as_mut().map_or(false, |v| v(&node)) {
                    break Some(node);
                }

                for (neighbor, cost) in node.neighbors() {
                    let (mut entry, existed) = match graph.entry(neighbor) {
                        Entry::Vacant(ve) => {
                            (ve.insert_entry((i32::MAX, false, node.clone())), false)
                        }
                        Entry::Occupied(oe) => (oe, true),
                    };
                    let (n_dist, visited, came_from) = entry.get_mut();
                    if *visited {
                        continue;
                    }

                    let new_dist = dist + cost;
                    if existed && *n_dist > new_dist {
                        *n_dist = new_dist;
                        *came_from = node.clone();
                    }

                    candidates.push((entry.key().clone(), new_dist));
                }
            }
        }

        pub fn path<'a>(&'a self, end: &'a N) -> Option<(Vec<&'a N>, i32)>
        where
            N: Node,
        {
            let (k, v) = self.graph.get_key_value(&end)?;
            let dist = v.0;
            let mut path = vec![k];

            while let Some(n) = path.last() {
                if **n == self.start {
                    break;
                }

                if let Some((_, _, came_from)) = self.graph.get(n) {
                    path.push(came_from);
                } else {
                    break;
                }
            }

            path.reverse();
            Some((path, dist))
        }
    }
}

impl NumPadBtn {
    fn to_idx(self) -> usize {
        match self {
            Self::A => 0,
            Self::Num(n) if n <= 9 => (n + 1) as usize,
            n => unreachable!("{n:?} is invalid"),
        }
    }

    const ALL: [NumPadBtn; 11] = [
        Self::A,
        Self::Num(0),
        Self::Num(1),
        Self::Num(2),
        Self::Num(3),
        Self::Num(4),
        Self::Num(5),
        Self::Num(6),
        Self::Num(7),
        Self::Num(8),
        Self::Num(9),
    ];

    const BTN_TO_POS_LUT: [Vec2; 11] = [
        Vec2(2, 3), // A
        Vec2(1, 3), // 0
        Vec2(0, 2), // 1
        Vec2(1, 2), // 2
        Vec2(2, 2), // 3
        Vec2(0, 1), // 4
        Vec2(1, 1), // 5
        Vec2(2, 1), // 6
        Vec2(0, 0), // 7
        Vec2(1, 0), // 8
        Vec2(2, 0), // 9
    ];

    fn to_pos(self) -> Vec2 {
        Self::BTN_TO_POS_LUT[self.to_idx()]
    }

    fn apply(self, dir: Dir) -> Option<Self> {
        let pos = self.to_pos().offset_vec(dir.into_vec2());
        let (i, _) = Self::BTN_TO_POS_LUT
            .iter()
            .enumerate()
            .find(|(_, v)| **v == pos)?;
        Some(Self::ALL[i])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirPadBtn {
    Dir(Dir),
    A,
}

impl std::fmt::Display for DirPadBtn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::A => 'A',
            Self::Dir(Dir::Up) => '^',
            Self::Dir(Dir::Right) => '>',
            Self::Dir(Dir::Down) => 'v',
            Self::Dir(Dir::Left) => '<',
        };
        write!(f, "{c}")
    }
}

impl DirPadBtn {
    fn dir(self) -> Dir {
        match self {
            Self::Dir(d) => d,
            _ => panic!("not a dir"),
        }
    }

    fn to_idx(self) -> usize {
        use shortest_path::Dir::*;
        use DirPadBtn::*;
        match self {
            Dir(Up) => 0,
            Dir(Right) => 1,
            Dir(Down) => 2,
            Dir(Left) => 3,
            A => 4,
        }
    }

    const ALL: [DirPadBtn; 5] = [
        Self::Dir(Dir::Up),
        Self::Dir(Dir::Right),
        Self::Dir(Dir::Down),
        Self::Dir(Dir::Left),
        Self::A,
    ];
    const BTN_TO_POS_LUT: [Vec2; 5] = [
        Vec2(1, 0), // Up
        Vec2(2, 1), // Right
        Vec2(1, 1), // Down
        Vec2(0, 1), // Left
        Vec2(2, 0), // A
    ];

    fn to_pos(self) -> Vec2 {
        Self::BTN_TO_POS_LUT[self.to_idx()]
    }

    fn apply(self, dir: Dir) -> Option<Self> {
        let pos = self.to_pos().offset_vec(dir.into_vec2());
        let (i, _) = Self::BTN_TO_POS_LUT
            .iter()
            .enumerate()
            .find(|(_, v)| **v == pos)?;
        Some(Self::ALL[i])
    }
}

pub fn day21(data: &str, _p1: bool) -> i64 {
    let codes = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let n = l
                .trim_matches(|c: char| !c.is_numeric())
                .parse::<u32>()
                .ok();
            let code = l
                .chars()
                .map(|c| {
                    if let Some(d) = c.to_digit(10) {
                        NumPadBtn::Num(d as u8)
                    } else if c == 'A' {
                        NumPadBtn::A
                    } else {
                        unreachable!("{c} is invalid");
                    }
                })
                .collect_vec();
            (n, code)
        })
        .collect_vec();

    fn shortest_seq(code: &Vec<NumPadBtn>) -> String {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct SeqNode<'a>(NumPadBtn, DirPadBtn, DirPadBtn, DirPadBtn, &'a [NumPadBtn]);

        impl<'a> SeqNode<'a> {
            fn step(&self, n: DirPadBtn) -> Option<SeqNode<'a>> {
                let SeqNode(mut num, mut dir1, mut dir2, _, mut s) = self.clone();

                if n == DirPadBtn::A {
                    if dir2 == DirPadBtn::A {
                        if dir1 == DirPadBtn::A {
                            if num == *s.get(0)? {
                                s = &s[1..];
                            } else {
                                return None;
                            }
                        } else {
                            num = num.apply(dir1.dir())?;
                        }
                    } else {
                        dir1 = dir1.apply(dir2.dir())?;
                    }
                } else {
                    dir2 = dir2.apply(n.dir())?;
                }

                Some(SeqNode(num, dir1, dir2, n, s))
            }
        }

        impl<'a> dijkstra::Node for SeqNode<'a> {
            fn neighbors<'b>(&'b self) -> impl Iterator<Item = (SeqNode<'a>, i32)>
            where
                Self: 'a,
            {
                let v = DirPadBtn::ALL
                    .into_iter()
                    .filter_map(move |d| Some((self.step(d)?, 1)))
                    .collect_vec();
                v.into_iter()
            }
        }

        use dijkstra::*;
        let mut sp = ShortestPath::new(SeqNode(
            NumPadBtn::A,
            DirPadBtn::A,
            DirPadBtn::A,
            DirPadBtn::A,
            code as &[NumPadBtn],
        ));
        let n = sp.calc(Some(|n: &SeqNode| n.4.len() == 0));

        let n = n.unwrap();
        let (p, _dist) = sp.path(&n).unwrap();

        let seq = p.iter().map(|n| n.3).skip(1).join("");

        seq
    }

    let mut result = 0;
    for (n, code) in codes {
        let code = shortest_seq(&code);
        let compl = code.len() as u32 * n.unwrap_or(0);
        println!("len={}, {}: {code}", code.len(), compl);
        result += compl;
    }

    result as i64
}
