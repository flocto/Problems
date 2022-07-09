impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let mut deque = VecDeque::<usize>::new();
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        deque.push_back(0);

        for i in 1..nums.len() {
            if i > k as usize && *deque.front().unwrap() < i - (k as usize) { // can't reach
                deque.pop_front();
            }
            dp[i] = nums[i] + dp[*deque.front().unwrap()];
            while !deque.is_empty() && dp[*deque.back().unwrap()] < dp[i] - nums[i] {
                deque.pop_back();
            }
            deque.push_back(i);
        }

        dp[nums.len() - 1]
    }
}

