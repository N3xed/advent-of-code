use super::day18::shortest_path::{self, Dir, Vec2};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NumPadBtn {
    Num(u8),
    A,
}

pub mod dijkstra {
    use std::{
        collections::{hash_map::Entry, HashMap},
        marker::PhantomData,
    };

    use itertools::Itertools;
    use smallvec::{smallvec, SmallVec};

    pub trait Node<T = ()>: Sized {
        fn neighbors(&self, context: &'_ T) -> impl Iterator<Item = (Self, i32)>;
    }

    pub struct ShortestPath<N> {
        // Graph of Node and (distance, visited status, came_from nodes).
        graph: HashMap<N, (i32, bool, SmallVec<[N; 1]>)>,
        start: N,
        eval_all: bool,
    }

    #[allow(dead_code)]
    impl<N: Eq + Clone + std::hash::Hash> ShortestPath<N> {
        pub fn new(start: N) -> Self {
            Self {
                graph: HashMap::new(),
                start,
                eval_all: false,
            }
        }

        pub fn new_eval_all(start: N) -> Self {
            Self {
                graph: HashMap::new(),
                start,
                eval_all: true,
            }
        }

        pub fn calc<'a, C>(
            &mut self,
            context: &C,
            mut is_end: impl FnMut(&N, i32) -> bool,
        ) -> Box<[N]>
        where
            N: 'a,
            N: Node<C>,
            N: std::fmt::Debug,
        {
            let Self {
                graph,
                start,
                eval_all,
            } = self;
            let eval_all = *eval_all;

            let mut candidates = vec![(start.clone(), 0_i32)];
            graph.insert(start.clone(), (0, false, smallvec![start.clone()]));

            let mut end_nodes = HashMap::new();
            let mut last_end_dist = None;

            loop {
                let Some((idx, _)) = candidates
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, (_, dist))| dist)
                else {
                    break;
                };
                let (node, dist) = candidates.swap_remove(idx);
                // The node must be already in the graph, since it is a candidate.
                let (_, visited, _) = graph.get_mut(&node).unwrap();
                if *visited {
                    continue;
                }
                *visited = true;

                // If we're considering the end node, we're done.
                if is_end(&node, dist) {
                    if !eval_all {
                        if let Some(last_end_dist) = last_end_dist {
                            if last_end_dist < dist {
                                break;
                            }
                        }
                        last_end_dist = Some(dist);
                    }
                    end_nodes.insert(node.clone(), dist);
                }

                for (neighbor, cost) in node.neighbors(context) {
                    let new_dist = dist + cost;
                    let (mut entry, existed) = match graph.entry(neighbor) {
                        Entry::Vacant(ve) => (
                            ve.insert_entry((new_dist, false, smallvec![node.clone()])),
                            false,
                        ),
                        Entry::Occupied(oe) => (oe, true),
                    };
                    let (n_dist, visited, came_from) = entry.get_mut();
                    if *visited {
                        continue;
                    }

                    if existed {
                        if *n_dist > new_dist {
                            *n_dist = new_dist;
                            *came_from = smallvec![node.clone()];
                        } else if *n_dist == new_dist && !came_from.contains(&node) {
                            came_from.push(node.clone());
                        }
                    }

                    candidates.push((entry.key().clone(), new_dist));
                }
            }

            end_nodes
                .into_iter()
                .sorted_by_key(|(_, d)| *d)
                .map(|(n, _)| n)
                .collect()
        }

        pub fn dist_at(&self, node: &N) -> Option<i32> {
            Some(self.graph.get(node)?.0)
        }

        pub fn paths<'a, C>(&'a self, end: &'a [N]) -> impl Iterator<Item = (Vec<&'a N>, i32)>
        where
            N: Node<C> + std::hash::Hash + Eq,
        {
            struct Iter<'b, N, C> {
                this: &'b ShortestPath<N>,
                end: &'b N,
                path_states: HashMap<&'b N, usize>,
                _ctx: PhantomData<C>,
            }

            impl<'b, N2, C2> Iterator for Iter<'b, N2, C2>
            where
                N2: Node<C2> + std::hash::Hash + Eq,
            {
                type Item = (Vec<&'b N2>, i32);

                fn next(&mut self) -> Option<Self::Item> {
                    let Self {
                        this,
                        end,
                        path_states,
                        ..
                    } = self;

                    let (k, v) = this.graph.get_key_value(&end)?;
                    let dist = v.0;
                    let mut path = vec![k];

                    while let Some(n) = path.last() {
                        if **n == this.start {
                            break;
                        }

                        if let Some((_, _, came_from)) = this.graph.get(n) {
                            if came_from.len() > 1 {
                                let idx = path_states.entry(n).or_default();
                                let next_node = came_from.get(*idx)?;
                                *idx += 1;
                                path.push(next_node);
                            } else {
                                path.push(came_from.first().unwrap());
                            }
                        } else {
                            break;
                        }
                    }

                    path.reverse();
                    Some((path, dist))
                }
            }

            end.iter().flat_map(|end| Iter {
                this: self,
                end,
                path_states: HashMap::new(),
                _ctx: PhantomData,
            })
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
            fn neighbors<'b>(&'b self, _: &()) -> impl Iterator<Item = (SeqNode<'a>, i32)>
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
        let n = sp.calc(&(), |n: &SeqNode, _| n.4.len() == 0);

        let (p, _dist) = sp.paths(&n).next().unwrap();

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
