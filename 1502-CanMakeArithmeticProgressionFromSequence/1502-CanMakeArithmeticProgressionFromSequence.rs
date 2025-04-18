// Last updated: 18.04.2025, 13:54:20
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();
        let mut iter = arr.into_iter();
        let mut prev = iter.next().unwrap();
        let mut num = iter.next().unwrap();
        let diff = num - prev;
        prev = num;
        iter.all(|num| {
            if num - prev != diff {
                return false;
            }
            prev = num;
            true
        })
    }
}