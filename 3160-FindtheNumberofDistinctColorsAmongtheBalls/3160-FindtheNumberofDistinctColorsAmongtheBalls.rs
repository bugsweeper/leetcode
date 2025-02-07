impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let limit = limit as usize;
        use std::collections::HashMap;
        let mut balls = HashMap::with_capacity(queries.len());
        let mut result = Vec::with_capacity(queries.len());
        let mut color2ball_count = HashMap::with_capacity(queries.len());
        for (index, query) in queries.into_iter().enumerate() {
            let ball = *unsafe{query.get_unchecked(0)} as usize;
            let color = *unsafe{query.get_unchecked(1)} as usize;
            let ball_color = balls.entry(ball).or_insert(0);
            if *ball_color != 0 {
                let prev_color_count = color2ball_count.get_mut(ball_color).unwrap();
                if *prev_color_count == 1usize {
                    color2ball_count.remove(ball_color);
                } else {
                    *prev_color_count -= 1;
                }
            }
            *color2ball_count.entry(color).or_insert(0) += 1;
            *ball_color = color;
            result.insert(index, color2ball_count.len() as i32);
        }
        result
    }
}