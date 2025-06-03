// Last updated: 03.06.2025, 12:36:15
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut count = [0; 1001];
        for num in arr {
            count[num as usize] += 1;
        }
        for num in target {
            let count = count.get_mut(num as usize).unwrap();
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        true
    }
}