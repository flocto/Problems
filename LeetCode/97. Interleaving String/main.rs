impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        let n = s1.len();
        let m = s2.len();
        let k = s3.len();
        if n + m != k {
            return false;
        }

        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true; // empty string
        for i in 0..n {
            if s1 [i] == s3 [i] {
                dp[i + 1][0] = true;
            } else {
                break;
            }
        }
        for j in 0..m {
            if s2 [j] == s3 [j] {
                dp[0][j + 1] = true;
            } else {
                break;
            }
        }
        for i in 1..n+1 {
            for j in 1..m+1 {
                let s1match = s1[i-1] == s3[i+j-1]; // s1 matches
                let s2match = s2[j-1] == s3[i+j-1]; // s2 matches
                dp[i][j] = dp[i-1][j] && s1match || dp[i][j-1] && s2match; // either s1 or s2 match and their previous dp is also true
            }
        }
        println!("{:?}", dp);
        dp[n][m]
    }
}