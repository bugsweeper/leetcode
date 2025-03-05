impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut result = Vec::new();
        for hour in 0usize..12 {
            let hour_bits = hour.count_ones();
            if hour_bits > turned_on {
                continue;
            }
            for minute in 0usize..60 {
                if hour_bits + minute.count_ones() == turned_on {
                    result.push(format!("{hour}:{minute:02}"));
                }
            }
        }
        result
    }
}