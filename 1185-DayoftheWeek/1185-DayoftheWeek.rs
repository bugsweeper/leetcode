// Last updated: 21.05.2025, 15:02:18
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts = vec![0; 2001];
        for num in arr {
            counts[(num + 1000) as usize] += 1;
        }
        counts.sort_unstable();
        let non_zero_start = counts.partition_point(|&count| count == 0);
        for slice in counts[non_zero_start..].windows(2) {
            if slice[0] == slice[1] {
                return false;
            }
        }
        true
    }
}