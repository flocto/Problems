use std::io;

fn main(){
    let word1 = read_line();
    let word2 = read_line();
    let solve = Solution::min_distance(word1, word2);
    println!("{}", solve);
}

use std::cmp::min;
impl Solution {
    //the minimum number of deletions to make both strings the same
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        for i in 0..word1.len() + 1 {
            dp[i][0] = i as i32;
        }
        for j in 0..word2.len() + 1 {
            dp[0][j] = j as i32;
        }
        for i in 1..word1.len() + 1 {
            for j in 1..word2.len() + 1 {
                //if the chars are the same, no need to delete
                dp[i][j] = match word1[i-1] == word2[j-1] {
                    true => dp[i-1][j-1],
                    false => min(dp[i-1][j], dp[i][j-1]) + 1
                }
            }
        }
        dp[word1.len()][word2.len()]        
    }
}