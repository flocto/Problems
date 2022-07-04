impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy[i] = candy[i - 1] + 1;
            }
        }
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candy[i] = std::cmp::max(candy[i], candy[i + 1] + 1);
            }
        }
        candy.iter().sum()
    }
}
