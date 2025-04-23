// Last updated: 23.04.2025, 11:34:16
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut groups = vec![0; 37];
        let mut max_group_size = 0;
        for mut i in 1..=n {
            let mut digit_sum = 0;
            while i > 0 {
                digit_sum += i % 10;
                i /= 10;
            }
            let group = groups.get_mut(digit_sum as usize).unwrap();
            *group += 1;
            max_group_size = max_group_size.max(*group);
        }
        groups.into_iter().filter(|&size| size == max_group_size).count() as i32
    }
}