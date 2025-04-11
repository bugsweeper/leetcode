// Last updated: 11.04.2025, 16:14:14
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed
            .iter()
            .chain(std::iter::once(&0))
            .chain(std::iter::once(&1))
            .fold((1, n), |(zero_length, n), &num| {
                if num == 0 {
                    (zero_length + 1, n)
                } else {
                    (0, n - (zero_length - 1) / 2)
                }
            }).1 <= 0
    }
}