impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        if k == 0 || k == (n * (n - 1)) / 2{
            return 1;
        }
        if n == 1 || k > (n * (n - 1)) / 2 {
            return 0;
        }
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![vec![0 as i64; (k + 1) as usize]; (n + 1) as usize];
        for i in 1..(n + 1) as usize {
            for j in 0..(k + 1) as usize {
                if j == 0 { // k == 0 is always 1 (sorted array)
                    dp[i][j] = 1;
                }
                else if j >= i {
                    dp[i][j] = ((dp[i][j - 1] + dp[i - 1][j]) % MOD - dp[i - 1][j - i]) % MOD;
                }
                else { // dp[i-1][j-i] does not exist
                    dp[i][j] = (dp[i][j - 1] + dp[i - 1][j]) % MOD;
                }
 
            }
        }
        let res = (dp[n as usize][k as usize] - dp[n as usize][(k - 1) as usize]);
        if res < 0 {
            return (res + MOD) as i32;
        }
        return res as i32;
    }
}