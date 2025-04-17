// Last updated: 17.04.2025, 15:55:34
#[inline]
fn average(
    img: &[Vec<i32>],
    from_i: usize,
    to_i: usize,
    from_j: usize,
    to_j: usize,
    count: i32,
) -> i32 {
    let sum: i32 = img
        .iter()
        .take(to_i + 1)
        .skip(from_i)
        .map(|row| row.iter().take(to_j + 1).skip(from_j).sum::<i32>())
        .sum();
    (sum as f32 / count as f32).floor() as i32
}

#[inline]
fn row_smoother(
    result_row: &mut [i32],
    img: &[Vec<i32>],
    from_i: usize,
    to_i: usize,
    edge_count: i32,
    inner_count: i32,
) {
    let end_j = result_row.len() - 1;
    result_row[0] = average(img, from_i, to_i, 0, 1.min(end_j), edge_count);
    for (j, cell) in result_row.iter_mut().enumerate().take(end_j).skip(1) {
        *cell = average(img, from_i, to_i, j - 1, j + 1, inner_count);
    }
    if end_j > 0 {
        result_row[end_j] = average(img, from_i, to_i, end_j - 1, end_j, edge_count);
    }
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let (corner, edge, middle) = match (m, n) {
            (1, 1) => return img,
            (_, 1) | (1, _) => (2, 3, 3),
            _ => (4, 6, 9),
        };
        let end_i = m - 1;
        let mut result = vec![vec![0; n]; m];
        row_smoother(
            result.get_mut(0).unwrap(),
            &img,
            0,
            1.min(end_i),
            corner,
            edge,
        );
        for (i, row) in result.iter_mut().enumerate().take(end_i).skip(1) {
            row_smoother(row, &img, i - 1, i + 1, edge, middle);
        }
        if end_i > 0 {
            row_smoother(
                result.get_mut(end_i).unwrap(),
                &img,
                end_i - 1,
                end_i,
                corner,
                edge,
            );
        }
        result
    }
}
