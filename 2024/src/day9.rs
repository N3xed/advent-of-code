use std::hash::{DefaultHasher, Hash, Hasher};

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Block {
    File(u32),
    Free,
}

impl Block {
    fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MultiBlock {
    File { n_blocks: u32, id: u32 },
    Free { n_blocks: u32 },
}

impl MultiBlock {
    fn n_blocks_mut(&mut self) -> &mut u32 {
        match self {
            Self::File { n_blocks, .. } | Self::Free { n_blocks } => n_blocks,
        }
    }

    fn blocks(&self) -> impl Iterator<Item = Block> {
        let (n_blocks, block) = match *self {
            Self::File { n_blocks, id } => (n_blocks, Block::File(id)),
            Self::Free { n_blocks } => (n_blocks, Block::Free),
        };

        std::iter::repeat_n(block, n_blocks as usize)
    }
}

struct File {
    id: u32,
    file_blocks: u32,
    free_blocks: u32,
}

fn print_disk_layout(blocks: impl IntoIterator<Item = Block>) {
    let layout = blocks
        .into_iter()
        .map(|b| match b {
            Block::File(id) => {
                let mut hasher = DefaultHasher::new();
                id.hash(&mut hasher);
                char::from_digit((hasher.finish() % 16) as u32, 16).unwrap()
            }
            Block::Free => '.',
        })
        .join("");
    println!("{layout}");
}

const MAX_PRINT_SIZE: usize = 50;

fn compact_blocks(files: &[File]) -> Vec<Block> {
    let mut blocks = files
        .into_iter()
        .map(|f| {
            (0..f.file_blocks)
                .map(move |_| Block::File(f.id))
                .chain((0..f.free_blocks).map(|_| Block::Free))
        })
        .flatten()
        .collect_vec();

    let disk_size = blocks.len();
    if disk_size <= MAX_PRINT_SIZE {
        print_disk_layout(blocks.iter().copied());
    }

    // Compact disk by moving the right most `Block::File` to the left-most empty block
    // until all empty blocks are filled.
    'outer: for idx in 0.. {
        if idx >= blocks.len() {
            break;
        }

        if blocks[idx].is_file() {
            continue;
        }

        let file = loop {
            if (blocks.len() - 1) <= idx {
                break 'outer;
            }

            let block = blocks.pop().unwrap();
            if block.is_file() {
                break block;
            }
        };

        blocks[idx] = file;

        if disk_size <= MAX_PRINT_SIZE {
            print_disk_layout(
                blocks
                    .iter()
                    .copied()
                    .chain(std::iter::repeat_n(Block::Free, disk_size - blocks.len())),
            );
        }
    }

    blocks
}

fn compact_muti_blocks(files: &[File]) -> Vec<Block> {
    let mut blocks = files
        .into_iter()
        .map(|f| {
            [
                MultiBlock::File {
                    id: f.id,
                    n_blocks: f.file_blocks,
                },
                MultiBlock::Free {
                    n_blocks: f.free_blocks,
                },
            ]
        })
        .flatten()
        .collect_vec();

    let disk_size: usize = files
        .iter()
        .map(|f| f.free_blocks as usize + f.file_blocks as usize)
        .sum();
    if disk_size <= MAX_PRINT_SIZE {
        print_disk_layout(blocks.iter().flat_map(MultiBlock::blocks));
    }

    // Compact disk by moving the right most multi-block file into the left most space that fits.
    // If the file doesn't fill the whole free space, fill the rest with free blocks.
    // Do one pass for all files.
    let mut idx_a_iter = (0..blocks.len()).rev();
    loop {
        let Some(idx_a) = idx_a_iter.next() else {
            break;
        };
        let MultiBlock::File { n_blocks, .. } = blocks[idx_a] else {
            continue;
        };

        let Some((idx_b, n_free_blocks)) =
            blocks[0..idx_a]
                .iter()
                .enumerate()
                .find_map(|(i, b)| match *b {
                    MultiBlock::Free {
                        n_blocks: free_blocks,
                    } if free_blocks >= n_blocks => Some((i, free_blocks)),
                    _ => None,
                })
        else {
            continue;
        };

        blocks.swap(idx_a, idx_b);
        if n_blocks < n_free_blocks {
            *blocks[idx_a].n_blocks_mut() = n_blocks;
            blocks.insert(
                idx_b + 1,
                MultiBlock::Free {
                    n_blocks: n_free_blocks - n_blocks,
                },
            );
            idx_a_iter = (0..(idx_a + 1)).rev();
        }

        if disk_size <= MAX_PRINT_SIZE {
            print_disk_layout(blocks.iter().flat_map(MultiBlock::blocks));
        }
    }

    blocks.iter().flat_map(MultiBlock::blocks).collect_vec()
}

pub fn day9(data: &str, p1: bool) -> i64 {
    let mut chars = data.trim().chars().collect_vec();
    if chars.len() % 2 != 0 {
        chars.push('0');
    }

    let files = chars
        .into_iter()
        .tuples::<(_, _)>()
        .enumerate()
        .map(|(id, (file_blocks, free_blocks))| {
            let file_blocks: u32 = file_blocks.to_digit(10).expect("digit");
            let free_blocks: u32 = free_blocks.to_digit(10).expect("digit");

            File {
                id: id as u32,
                file_blocks,
                free_blocks,
            }
        })
        .collect_vec();

    let blocks = if p1 {
        compact_blocks(&files)
    } else {
        compact_muti_blocks(&files)
    };

    let result: u64 = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            let Block::File(id) = b else {
                return None;
            };

            Some(i as u64 * (*id as u64))
        })
        .sum();

    result as i64
}
