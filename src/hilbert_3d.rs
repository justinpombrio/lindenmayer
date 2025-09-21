// This function is transliterated from John Burkardt's Python implementation of the same, which is
// MIT licensed.
// https://people.sc.fsu.edu/~jburkardt//py_src/hilbert_curve_3d/hilbert_curve_3d.html
/// Find the (x, y, z) position of the `index`th point along the 3D Hilbert Curve of order `depth`.
pub fn hilbert_3d_coords(depth: usize, index: usize) -> (usize, usize, usize) {
    let mut i = index;

    let (mut x, mut y, mut z): (usize, usize, usize) = match i % 8 {
        0 => (0, 0, 0),
        1 => (1, 0, 0),
        2 => (1, 0, 1),
        3 => (0, 0, 1),
        4 => (0, 1, 1),
        5 => (1, 1, 1),
        6 => (1, 1, 0),
        7 => (0, 1, 0),
        _ => unreachable!("i % 8 must be in range [0, 8)"),
    };
    i /= 8;

    let mut w = 2;
    while i > 0 {
        (x, y, z) = match i % 8 {
            0 => (y, z, x),
            1 => (z + w, x, y),
            2 => (z + w, x, y + w),
            3 => (w - x - 1, y, 2 * w - z - 1),
            4 => (w - x - 1, y + w, 2 * w - z - 1),
            5 => (z + w, 2 * w - x - 1, 2 * w - y - 1),
            6 => (z + w, 2 * w - x - 1, w - y - 1),
            7 => (w - y - 1, 2 * w - z - 1, x),
            _ => unreachable!("i % 8 must be in range [0, 8)"),
        };
        i /= 8;
        w *= 2; // This was wrong in the source
    }

    let max = x.max(y).max(z);
    let rmin = if max == 0 { 0 } else { max.ilog2() as usize };
    (x, y, z) = match (depth - rmin) % 3 {
        0 => (x, y, z),
        1 => (y, z, x),
        2 => (z, x, y),
        _ => unreachable!("x % 3 must be in range [0, 3)"),
    };

    (x, y, z)
}

#[test]
fn test_hilbert_3d_coords() {
    // Construct the 8x8x8 Hilbert cube
    let seq = (0..512)
        .map(|i| hilbert_3d_coords(3, i))
        .collect::<Vec<_>>();

    // Verify the endpoints
    assert_eq!(seq[0], (0, 0, 0));
    assert_eq!(seq[seq.len() - 1], (7, 0, 0));

    // Verify that all points are inside the cube boundaries
    for (x, y, z) in &seq {
        assert!(*x < 8);
        assert!(*y < 8);
        assert!(*z < 8);
    }

    // Verify that all points are distinct
    for i in 0..512 {
        for j in i + 1..512 {
            assert!(seq[i] != seq[j]);
        }
    }

    // Verify that each point differs by 1 from the previous point
    for i in 0..511 {
        let (x0, y0, z0) = seq[i];
        let (x1, y1, z1) = seq[i + 1];
        let diff = (x0.abs_diff(x1), y0.abs_diff(y1), z0.abs_diff(z1));
        assert!(diff == (0, 0, 1) || diff == (0, 1, 0) || diff == (1, 0, 0));
    }
}
