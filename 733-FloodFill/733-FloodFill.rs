// Last updated: 24.04.2025, 14:59:36
#[inline]
fn process_in_image(image: &mut [Vec<i32>], queue: &mut Vec<(usize, usize)>, i: usize, j: usize, filled: i32, color: i32) {
    process_in_row(image.get_mut(i).unwrap(), queue, i, j, filled, color);
}

#[inline]
fn process_in_row(row: &mut [i32], queue: &mut Vec<(usize, usize)>, i: usize, j: usize, filled: i32, color: i32) {
    let cell = row.get_mut(j).unwrap();
    if *cell == filled {
        *cell = color;
        queue.push((i, j));
    }
}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let mut image = image;
        let cell = image.get_mut(sr).unwrap().get_mut(sc).unwrap();
        let filled = *cell;
        if filled == color {
            return image;
        } else {
            *cell = color;
        }
        let m = image.len();
        let n = image[0].len();
        let mut queue = Vec::with_capacity(2 * m + 2 * n);
        queue.push((sr, sc));
        while let Some((i, j)) = queue.pop() {
            if i > 0 {
                process_in_image(&mut image, &mut queue, i - 1, j, filled, color);
            }
            if i < m - 1 {
                process_in_image(&mut image, &mut queue, i + 1, j, filled, color);
            }
            let row = image.get_mut(i).unwrap();
            if j > 0 {
                process_in_row(row, &mut queue, i, j - 1, filled, color);
            }
            if j < n - 1 {
                process_in_row(row, &mut queue, i, j + 1, filled, color);
            }
        }
        image
    }
}