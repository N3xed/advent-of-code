use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn offset(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: (self.x as i32 + x) as u32,
            y: (self.y as i32 + y) as u32,
        }
    }

    fn neighbors(&self, width: usize, height: usize) -> impl Iterator<Item = Self> {
        let mut idx = 0;
        let mut result = [Pos::default(); 4];

        let mut pos = self.offset(0, 1);
        if (pos.y as usize) < height {
            result[idx] = pos;
            idx += 1;
        }
        pos = self.offset(1, 0);
        if (pos.x as usize) < width {
            result[idx] = pos;
            idx += 1;
        }
        if self.y > 0 {
            result[idx] = self.offset(0, -1);
            idx += 1;
        }
        if self.x > 0 {
            result[idx] = self.offset(-1, 0);
            idx += 1;
        }
        result.into_iter().take(idx)
    }

    fn to_idx(&self, width: usize) -> usize {
        self.x as usize + (self.y as usize) * width
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: Pos,
    elev: u8,
}

#[derive(Default, Debug)]
struct Tree {
    paths: Vec<Vec<Node>>,
}

impl Tree {
    fn new(head: Pos, elev: u8) -> Self {
        Tree {
            paths: vec![vec![Node { pos: head, elev }]],
        }
    }

    const TOP_ELEV: u8 = 9;

    fn find_trails(&mut self, map: &[u8], width: usize, height: usize) {
        let Self { paths } = self;

        let mut new_paths = Vec::<Vec<Node>>::new();
        let mut to_remove = Vec::new();
        let mut finished = Vec::new();
        loop {
            for (i, path) in paths.iter_mut().enumerate() {
                let node = path.last().unwrap().clone();

                let mut is_new_path = false;
                for pos in node.pos.neighbors(width, height) {
                    let elev_should_be = node.elev + 1;
                    let elev_is = map[pos.to_idx(width)];
                    if elev_is != elev_should_be {
                        continue;
                    }

                    let node = Node { pos, elev: elev_is };
                    if is_new_path {
                        let mut new_path = path.clone();
                        *new_path.last_mut().unwrap() = node;
                        new_paths.push(new_path);
                    } else {
                        path.push(node);
                        is_new_path = true;
                    }
                }

                if !is_new_path {
                    to_remove.push(i);
                }
            }

            for i in to_remove.drain(..).rev() {
                paths.swap_remove(i);
            }
            paths.extend(new_paths.drain(..));
            paths.retain_mut(|p| {
                let last = p.last().unwrap();
                if last.elev == Self::TOP_ELEV {
                    finished.push((last.clone(), std::mem::take(p)));
                    false
                } else {
                    true
                }
            });

            if paths.is_empty() {
                break;
            }
        }

        finished.sort_unstable_by_key(|(l, _)| l.pos);
        *paths = finished.iter().map(|(_, p)| p.clone()).collect();
    }
}

#[test]
fn test_p1() {
    let s = "\
        0123\n\
        1234\n\
        8765\n\
        9876\n\
    ";

    assert_eq!(6, day10(s, true));
}

pub fn day10(data: &str, p1: bool) -> i64 {
    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("digit") as u8)
                .collect_vec()
        })
        .collect_vec();

    let width = map.first().expect("at least one line").len();
    assert!(map.iter().all(|l| l.len() == width));
    let height = map.len();
    let map = map.into_iter().flatten().collect_vec();

    let pos_iter = (0..height as u32).flat_map(|y| (0..width as u32).map(move |x| Pos { x, y }));
    let result: usize = pos_iter
        .map(|pos| {
            if map[pos.to_idx(width)] != 0 {
                return 0;
            }

            let mut tree = Tree::new(pos, 0);
            tree.find_trails(&map, width, height);

            if p1 {
                tree.paths.dedup_by_key(|p| p.last().unwrap().pos);
                tree.paths.len()
            } else {
                tree.paths.len()
            }
        })
        .sum();

    result as i64
}
