fn can_be_partitioned(square: i32, sum: i32) -> bool {
    if sum <= 0 || square == 0 {
        return false;
    }
    if square == sum {
        return true;
    }
    can_be_partitioned(square / 10, sum - square % 10)
        || can_be_partitioned(square / 100, sum - square % 100)
        || can_be_partitioned(square / 1000, sum - square % 1000)
        || can_be_partitioned(square / 10000, sum - square % 10000)
}

const CAN_BE_PARTITIONED: [i32; 29] = [
    1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756, 792,
    909, 918, 945, 964, 990, 991, 999, 1000,
];

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut result = 0;
        // could be easily calculated by `can_be_partitioned`,
        // but why if we can prepare this small set
        for item in CAN_BE_PARTITIONED {
            if item > n {
                break;
            }
            result += item * item;
        }
        result
    }
}