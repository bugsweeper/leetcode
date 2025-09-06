// Last updated: 06.09.2025, 15:58:48
const SHIFTS_COUNT: [(i32, i64); 16] = [
    (1, 0),
    (4, 3),
    (16, 27),
    (64, 171),
    (256, 939),
    (1024, 4779),
    (4096, 23211),
    (16384, 109227),
    (65536, 502443),
    (262144, 2271915),
    (1048576, 10136235),
    (4194304, 44739243),
    (16777216, 195734187),
    (67108864, 850045611),
    (268435456, 3668617899),
    (1073741824, 15748213419),
];

fn shifts(last_num: i32) -> i64 {
    if last_num == 0 {
        return 0;
    }
    let index = SHIFTS_COUNT.partition_point(|(power_of_two, _)| last_num >= *power_of_two);
    let (lower_power_of_two, shift_count_before_it) = SHIFTS_COUNT[index - 1];
    shift_count_before_it + (last_num - lower_power_of_two + 1) as i64 * index as i64
}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        queries
            .into_iter()
            .map(|query| (shifts(query[1]) - shifts(query[0] - 1) + 1) >> 1)
            .sum()
    }
}