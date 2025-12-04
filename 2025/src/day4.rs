use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,             // Space is empty.
    Full,              // Space is filled by a paper roll.
    Accessible(usize), // Space is filled by a paper roll and accessible.
}

impl Space {
    fn get_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Full => '@',
            Self::Accessible(_) => 'x',
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<Space>,
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .chunks_exact(self.width)
            .map(|l| l.iter().map(|s| s.get_char()).join(""))
            .join("\n");
        writeln!(f, "{s}")
    }
}

impl Map {
    fn format_with_iter(&self, iter: usize) -> String {
        let s = self
            .data
            .chunks_exact(self.width)
            .map(move |l| {
                l.iter()
                    .map(|s| {
                        match *s {
                            Space::Accessible(i) if i != iter => Space::Empty,
                            s => s,
                        }
                        .get_char()
                    })
                    .join("")
            })
            .join("\n");
        format!("{s}")
    }

    /// Pad the map with equal amount of `pad` padding on all sides, and fill the
    /// padded elements with `elem`.
    fn pad(&self, pad: usize, elem: Space) -> Map {
        let width = 2 * pad + self.width;
        let height = self.height + 2 * pad;
        let mut data = vec![elem; width * height];
        for (y, line) in self.data.chunks_exact(self.width).enumerate() {
            let start_i = (y + pad) * width + pad;
            let end_i = start_i + self.width;
            data[start_i..end_i].copy_from_slice(line);
        }
        Map {
            width,
            height,
            data,
        }
    }

    /// Get an iterator over all mutable correlation kernel windows into map, where the kernel width
    /// and height is `KERNEL_SIZE`.
    ///
    /// The iterator element is a tuple of (x, y, kernel window) where x and y are the coordinates
    /// of the top-left corner of the current kernel window.
    fn kernel_windows_mut<const KERNEL_SIZE: usize>(
        &mut self,
    ) -> impl Iterator<Item = (usize, usize, [&mut [Space]; KERNEL_SIZE])> {
        let corr_height = self.height - KERNEL_SIZE;
        let corr_width = self.width - KERNEL_SIZE;

        let len = self.data.len();
        let data_ptr: *mut Space = self.data.as_mut_ptr();

        (0..corr_height)
            .flat_map(move |y| (0..corr_width).map(move |x| (x, y)))
            .map(move |(x, y)| {
                let arr: [&mut [Space]; KERNEL_SIZE] = (y..(y + KERNEL_SIZE))
                    .map(|ky| {
                        let start_i = ky * self.width + x;
                        let end_i = start_i + KERNEL_SIZE;
                        debug_assert!(start_i < len && end_i < len);

                        // Safety: range is always non-overlapping and in bounds.
                        // TODO: is it possible have access to two kernel windows at the same
                        // time? If so this is not safe, since they can alias. I.e. two
                        // kernel windows can overlap, but nothing overlaps within a kernel window.
                        unsafe {
                            std::slice::from_raw_parts_mut(
                                data_ptr.offset(start_i as isize),
                                KERNEL_SIZE,
                            )
                        }
                    })
                    .collect_array()
                    .expect("logic error");
                (x, y, arr)
            })
    }
}

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let mut map_width = 0;
    let map = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let l = l.trim();
            assert!(map_width == 0 || map_width == l.len());
            map_width = l.len();

            l.as_bytes().iter().map(|&c| match c {
                b'.' => Space::Empty,
                b'@' => Space::Full,
                s => panic!("invalid symbol {}", char::from(s)),
            })
        })
        .flatten()
        .collect_vec();
    let map = Map {
        width: map_width,
        height: map.len() / map_width,
        data: map,
    };

    // 3x3 kernel so pad by 2 on each side.
    let mut padded_map = map.pad(2, Space::Empty);

    if p1 {
        let result = padded_map
            .kernel_windows_mut::<3>()
            .map(|(_x, _y, submap)| {
                // If center of submap is empty, skip.
                if submap[1][1] == Space::Empty {
                    return false;
                }

                // Count the rolls.
                let mut num_rolls = submap
                    .iter()
                    .flat_map(|l| l.iter())
                    .filter(|&&s| s != Space::Empty)
                    .count();
                // Minus one since the center doesn't count, and we know that center is not empty
                // since we checked it above.
                num_rolls -= 1;

                if num_rolls < 4 {
                    // Also set the space to accessible for pretty printing.
                    submap[1][1] = Space::Accessible(0);
                    true
                } else {
                    false
                }
            })
            .filter(|v| *v)
            .count();

        println!("{padded_map}");
        return result;
    }

    let mut total_removed = 0_usize;
    for i in 0.. {
        let result = padded_map
            .kernel_windows_mut::<3>()
            .map(move |(_x, _y, submap)| {
                let center = submap[1][1];
                // If center of submap is empty, skip.
                match center {
                    Space::Empty | Space::Accessible(_) => return false,
                    _ => (),
                }

                // Count the rolls.
                let mut num_rolls = submap
                    .iter()
                    .flat_map(|l| l.iter())
                    .filter(|&&s| match s {
                        Space::Full => true,
                        _ => false,
                    })
                    .count();
                // Minus one since the center doesn't count, and we know that center is not empty
                // since we checked it above.
                num_rolls -= 1;

                if num_rolls < 4 {
                    // Also set the space to accessible for pretty printing, and to mark it as
                    // removed.
                    submap[1][1] = Space::Accessible(i);
                    true
                } else {
                    false
                }
            })
            .filter(|v| *v)
            .count();

        println!(
            "\n\niteration {i} (removed = {result}, total = {total_removed}):\n\n{}",
            padded_map.format_with_iter(i)
        );

        if result == 0 {
            break;
        }
        total_removed += result;
    }
    return total_removed;
}
