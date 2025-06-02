// Last updated: 02.06.2025, 12:30:05
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();
        let mut distance = 0;
        for num1 in arr1 {
            let insert_index = arr2.partition_point(|&num2| num2 < num1);
            if insert_index < arr2.len() && arr2[insert_index] - num1 <= d {
                continue;
            }
            if insert_index > 0 && num1 - arr2[insert_index - 1] <= d {
                continue;
            }
            distance += 1;
        }
        distance
    }
}