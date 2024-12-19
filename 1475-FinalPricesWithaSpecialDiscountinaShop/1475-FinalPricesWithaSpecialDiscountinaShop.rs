impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut prices = prices;
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..prices.len() {
            if let Some(mut max_stack_index) = stack.pop() {
                let value = prices[i];
                while prices[max_stack_index] >= value {
                    prices[max_stack_index] -= value;
                    if let Some(next_max_stack_index) = stack.pop() {
                        max_stack_index = next_max_stack_index;
                    } else {
                        max_stack_index = usize::MAX;
                        break;
                    }
                }
                if max_stack_index != usize::MAX {
                    stack.push(max_stack_index);
                }
            }
            stack.push(i);
        }
        prices
    }
}