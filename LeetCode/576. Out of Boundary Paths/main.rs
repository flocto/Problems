const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![0; 51]; 51]; 51];
        let mut ans = 0;
        dp[0][start_row as usize][start_column as usize] = 1;
        for i in 0..max_move {
            for j in 0..m {
                for k in 0..n {
                    for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                        let x = j as i32 + dx;
                        let y = k as i32 + dy;
                        if x < 0 || x >= m || y < 0 || y >= n {
                            ans += dp[i as usize][j as usize][k as usize];
                            ans %= MOD;
                        }
                        else {
                            dp[(i + 1) as usize][x as usize][y as usize] += dp[i as usize][j as usize][k as usize];
                            dp[(i + 1) as usize][x as usize][y as usize] %= MOD;
                        }
                    }
                }
            }
        }
        ans
    }
}