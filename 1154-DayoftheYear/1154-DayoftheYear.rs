// Last updated: 21.05.2025, 13:19:08
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (mut start, mut destination) = (start as usize, destination as usize);
        if (start > destination) {
            (start, destination) = (destination, start);
        }
        let total_sum = distance.iter().sum::<i32>();
        let sub_sum = distance[start..destination].iter().sum::<i32>();
        sub_sum.min(total_sum - sub_sum)
    }
}