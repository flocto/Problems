use std::io;

fn main() {
    let words = ["a","b","ba","bca","bda","bdca"];
    let ans = Solution::longest_str_chain(words);
}

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut dp = std::collections::HashMap::new();
        let mut ans = 0;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        for word in words {
            let mut best = 0;
            for i in 0..word.len() {
                let mut w = word.clone();
                w.remove(i);
                if dp.contains_key(&w) {
                    best = std::cmp::max(best, dp[&w] + 1);
                }
                else {
                    best = std::cmp::max(best, 1);
                }
            }
            dp.insert(word, best);
            ans = std::cmp::max(ans, best);
        }
        ans
    }
}