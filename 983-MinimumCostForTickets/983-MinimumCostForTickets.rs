impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = *days.last().unwrap() as usize;
        let mut cost = vec![0; last_day + 1];

        let mut index_in_days = 0;
        for day in 1..=last_day {
            if day < days[index_in_days] as usize {
                cost[day] = cost[day - 1];
            } else {
                index_in_days += 1;
                cost[day] = (cost[day - 1] + costs[0])
                    .min(if day >= 7 { cost[day - 7] } else { 0 } + costs[1])
                    .min(if day >= 30 { cost[day - 30] } else { 0 } + costs[2]);
            }
        }
        cost[last_day]
    }
}