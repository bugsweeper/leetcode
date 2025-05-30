// Last updated: 30.05.2025, 14:18:24
const SOLDIER: i32 = 1;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut mat = mat.into_iter().enumerate().map(|(index, row)| (row.partition_point(|&human| human == SOLDIER), index as i32, row)).collect::<Vec<_>>();
        mat.sort_by_key(|(soldiers, ..)| *soldiers);
        mat.into_iter().take(k as usize).map(|(_, index, _)| index).collect::<Vec<_>>()
    }
}