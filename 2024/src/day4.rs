use itertools::Itertools;

/// Count the amount of times `kernel` is in `data`, where a `0` in kernel matches everything.
fn correlate(data: &[&[u8]], data_max_width: usize, kernel: &[&[u8]], kernel_width: usize) -> u32 {
    assert!(kernel_width > 0 && kernel.len() > 0);

    let kernel_height = kernel.len();
    let data_height = data.len();
    let x_range = data_max_width - kernel_width + 1;
    let y_range = data_height - kernel_height + 1;

    let mut count = 0;
    for y in 0..y_range {
        'outer: for x in 0..x_range {
            for y_offset in 0..kernel_height {
                for x_offset in 0..kernel_width {
                    let k = kernel[y_offset][x_offset];
                    if k == 0 {
                        continue;
                    }

                    let y_data = data[y + y_offset];
                    let x_pos = x + x_offset;
                    if x_pos >= y_data.len() {
                        if k != 0 {
                            // Kernel at current position does not match, try the next position in data.
                            continue 'outer;
                        }
                        continue;
                    }

                    // Kernel at current position does not match, try the next position in data.
                    if y_data[x_pos] != k {
                        continue 'outer;
                    }
                }
            }

            // When we get here, the kernel matched.
            count += 1;
        }
    }

    count
}

pub fn day4(data: &str, p1: bool) -> i64 {
    // P1 kernels
    let diag_kernel: &[&[u8]] = &[
        &[b'X', 0, 0, 0],
        &[0, b'M', 0, 0],
        &[0, 0, b'A', 0],
        &[0, 0, 0, b'S'],
    ];
    let diag_kernel_m: &[&[u8]] = &[
        &[0, 0, 0, b'X'],
        &[0, 0, b'M', 0],
        &[0, b'A', 0, 0],
        &[b'S', 0, 0, 0],
    ];
    let diag_kernel_r: &[&[u8]] = &[
        &[b'S', 0, 0, 0],
        &[0, b'A', 0, 0],
        &[0, 0, b'M', 0],
        &[0, 0, 0, b'X'],
    ];
    let diag_kernel_mr: &[&[u8]] = &[
        &[0, 0, 0, b'S'],
        &[0, 0, b'A', 0],
        &[0, b'M', 0, 0],
        &[b'X', 0, 0, 0],
    ];
    let vert_kernel: &[&[u8]] = &[&[b'X'], &[b'M'], &[b'A'], &[b'S']];
    let vert_kernel_r: &[&[u8]] = &[&[b'S'], &[b'A'], &[b'M'], &[b'X']];
    let horiz_kernel: &[&[u8]] = &[&[b'X', b'M', b'A', b'S']];
    let horiz_kernel_r: &[&[u8]] = &[&[b'S', b'A', b'M', b'X']];

    // P2 kernels
    let xmas: &[&[u8]] = &[
        &[b'M', 0, b'M'],
        &[0,  b'A', 0],
        &[b'S', 0, b'S'],
    ];
    let xmas_nr: &[&[u8]] = &[
        &[b'M', 0, b'S'],
        &[0,  b'A', 0],
        &[b'M', 0, b'S'],
    ];
    let xmas_rn: &[&[u8]] = &[
        &[b'S', 0, b'M'],
        &[0,  b'A', 0],
        &[b'S', 0, b'M'],
    ];
    let xmas_rr: &[&[u8]] = &[
        &[b'S', 0, b'S'],
        &[0,  b'A', 0],
        &[b'M', 0, b'M'],
    ];


    let data = data.lines().map(|l| l.as_bytes()).collect_vec();
    let max_width = data.iter().map(|l| l.len()).max().unwrap_or(0);

    let result = if p1 {
        correlate(&data, max_width, diag_kernel, 4)
            + correlate(&data, max_width, diag_kernel_m, 4)
            + correlate(&data, max_width, diag_kernel_r, 4)
            + correlate(&data, max_width, diag_kernel_mr, 4)
            + correlate(&data, max_width, vert_kernel, 1)
            + correlate(&data, max_width, vert_kernel_r, 1)
            + correlate(&data, max_width, horiz_kernel, 4)
            + correlate(&data, max_width, horiz_kernel_r, 4)
    } else {
        correlate(&data, max_width, xmas, 3)
            + correlate(&data, max_width, xmas_nr, 3)
            + correlate(&data, max_width, xmas_rn, 3)
            + correlate(&data, max_width, xmas_rr, 3)
    };

    result as i64
}

#[test]
fn test_correlate() {
    let data: &[&[u8]] = &[
        &[1, 9, 3, 4, 40, 30, 20],
        &[9, 2, 9, 4, 40, 30],
        &[1, 9, 3, 4, 40, 30, 20],
        &[1, 9, 3, 4, 1, 9, 3],
        &[1, 9, 3, 4, 9, 2],
        &[1, 9, 3, 4, 1, 9, 3],
    ];

    let kernel1: &[&[u8]] = &[&[40, 30, 20]];
    assert_eq!(correlate(data, 7, kernel1, 3), 2u32);

    let kernel2: &[&[u8]] = &[&[1, 0, 0], &[0, 2, 0], &[0, 0, 3]];
    assert_eq!(correlate(data, 7, kernel2, 3), 2u32);

    let kernel3: &[&[u8]] = &[&[0, 0, 3], &[0, 2, 0], &[1, 0, 0]];
    assert_eq!(correlate(data, 7, kernel3, 3), 2u32);

    let kernel4: &[&[u8]] = &[&[1], &[1]];
    assert_eq!(correlate(data, 7, kernel4, 1), 3u32);
}
