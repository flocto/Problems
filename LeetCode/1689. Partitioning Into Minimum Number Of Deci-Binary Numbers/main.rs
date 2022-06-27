impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        // using fold again :)
        use std::cmp::max;
        n.chars().fold(0, |acc, c| {
            max(acc, c as i32 - '0' as i32)
        }) as i32
    }
}