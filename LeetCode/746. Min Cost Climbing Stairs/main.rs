impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];
        dp[1] = cost[0];
        dp[2] = cost[1];
        for i in 3..=cost.len() {
            dp[i] = std::cmp::min(dp[i - 1], dp[i - 2]) + cost[i - 1];
        }
        std::cmp::min(dp[cost.len()], dp[cost.len() - 1])
    }
}