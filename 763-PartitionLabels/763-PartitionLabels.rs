// Last updated: 30.03.2025, 13:47:51
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn get_statistic(s: &str) -> [i32; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &c in s.as_bytes() {
        *statistic.get_mut((c - b'a') as usize).unwrap() += 1;
    }
    statistic
}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let all_statistic = get_statistic(&s);
        let mut statistic = [0; ABC_LEN];
        let mut part_sizes = Vec::with_capacity(ABC_LEN);
        let mut prev_index = -1;
        for (index, &c) in s.as_bytes().iter().enumerate() {
            let statistic_index = (c - b'a') as usize;
            let count = statistic.get_mut(statistic_index).unwrap();
            *count += 1;
            if *count == all_statistic[statistic_index] {
                if statistic.iter().zip(&all_statistic).all(|(&current, &global)| current == 0 || current == global) {
                    let index = index as i32;
                    part_sizes.push(index - prev_index);
                    prev_index = index;
                }
            }
        }
        part_sizes
    }
}