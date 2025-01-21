fn make_prefix(row: &Vec<i32>) -> (Vec<i64>, i64) {
    let mut prefix = Vec::with_capacity(row.len());
    let mut sum = 0;
    for item in row {
        sum += *item as i64;
        prefix.push(sum);
    }
    (prefix, sum)
}

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let (prefix0, sum0) = make_prefix(unsafe { grid.get_unchecked(0) });
        let (prefix1, _) = make_prefix(unsafe { grid.get_unchecked(1) });
        let (mut iter0, iter1) = (prefix0.into_iter(), prefix1.into_iter());
        let item0 = iter0.next().unwrap();
        let mut max_second = sum0 - item0;
        for (item0, item1) in iter0.zip(iter1) {
            max_second = max_second.min(item1.max(sum0 - item0));
        }
        max_second
    }
}