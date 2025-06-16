// Last updated: 16.06.2025, 15:28:32
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut altitude = 0;
        let mut max_altitude = 0;
        for gain in gain {
            altitude += gain;
            max_altitude = max_altitude.max(altitude);
        }
        max_altitude
    }
}