// Last updated: 22.05.2025, 14:45:26
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut max = -1;
        for num in arr.iter_mut().rev() {
            (*num, max) = (max, max.max(*num))
        }
        arr
    }
}