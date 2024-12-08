use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Antenna {
    freq: char,
    x: i32,
    y: i32,
}

/// Whether a position (`x`, `y`) is within the rectangle from (0, 0) to
/// and without (`width`, `height`).
fn is_in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && x < (width as i32) && y < (height as i32)
}

pub fn print_nodes(lines: &[&str], nodes: &[(i32, i32)]) {
    for mut l in lines.iter().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(move |(x, c)| {
            if c == '.' && nodes.contains(&(x as i32, y as i32)) {
                '*'
            } else {
                c
            }
        })
    }) {
        println!("{}", l.join(""));
    }
}

pub fn day8(data: &str, p1: bool) -> i64 {
    let lines = data.lines().filter(|l| !l.is_empty()).collect_vec();

    let width = lines.get(0).expect("map").len();
    assert!(lines.iter().all(|l| l.len() == width));
    let height = lines.len();

    let antennas = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| Antenna {
                    freq: c,
                    x: x as i32,
                    y: y as i32,
                })
        })
        .flatten()
        .collect_vec();

    /// Find all antinodes of the two antenna with the same frequency `a` and `other_a`.
    ///
    /// An antinode occurs at any point that is perfectly in line with the two antennas, but
    /// only when one of the antennas is twice as far away as the other.
    ///
    /// Arguments:
    /// - `out`: The collection where the antinode position is inserted if any.
    /// - `a`, `other_a`: Two different antennas with the same frequency.
    /// - `width`, `height`: The size of the map to consider.
    fn model_p1(
        out: &mut HashSet<(i32, i32)>,
        a: &Antenna,
        other_a: &Antenna,
        width: usize,
        height: usize,
    ) {
        let x_dist = 2 * (other_a.x - a.x);
        let y_dist = 2 * (other_a.y - a.y);

        let node1_x = a.x + x_dist;
        let node1_y = a.y + y_dist;

        if is_in_bounds(node1_x, node1_y, width, height) {
            out.insert((node1_x, node1_y));
        }

        let node2_x = other_a.x - x_dist;
        let node2_y = other_a.y - y_dist;

        if is_in_bounds(node2_x, node2_y, width, height) {
            out.insert((node2_x, node2_y));
        }
    }

    /// Find all antinodes of the two antenna with the same frequency `a` and `other_a`.
    ///
    /// An antinode occurs at any grid position exactly in line with at least two antennas of the
    /// same frequency, regardless of distance.
    ///
    /// Arguments:
    /// - `out`: The collection where the antinode position is inserted if any.
    /// - `a`, `other_a`: Two different antennas with the same frequency.
    /// - `width`, `height`: The size of the map to consider.
    fn model_p2(
        out: &mut HashSet<(i32, i32)>,
        a: &Antenna,
        other_a: &Antenna,
        width: usize,
        height: usize,
    ) {
        let x_dist = other_a.x - a.x;
        let y_dist = other_a.y - a.y;

        let mut x = other_a.x;
        let mut y = other_a.y;
        while is_in_bounds(x, y, width, height) {
            out.insert((x, y));
            x += x_dist;
            y += y_dist;
        }

        let mut x = a.x;
        let mut y = a.y;
        while is_in_bounds(x, y, width, height) {
            out.insert((x, y));
            x -= x_dist;
            y -= y_dist;
        }
    }

    let model = if p1 { model_p1 } else { model_p2 };

    let mut antinodes = HashSet::new();
    for (a, other_a) in antennas
        .iter()
        .map(|a| {
            antennas
                .iter()
                .filter(|other_a| other_a.freq == a.freq && other_a.x != a.x && other_a.y != a.y)
                .map(move |other_a| (a, other_a))
        })
        .flatten()
    {
        model(&mut antinodes, a, other_a, width, height);
    }

    let antinodes = antinodes.into_iter().collect_vec();
    print_nodes(&lines, &antinodes);

    antinodes.len() as i64
}
