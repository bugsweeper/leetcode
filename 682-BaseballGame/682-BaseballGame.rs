// Last updated: 21.04.2025, 11:03:20
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut records = Vec::with_capacity(operations.len());
        for operation in operations {
            match operation.as_str() {
                "C" => _ = records.pop().unwrap(),
                "D" => {
                    let last = *records.last().unwrap();
                    records.push(last * 2);
                }
                "+" => {
                    let mut new_record = 0;
                    for &record in records.iter().rev().take(2) {
                        new_record += record;
                    }
                    records.push(new_record);
                }
                integer => records.push(integer.parse().unwrap()),
            }
        }
        records.into_iter().sum()
    }
}