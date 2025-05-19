// Last updated: 19.05.2025, 15:49:02
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut indexes = vec![usize::MAX; 1001];
        for (index, &num1) in arr2.iter().enumerate() {
            indexes[num1 as usize] = index;
        }
        let mut arr1 = arr1;
        arr1.sort_by_key(|num| {
            let num = *num as usize;
            let index = indexes[num];
            if index == usize::MAX {
                num + 1001
            } else {
                index
            }
        });
        arr1
    }
}