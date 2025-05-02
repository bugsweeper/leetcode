// Last updated: 02.05.2025, 16:29:22
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;
        for row in &mut image {
            let end = row.len() - 1;
            for i in 0..=end / 2 {
                row.swap(i, end - i);
            }
            for digit in row {
                *digit = 1 - *digit
            }
        }
        image
    }
}