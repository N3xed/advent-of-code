use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Beam(usize);

impl Beam {
    fn split(&self) -> Beam {
        Beam(self.0)
    }
}

impl std::ops::Add<&Beam> for Beam {
    type Output = Beam;

    fn add(self, rhs: &Beam) -> Self::Output {
        Beam(self.0 + rhs.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Space {
    Empty,
    Start,
    Splitter,
    Beam(Beam),
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '^' => Self::Splitter,
            '.' => Self::Empty,
            c => panic!("invalid char {c}"),
        }
    }
}

#[allow(dead_code)]
impl Space {
    fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Start => 'S',
            Self::Splitter => '^',
            Self::Beam(_) => '|',
        }
    }

    fn beam(&self) -> Option<&Beam> {
        match self {
            Self::Beam(b) => Some(b),
            _ => None,
        }
    }

    fn beam_mut(&mut self) -> Option<&mut Beam> {
        match self {
            Self::Beam(b) => Some(b),
            _ => None,
        }
    }
}

pub fn run(data: &str, p1: bool) -> impl std::fmt::Display {
    let mut width = 0;
    let mut layers = data
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            assert!(
                width == 0 || width == l.len(),
                "all lines must be the same length"
            );
            width = l.len();
            l.chars().map(Space::from).collect_vec()
        })
        .collect_vec();

    let mut curr_beams = vec![None::<Beam>; width];
    let mut last_layer = vec![Space::Empty; width];

    let mut missed_splitters = 0;
    let mut hit_splitters = 0;

    for chunks in layers.chunks_exact_mut(2) {
        let [splitters_layer, forward_layer] = chunks else {
            panic!("{} != 2", chunks.len())
        };

        for (i, space) in splitters_layer.iter_mut().enumerate() {
            let (left_beam, curr_beam, right_beam) = {
                let (lhs, rhs) = curr_beams.split_at_mut(i);
                let (center, rhs) = rhs.split_first_mut().unwrap();

                (lhs.last_mut(), center, rhs.first_mut())
            };

            match (space, curr_beam.clone()) {
                // `S` starts a new beam.
                (Space::Start, prev_beam) => {
                    assert!(prev_beam.is_none());
                    let next_beam = Beam(1);
                    *curr_beam = Some(next_beam.clone());
                    forward_layer[i] = Space::Beam(next_beam);
                }
                // `.` forwards a previous beam.
                (space @ Space::Empty, _) => {
                    if let Space::Beam(b) = &last_layer[i] {
                        forward_layer[i] = Space::Beam(b.clone());
                        *space = Space::Beam(b.clone());
                    }
                }
                // `^` splits the beam.
                (Space::Splitter, Some(prev_beam)) => {
                    hit_splitters += 1;
                    let split_beam = prev_beam.split();
                    *curr_beam = None;

                    fn combine_and_set(
                        to_set: Option<&mut Option<Beam>>,
                        split_beam: &Beam,
                    ) -> Beam {
                        match to_set {
                            // If there is already a beam at the split location,
                            // combine with the new split beam.
                            Some(Some(b)) => {
                                *b = *b + split_beam;
                                b.clone()
                            }
                            // If not, set it to the new beam.
                            Some(b @ None) => {
                                *b = Some(split_beam.clone());
                                split_beam.clone()
                            }
                            // We are outside the bounds, this should never happen.
                            _ => unreachable!("outside of bounds"),
                        }
                    }
                    let left_beam = combine_and_set(left_beam, &split_beam);
                    let right_beam = combine_and_set(right_beam, &split_beam);

                    // Set the beams in the next layer, this must always work
                    // otherwise the input is malformed.
                    forward_layer[i - 1] = Space::Beam(left_beam);
                    forward_layer[i + 1] = Space::Beam(right_beam);
                }
                (Space::Splitter, None) => {
                    missed_splitters += 1;
                }
                _ => (),
            }
        }
        last_layer.copy_from_slice(forward_layer);
    }

    for layer in layers.iter() {
        println!("{}", layer.iter().map(Space::to_char).join(""));
    }

    if p1 {
        println!("hit_splitters = {hit_splitters}, missed_splitters = {missed_splitters}");
        return hit_splitters;
    }

    let timelines = curr_beams
        .into_iter()
        .map(|b| b.unwrap_or(Beam(0)).0)
        .collect_vec();
    println!("last layer timeline counts: {timelines:?}");

    // Sum all timeline counts of the last layer.
    timelines.into_iter().sum::<usize>()
}
